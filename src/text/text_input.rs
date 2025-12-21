//! TextInput view component
//!
//! A single-line text input with horizontal scrolling, cursor/selection rendering,
//! and keyboard/mouse handling. Reuses Document for text editing logic.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use floem::{
    Renderer, View, ViewId,
    action::exec_after,
    context::{ComputeLayoutCx, PaintCx},
    event::{Event, EventListener, EventPropagation},
    kurbo::{Point, Rect, Size},
    peniko::Color,
    reactive::{RwSignal, SignalGet, SignalUpdate, SignalWith, create_effect, create_rw_signal},
    style::{CursorStyle as StyleCursorStyle, Style},
    text::{FamilyOwned, LineHeightValue, Weight},
    unit::PxPct,
};

use crate::theme::ShadcnThemeProp;
use floem_editor_core::buffer::rope_text::RopeText;
use floem_editor_core::command::{EditCommand, MoveCommand};
use ui_events::{
    keyboard::{Key, KeyState, KeyboardEvent, Modifiers, NamedKey},
    pointer::PointerEvent,
};

use super::Document;

/// Cursor blink interval in milliseconds
const CURSOR_BLINK_INTERVAL_MS: u64 = 500;

/// A command that can be executed on the text input
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputCommand {
    Edit(EditCommand),
    Move(MoveCommand),
    /// Select all text
    SelectAll,
    /// Copy selected text to clipboard
    Copy,
    /// Cut selected text to clipboard
    Cut,
    /// Paste text from clipboard
    Paste,
}

/// A key press with modifiers
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InputKeyPress {
    pub key: Key,
    pub modifiers: Modifiers,
}

/// Keymap for single-line text input
pub struct InputKeypressMap {
    pub keymaps: HashMap<InputKeyPress, InputCommand>,
}

impl Default for InputKeypressMap {
    fn default() -> Self {
        let mut keymaps = HashMap::new();

        // Platform-specific modifier for common shortcuts
        #[cfg(target_os = "macos")]
        let cmd_or_ctrl = Modifiers::META;
        #[cfg(not(target_os = "macos"))]
        let cmd_or_ctrl = Modifiers::CONTROL;

        // =======================================================================
        // Basic navigation (arrow keys) - no Up/Down for single-line
        // =======================================================================
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::ArrowLeft),
                modifiers: Modifiers::default(),
            },
            InputCommand::Move(MoveCommand::Left),
        );
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::ArrowRight),
                modifiers: Modifiers::default(),
            },
            InputCommand::Move(MoveCommand::Right),
        );

        // =======================================================================
        // Line start/end navigation (acts as document start/end for single line)
        // =======================================================================
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::Home),
                modifiers: Modifiers::default(),
            },
            InputCommand::Move(MoveCommand::DocumentStart),
        );
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::End),
                modifiers: Modifiers::default(),
            },
            InputCommand::Move(MoveCommand::DocumentEnd),
        );
        // Cmd+Left (macOS) -> Start
        #[cfg(target_os = "macos")]
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::ArrowLeft),
                modifiers: Modifiers::META,
            },
            InputCommand::Move(MoveCommand::DocumentStart),
        );
        // Cmd+Right (macOS) -> End
        #[cfg(target_os = "macos")]
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::ArrowRight),
                modifiers: Modifiers::META,
            },
            InputCommand::Move(MoveCommand::DocumentEnd),
        );

        // =======================================================================
        // Word navigation
        // =======================================================================
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::ArrowLeft),
                modifiers: Modifiers::ALT,
            },
            InputCommand::Move(MoveCommand::WordBackward),
        );
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::ArrowRight),
                modifiers: Modifiers::ALT,
            },
            InputCommand::Move(MoveCommand::WordForward),
        );
        #[cfg(not(target_os = "macos"))]
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::ArrowLeft),
                modifiers: Modifiers::CONTROL,
            },
            InputCommand::Move(MoveCommand::WordBackward),
        );
        #[cfg(not(target_os = "macos"))]
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::ArrowRight),
                modifiers: Modifiers::CONTROL,
            },
            InputCommand::Move(MoveCommand::WordForward),
        );

        // =======================================================================
        // Basic editing (no newline for single-line input)
        // =======================================================================
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::Backspace),
                modifiers: Modifiers::default(),
            },
            InputCommand::Edit(EditCommand::DeleteBackward),
        );
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::Delete),
                modifiers: Modifiers::default(),
            },
            InputCommand::Edit(EditCommand::DeleteForward),
        );

        // =======================================================================
        // Word deletion
        // =======================================================================
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::Backspace),
                modifiers: Modifiers::ALT,
            },
            InputCommand::Edit(EditCommand::DeleteWordBackward),
        );
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::Delete),
                modifiers: Modifiers::ALT,
            },
            InputCommand::Edit(EditCommand::DeleteWordForward),
        );
        #[cfg(not(target_os = "macos"))]
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::Backspace),
                modifiers: Modifiers::CONTROL,
            },
            InputCommand::Edit(EditCommand::DeleteWordBackward),
        );
        #[cfg(not(target_os = "macos"))]
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::Delete),
                modifiers: Modifiers::CONTROL,
            },
            InputCommand::Edit(EditCommand::DeleteWordForward),
        );

        // =======================================================================
        // Line deletion (delete to beginning)
        // =======================================================================
        #[cfg(target_os = "macos")]
        keymaps.insert(
            InputKeyPress {
                key: Key::Named(NamedKey::Backspace),
                modifiers: Modifiers::META,
            },
            InputCommand::Edit(EditCommand::DeleteToBeginningOfLine),
        );

        // =======================================================================
        // Select All
        // =======================================================================
        keymaps.insert(
            InputKeyPress {
                key: Key::Character("a".into()),
                modifiers: cmd_or_ctrl,
            },
            InputCommand::SelectAll,
        );

        // =======================================================================
        // Clipboard operations (Cmd/Ctrl+C, Cmd/Ctrl+X, Cmd/Ctrl+V)
        // =======================================================================
        keymaps.insert(
            InputKeyPress {
                key: Key::Character("c".into()),
                modifiers: cmd_or_ctrl,
            },
            InputCommand::Copy,
        );
        keymaps.insert(
            InputKeyPress {
                key: Key::Character("x".into()),
                modifiers: cmd_or_ctrl,
            },
            InputCommand::Cut,
        );
        keymaps.insert(
            InputKeyPress {
                key: Key::Character("v".into()),
                modifiers: cmd_or_ctrl,
            },
            InputCommand::Paste,
        );

        // =======================================================================
        // Unix Emacs-style keybindings (Ctrl+letter) - macOS and Linux
        // =======================================================================
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            // Ctrl+H -> Delete backward (backspace)
            keymaps.insert(
                InputKeyPress {
                    key: Key::Character("h".into()),
                    modifiers: Modifiers::CONTROL,
                },
                InputCommand::Edit(EditCommand::DeleteBackward),
            );
            // Ctrl+D -> Delete forward
            keymaps.insert(
                InputKeyPress {
                    key: Key::Character("d".into()),
                    modifiers: Modifiers::CONTROL,
                },
                InputCommand::Edit(EditCommand::DeleteForward),
            );
            // Ctrl+A -> Move to beginning of line (document start for single-line)
            keymaps.insert(
                InputKeyPress {
                    key: Key::Character("a".into()),
                    modifiers: Modifiers::CONTROL,
                },
                InputCommand::Move(MoveCommand::DocumentStart),
            );
            // Ctrl+E -> Move to end of line (document end for single-line)
            keymaps.insert(
                InputKeyPress {
                    key: Key::Character("e".into()),
                    modifiers: Modifiers::CONTROL,
                },
                InputCommand::Move(MoveCommand::DocumentEnd),
            );
            // Ctrl+F -> Move forward (right)
            keymaps.insert(
                InputKeyPress {
                    key: Key::Character("f".into()),
                    modifiers: Modifiers::CONTROL,
                },
                InputCommand::Move(MoveCommand::Right),
            );
            // Ctrl+B -> Move backward (left)
            keymaps.insert(
                InputKeyPress {
                    key: Key::Character("b".into()),
                    modifiers: Modifiers::CONTROL,
                },
                InputCommand::Move(MoveCommand::Left),
            );
            // Ctrl+K -> Kill to end of line
            keymaps.insert(
                InputKeyPress {
                    key: Key::Character("k".into()),
                    modifiers: Modifiers::CONTROL,
                },
                InputCommand::Edit(EditCommand::DeleteToEndOfLine),
            );
        }

        Self { keymaps }
    }
}

/// A single-line text input view
pub struct TextInput {
    id: ViewId,
    doc: RwSignal<Document>,
    padding: RwSignal<(f64, f64, f64, f64)>,
    /// Horizontal scroll offset for long text
    scroll_offset: RwSignal<f64>,
    /// Visible width of the input area
    visible_width: RwSignal<f64>,
    /// Tracks the last time the cursor was moved or text was edited.
    last_cursor_action: RwSignal<Instant>,
    /// Placeholder text shown when input is empty
    placeholder: RwSignal<Option<String>>,
    /// Callback for Enter key
    on_enter: RwSignal<Option<Box<dyn Fn(&str)>>>,
}

impl TextInput {
    /// Creates a new text input with empty content
    pub fn new() -> Self {
        Self::with_text("")
    }

    /// Creates a new text input with the given initial text
    pub fn with_text(text: impl Into<String>) -> Self {
        Self::with_text_and_id(text, ViewId::new())
    }

    /// Creates a new text input with the given initial text and a specific ViewId
    pub fn with_text_and_id(text: impl Into<String>, id: ViewId) -> Self {
        let padding = create_rw_signal((0.0, 0.0, 0.0, 0.0));
        let scroll_offset = create_rw_signal(0.0);
        let visible_width = create_rw_signal(0.0);
        let doc = Document::new(text.into());
        let doc_signal = create_rw_signal(doc);
        let last_cursor_action = create_rw_signal(Instant::now());
        let placeholder = create_rw_signal(None);
        let on_enter: RwSignal<Option<Box<dyn Fn(&str)>>> = create_rw_signal(None);

        // Capture cursor signal for reactive tracking
        let cursor_signal = doc_signal.get_untracked().cursor();

        // Effect to ensure cursor is visible (horizontal scrolling)
        create_effect(move |_| {
            let cursor = cursor_signal.get();
            let doc = doc_signal.get_untracked();
            let lines = doc.text_layouts().borrow();
            let point = lines.point_of_offset(cursor.end);
            let cursor_x = point.x;
            drop(lines);

            let current_offset = scroll_offset.get_untracked();
            let width = visible_width.get_untracked();

            if width <= 0.0 {
                return;
            }

            // Ensure cursor is visible within the viewport
            let new_offset = if cursor_x < current_offset {
                cursor_x
            } else if cursor_x > current_offset + width {
                cursor_x - width
            } else {
                current_offset
            };

            if new_offset != current_offset {
                scroll_offset.set(new_offset);
            }
        });

        // Set up event handlers
        let keypress_map = std::sync::Arc::new(InputKeypressMap::default());
        let keypress_map_clone = keypress_map.clone();

        id.add_event_listener(
            EventListener::PointerDown,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Down(pointer_event)) = event {
                    let padding = padding.get_untracked();
                    let offset = scroll_offset.get_untracked();
                    let mut adjusted = pointer_event.clone();
                    adjusted.state.position.x -= padding.3;
                    adjusted.state.position.x += offset;
                    adjusted.state.position.y -= padding.0;
                    id.request_active();
                    id.request_focus();
                    doc_signal.get_untracked().pointer_down(&adjusted);
                    last_cursor_action.set(Instant::now());
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::PointerMove,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Move(pointer_event)) = event {
                    let padding = padding.get_untracked();
                    let offset = scroll_offset.get_untracked();
                    let mut adjusted = pointer_event.clone();
                    adjusted.current.position.x -= padding.3;
                    adjusted.current.position.x += offset;
                    adjusted.current.position.y -= padding.0;
                    doc_signal.get_untracked().pointer_move(&adjusted);
                    last_cursor_action.set(Instant::now());
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::PointerUp,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Up(pointer_event)) = event {
                    let padding = padding.get_untracked();
                    let offset = scroll_offset.get_untracked();
                    let mut adjusted = pointer_event.clone();
                    adjusted.state.position.x -= padding.3;
                    adjusted.state.position.x += offset;
                    adjusted.state.position.y -= padding.0;
                    doc_signal.get_untracked().pointer_up(&adjusted);
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::KeyDown,
            Box::new(move |event| {
                let Event::Key(KeyboardEvent {
                    state: KeyState::Down,
                    key,
                    modifiers,
                    ..
                }) = event
                else {
                    return EventPropagation::Continue;
                };

                // Handle Enter key specially
                if key == &Key::Named(NamedKey::Enter) {
                    on_enter.with_untracked(|cb| {
                        if let Some(callback) = cb {
                            let text = doc_signal.get_untracked().text();
                            callback(&text);
                        }
                    });
                    return EventPropagation::Stop;
                }

                let keypress = InputKeyPress {
                    key: key.clone(),
                    modifiers: modifiers.clone(),
                };

                // Try to find command
                let command = keypress_map_clone.keymaps.get(&keypress).or_else(|| {
                    let mut modified = keypress.clone();
                    modified.modifiers.set(Modifiers::SHIFT, false);
                    keypress_map_clone.keymaps.get(&modified)
                });

                let document = doc_signal.get_untracked();

                if let Some(command) = command {
                    let shift_held = modifiers.shift();
                    match command {
                        InputCommand::Edit(edit_cmd) => {
                            document.run_edit_command(edit_cmd);
                            id.request_layout();
                        }
                        InputCommand::Move(move_cmd) => {
                            document.run_move_command(move_cmd, shift_held);
                            id.request_paint();
                        }
                        InputCommand::SelectAll => {
                            document.select_all();
                            id.request_paint();
                        }
                        InputCommand::Copy => {
                            document.copy();
                        }
                        InputCommand::Cut => {
                            if document.cut() {
                                id.request_layout();
                            }
                        }
                        InputCommand::Paste => {
                            // Filter newlines for single-line input
                            if document.paste(true) {
                                id.request_layout();
                            }
                        }
                    }
                    last_cursor_action.set(Instant::now());
                    return EventPropagation::Stop;
                }

                // Handle character input (filter out newlines)
                let mut mods = modifiers.clone();
                mods.set(Modifiers::SHIFT, false);
                #[cfg(target_os = "macos")]
                mods.set(Modifiers::ALT, false);

                if mods.is_empty() {
                    if let Key::Character(c) = key {
                        // Filter out newlines for single-line input
                        let filtered: String = c.chars().filter(|&ch| ch != '\n' && ch != '\r').collect();
                        if !filtered.is_empty() {
                            document.insert_text(&filtered);
                            id.request_layout();
                            last_cursor_action.set(Instant::now());
                        }
                    }
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::ImeCommit,
            Box::new(move |event| {
                if let Event::ImeCommit(text) = event {
                    // Filter out newlines from IME input
                    let filtered: String = text.chars().filter(|&ch| ch != '\n' && ch != '\r').collect();
                    if !filtered.is_empty() {
                        doc_signal.get_untracked().insert_text(&filtered);
                        id.request_layout();
                        last_cursor_action.set(Instant::now());
                    }
                }
                EventPropagation::Stop
            }),
        );

        Self {
            id,
            doc: doc_signal,
            padding,
            scroll_offset,
            visible_width,
            last_cursor_action,
            placeholder,
            on_enter,
        }
    }

    /// Returns the document signal
    pub fn doc(&self) -> RwSignal<Document> {
        self.doc
    }

    /// Registers a callback to be called when the document content changes
    pub fn on_update(self, on_update: impl Fn(&str) + 'static) -> Self {
        self.doc.get_untracked().on_update(on_update);
        self
    }

    /// Sets the placeholder text shown when the input is empty
    pub fn placeholder(self, text: impl Into<String>) -> Self {
        self.placeholder.set(Some(text.into()));
        self
    }

    /// Sets the callback for when Enter is pressed
    pub fn on_enter(self, callback: impl Fn(&str) + 'static) -> Self {
        self.on_enter.set(Some(Box::new(callback)));
        self
    }

    /// Sets the editor content reactively
    pub fn value(self, set_value: impl Fn() -> String + 'static) -> Self {
        let doc = self.doc;
        create_effect(move |_| {
            let new_value = set_value();
            // Filter out newlines
            let filtered: String = new_value.chars().filter(|&ch| ch != '\n' && ch != '\r').collect();
            // Check if document already has this value
            let current_text = doc.with_untracked(|d| d.text());
            if current_text == filtered {
                return;
            }
            doc.with_untracked(|doc| {
                let end = doc.buffer().with_untracked(|b| b.text().len());
                use floem_editor_core::{
                    cursor::CursorAffinity, editor::EditType, selection::SelRegion,
                };
                doc.edit(
                    [(
                        SelRegion::new(0, end, CursorAffinity::Forward, None),
                        filtered.as_str(),
                    )],
                    EditType::Other,
                );
            });
        });
        self
    }

    /// Returns the current text content
    pub fn text(&self) -> String {
        self.doc.get_untracked().text()
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl View for TextInput {
    fn id(&self) -> ViewId {
        self.id
    }

    fn view_style(&self) -> Option<Style> {
        Some(
            Style::new()
                .cursor(StyleCursorStyle::Text)
                .focusable(true),
        )
    }

    fn compute_layout(&mut self, _cx: &mut ComputeLayoutCx) -> Option<Rect> {
        let layout = self.id.get_layout().unwrap_or_default();
        let style = self.id.get_combined_style();
        let builtin_style = style.builtin();

        let padding_left = match builtin_style.padding_left() {
            PxPct::Px(padding) => padding,
            PxPct::Pct(pct) => (pct / 100.) * layout.size.width as f64,
        };
        let padding_right = match builtin_style.padding_right() {
            PxPct::Px(padding) => padding,
            PxPct::Pct(pct) => (pct / 100.) * layout.size.width as f64,
        };
        let padding_top = match builtin_style.padding_top() {
            PxPct::Px(padding) => padding,
            PxPct::Pct(pct) => (pct / 100.) * layout.size.width as f64,
        };
        let padding_bottom = match builtin_style.padding_bottom() {
            PxPct::Px(padding) => padding,
            PxPct::Pct(pct) => (pct / 100.) * layout.size.width as f64,
        };

        if (padding_top, padding_right, padding_bottom, padding_left)
            != self.padding.get_untracked()
        {
            self.padding
                .set((padding_top, padding_right, padding_bottom, padding_left));
        }

        let width = layout.size.width as f64 - padding_left - padding_right;

        if width != self.visible_width.get_untracked() {
            self.visible_width.set(width);
        }

        // Get text styling from style and set them on the document
        let text_color = builtin_style.color().unwrap_or(Color::BLACK);
        let font_size = builtin_style.font_size().unwrap_or(14.0);
        let line_height = builtin_style
            .line_height()
            .unwrap_or(LineHeightValue::Normal(1.5));
        let font_weight = builtin_style.font_weight().unwrap_or(Weight::NORMAL);
        let font_family = builtin_style
            .font_family()
            .map(|f| vec![FamilyOwned::Name(f)])
            .unwrap_or_default();

        let doc = self.doc.get_untracked();
        doc.set_text_color(text_color);
        doc.set_font_size(font_size);
        doc.set_line_height(line_height);
        doc.set_font_weight(font_weight);
        doc.set_font_family(font_family);
        // Use a very large width to prevent wrapping
        doc.set_width(f64::MAX);

        None
    }

    fn paint(&mut self, cx: &mut PaintCx) {
        let padding = self.padding.get_untracked();
        let scroll_offset = self.scroll_offset.get_untracked();
        let visible_width = self.visible_width.get_untracked();

        // Get text styling and theme from style
        let style = self.id.get_combined_style();
        let builtin = style.builtin();
        let text_color = builtin.color().unwrap_or(Color::BLACK);
        let font_size = builtin.font_size().unwrap_or(14.0);
        let line_height = builtin
            .line_height()
            .unwrap_or(LineHeightValue::Normal(1.5));
        let font_weight = builtin.font_weight().unwrap_or(Weight::NORMAL);
        let font_family: Vec<FamilyOwned> = builtin
            .font_family()
            .map(|f| vec![FamilyOwned::Name(f)])
            .unwrap_or_default();
        let theme = style.get(ShadcnThemeProp);
        let selection_color = theme.primary.multiply_alpha(0.2);
        let placeholder_color = text_color.multiply_alpha(0.5);

        let layout = self.id.get_layout().unwrap_or_default();
        let height = layout.size.height as f64 - padding.0 - padding.2;

        cx.save();
        cx.clip(
            &Rect::from_origin_size(
                Point::new(padding.3, padding.0),
                Size::new(visible_width, height),
            ),
        );

        let doc = self.doc.get_untracked();
        let text = doc.text();
        let lines = doc.text_layouts().borrow();

        // Calculate y_offset for vertical centering using default glyph metrics
        let glyph_top = lines.default_glyph_top();
        let glyph_height = lines.default_glyph_height();
        let y_offset = (height - glyph_height) / 2.0 - glyph_top;

        // Draw placeholder when empty and not focused, or empty with placeholder
        if text.is_empty() {
            if let Some(placeholder_text) = self.placeholder.get_untracked() {
                // Draw placeholder text with same styling as main text
                let mut placeholder_attrs = floem::text::Attrs::default()
                    .font_size(font_size)
                    .color(placeholder_color)
                    .line_height(line_height)
                    .weight(font_weight);

                if !font_family.is_empty() {
                    placeholder_attrs = placeholder_attrs.family(&font_family);
                }

                let placeholder_layout = floem::text::TextLayout::new_with_text(
                    &placeholder_text,
                    floem::text::AttrsList::new(placeholder_attrs),
                    None,
                );

                // Center vertically using placeholder's own height
                let ph_text_height = placeholder_layout.size().height as f64;
                let ph_y_offset = (height - ph_text_height) / 2.0;

                cx.draw_text(
                    &placeholder_layout,
                    Point::new(padding.3, padding.0 + ph_y_offset),
                );
            }
        } else {
            // Draw text content
            let layout_iter = lines.visual_lines(0..lines.utf8_len() + 1);
            cx.draw_text_with_layout(
                layout_iter,
                Point::new(padding.3 - scroll_offset, padding.0 + y_offset),
            );
        }

        // Always draw cursor/selection when focused (even when empty)
        if cx.is_focused(self.id) {
            let cursor = doc.cursor().get_untracked();
            if cursor.is_caret() {
                // Calculate cursor visibility based on blink cycle
                let elapsed_ms = self
                    .last_cursor_action
                    .get_untracked()
                    .elapsed()
                    .as_millis();
                let blink_cycle = elapsed_ms / CURSOR_BLINK_INTERVAL_MS as u128;
                let is_cursor_visible = blink_cycle % 2 == 0;

                if is_cursor_visible {
                    let p = lines.point_of_offset(cursor.end);
                    let (cursor_top, cursor_height) = if p.glyph_bottom > p.glyph_top {
                        (p.glyph_top, p.glyph_bottom - p.glyph_top)
                    } else {
                        (lines.default_glyph_top(), lines.default_glyph_height())
                    };
                    let rect = Rect::from_origin_size(
                        (
                            p.x + padding.3 - scroll_offset - 1.0,
                            cursor_top + padding.0 + y_offset,
                        ),
                        (2.0, cursor_height),
                    );
                    cx.fill(&rect, text_color, 0.0);
                }

                // Schedule repaint for cursor blink
                let id = self.id;
                exec_after(Duration::from_millis(CURSOR_BLINK_INTERVAL_MS), move |_| {
                    id.request_paint();
                });
            } else {
                // Draw selection
                let start_x = lines.point_of_offset(cursor.min()).x;
                let end_x = lines.point_of_offset(cursor.max()).x;
                let p = lines.point_of_offset(cursor.min());
                let (sel_top, sel_height) = if p.glyph_bottom > p.glyph_top {
                    (p.glyph_top, p.glyph_bottom - p.glyph_top)
                } else {
                    (lines.default_glyph_top(), lines.default_glyph_height())
                };

                let rect = Rect::from_origin_size(
                    (
                        start_x + padding.3 - scroll_offset,
                        sel_top + padding.0 + y_offset,
                    ),
                    (end_x - start_x, sel_height),
                );
                cx.fill(&rect, selection_color, 0.0);
            }
        }

        cx.restore();
    }
}
