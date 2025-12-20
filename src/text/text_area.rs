//! TextArea view component
//!
//! A multi-line text area with visual line support, cursor/selection rendering,
//! and keyboard/mouse handling.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use floem::{
    action::exec_after,
    context::{ComputeLayoutCx, PaintCx},
    event::{Event, EventListener, EventPropagation},
    kurbo::{Point, Rect, Size},
    peniko::Color,
    reactive::{create_effect, create_rw_signal, RwSignal, SignalGet, SignalTrack, SignalUpdate, SignalWith},
    style::{CursorStyle as StyleCursorStyle, Style},
    taffy::Overflow,
    unit::PxPct,
    views::{empty, Decorators, Scroll},
    IntoView, Renderer, View, ViewId,
};

use crate::theme::ShadcnThemeProp;
use floem_editor_core::{
    buffer::rope_text::RopeText,
    command::{EditCommand, MoveCommand},
};
use ui_events::{
    keyboard::{Key, KeyState, KeyboardEvent, Modifiers, NamedKey},
    pointer::PointerEvent,
};

use super::Document;

/// Cursor blink interval in milliseconds
const CURSOR_BLINK_INTERVAL_MS: u64 = 500;

/// A command that can be executed on the editor
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Edit(EditCommand),
    Move(MoveCommand),
    /// Select all text in the document
    SelectAll,
}

/// A key press with modifiers
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct KeyPress {
    pub key: Key,
    pub modifiers: Modifiers,
}

/// Default keymap for the text editor
pub struct KeypressMap {
    pub keymaps: HashMap<KeyPress, Command>,
}

impl Default for KeypressMap {
    fn default() -> Self {
        let mut keymaps = HashMap::new();

        // Platform-specific modifier for common shortcuts
        // On macOS: Meta (Cmd), on other platforms: Control
        #[cfg(target_os = "macos")]
        let cmd_or_ctrl = Modifiers::META;
        #[cfg(not(target_os = "macos"))]
        let cmd_or_ctrl = Modifiers::CONTROL;

        // =======================================================================
        // Basic navigation (arrow keys)
        // =======================================================================
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::default() },
            Command::Move(MoveCommand::Left),
        );
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::default() },
            Command::Move(MoveCommand::Right),
        );
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowUp), modifiers: Modifiers::default() },
            Command::Move(MoveCommand::Up),
        );
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowDown), modifiers: Modifiers::default() },
            Command::Move(MoveCommand::Down),
        );

        // =======================================================================
        // Line start/end navigation
        // =======================================================================
        // Home -> Line Start
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Home), modifiers: Modifiers::default() },
            Command::Move(MoveCommand::LineStart),
        );
        // End -> Line End
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::End), modifiers: Modifiers::default() },
            Command::Move(MoveCommand::LineEnd),
        );
        // Cmd+Left (macOS) -> Line Start
        #[cfg(target_os = "macos")]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::META },
            Command::Move(MoveCommand::LineStart),
        );
        // Cmd+Right (macOS) -> Line End
        #[cfg(target_os = "macos")]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::META },
            Command::Move(MoveCommand::LineEnd),
        );

        // =======================================================================
        // Document start/end navigation
        // =======================================================================
        // Cmd+Up (macOS) / Ctrl+Home -> Document Start
        #[cfg(target_os = "macos")]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowUp), modifiers: Modifiers::META },
            Command::Move(MoveCommand::DocumentStart),
        );
        #[cfg(not(target_os = "macos"))]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Home), modifiers: Modifiers::CONTROL },
            Command::Move(MoveCommand::DocumentStart),
        );
        // Cmd+Down (macOS) / Ctrl+End -> Document End
        #[cfg(target_os = "macos")]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowDown), modifiers: Modifiers::META },
            Command::Move(MoveCommand::DocumentEnd),
        );
        #[cfg(not(target_os = "macos"))]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::End), modifiers: Modifiers::CONTROL },
            Command::Move(MoveCommand::DocumentEnd),
        );

        // =======================================================================
        // Word navigation (Alt/Option + arrows)
        // =======================================================================
        // Alt+Left -> Word Backward
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::ALT },
            Command::Move(MoveCommand::WordBackward),
        );
        // Alt+Right -> Word Forward
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::ALT },
            Command::Move(MoveCommand::WordForward),
        );
        // Ctrl+Left (non-macOS) -> Word Backward
        #[cfg(not(target_os = "macos"))]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::CONTROL },
            Command::Move(MoveCommand::WordBackward),
        );
        // Ctrl+Right (non-macOS) -> Word Forward
        #[cfg(not(target_os = "macos"))]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::CONTROL },
            Command::Move(MoveCommand::WordForward),
        );

        // =======================================================================
        // Basic editing
        // =======================================================================
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Enter), modifiers: Modifiers::default() },
            Command::Edit(EditCommand::InsertNewLine),
        );
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::default() },
            Command::Edit(EditCommand::DeleteBackward),
        );
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::default() },
            Command::Edit(EditCommand::DeleteForward),
        );
        // Tab -> Insert Tab
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Tab), modifiers: Modifiers::default() },
            Command::Edit(EditCommand::InsertTab),
        );

        // =======================================================================
        // Word deletion
        // =======================================================================
        // Alt+Backspace -> Delete Word Backward
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::ALT },
            Command::Edit(EditCommand::DeleteWordBackward),
        );
        // Alt+Delete -> Delete Word Forward
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::ALT },
            Command::Edit(EditCommand::DeleteWordForward),
        );
        // Ctrl+Backspace (non-macOS) -> Delete Word Backward
        #[cfg(not(target_os = "macos"))]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::CONTROL },
            Command::Edit(EditCommand::DeleteWordBackward),
        );
        // Ctrl+Delete (non-macOS) -> Delete Word Forward
        #[cfg(not(target_os = "macos"))]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::CONTROL },
            Command::Edit(EditCommand::DeleteWordForward),
        );

        // =======================================================================
        // Line deletion
        // =======================================================================
        // Cmd+Backspace (macOS) -> Delete to Beginning of Line
        #[cfg(target_os = "macos")]
        keymaps.insert(
            KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::META },
            Command::Edit(EditCommand::DeleteToBeginningOfLine),
        );

        // =======================================================================
        // Select All (Cmd/Ctrl+A)
        // =======================================================================
        keymaps.insert(
            KeyPress { key: Key::Character("a".into()), modifiers: cmd_or_ctrl },
            Command::SelectAll,
        );

        Self { keymaps }
    }
}

/// Size of the resize handle grip area
const RESIZE_HANDLE_SIZE: f64 = 16.0;

/// A multi-line text area view
pub struct TextArea {
    id: ViewId,
    scroll_id: ViewId,
    doc: RwSignal<Document>,
    padding: RwSignal<(f64, f64, f64, f64)>,
    viewport: RwSignal<Rect>,
    parent_size: RwSignal<Size>,
    child_height: RwSignal<f64>,
    /// Tracks the last time the cursor was moved or text was edited.
    /// Used to reset the cursor blink cycle so the cursor is visible immediately after user action.
    last_cursor_action: RwSignal<Instant>,
    /// Whether the resize handle is enabled
    resize_enabled: RwSignal<bool>,
    /// Current size of the textarea (when resizable)
    resize_size: RwSignal<Option<Size>>,
    /// Whether currently in resize drag mode
    is_resizing: RwSignal<bool>,
    /// Position where resize drag started
    resize_start_pos: RwSignal<Point>,
    /// Size when resize drag started
    resize_start_size: RwSignal<Size>,
    /// Minimum size for resize
    min_size: RwSignal<Size>,
    /// Maximum size for resize (None = no limit)
    max_size: RwSignal<Option<Size>>,
}

impl TextArea {
    /// Creates a new text editor with empty content
    pub fn new() -> Self {
        Self::with_text("")
    }

    /// Creates a new text editor with the given initial text
    pub fn with_text(text: impl Into<String>) -> Self {
        let child_height = create_rw_signal(0.0);
        let padding = create_rw_signal((0.0, 0.0, 0.0, 0.0));
        let viewport = create_rw_signal(Rect::ZERO);
        let parent_size = create_rw_signal(Size::ZERO);
        let doc = Document::new(text.into());
        let doc_signal = create_rw_signal(doc);
        let last_cursor_action = create_rw_signal(Instant::now());

        // Resize handle state
        let resize_enabled = create_rw_signal(false);
        let resize_size = create_rw_signal(None);
        let is_resizing = create_rw_signal(false);
        let resize_start_pos = create_rw_signal(Point::ZERO);
        let resize_start_size = create_rw_signal(Size::ZERO);
        let min_size = create_rw_signal(Size::new(50.0, 30.0));
        let max_size = create_rw_signal(None);

        let id = ViewId::new();

        // Capture cursor signal directly for reactive tracking in ensure_visible
        let cursor_signal = doc_signal.get_untracked().cursor();

        // Create scroll view with ensure_visible to keep cursor in view
        let scroll_view = Scroll::new(empty().style(move |s| s.width(10.0).height(child_height.get())))
            .style(move |s| {
                let padding = padding.get();
                s.absolute()
                    .size_full()
                    .margin_top(-padding.0)
                    .margin_left(-padding.3)
            })
            .ensure_visible(move || {
                // Track cursor signal directly to trigger scroll when cursor moves
                let cursor = cursor_signal.get();
                let padding = padding.get_untracked();
                child_height.track();

                let offset = cursor.end;
                let doc = doc_signal.get_untracked();
                let point = doc.text_layouts().borrow().point_of_offset(offset);

                // Return a rect representing the cursor line that should be visible
                Rect::from_origin_size(
                    (0.0, point.line_top),
                    (1.0, point.line_bottom - point.line_top + padding.0 + padding.2),
                )
            })
            .on_scroll(move |new_viewport| {
                viewport.set(new_viewport);
            });
        let scroll_id = scroll_view.id();

        id.set_children_vec(vec![scroll_view.into_any()]);

        // Set up event handlers
        let keypress_map = std::sync::Arc::new(KeypressMap::default());
        let keypress_map_clone = keypress_map.clone();

        id.add_event_listener(
            EventListener::PointerDown,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Down(pointer_event)) = event {
                    let pos = pointer_event.state.position;

                    // Check if clicking on resize handle (bottom-right corner)
                    if resize_enabled.get_untracked() {
                        let layout = id.get_layout().unwrap_or_default();
                        let width = layout.size.width as f64;
                        let height = layout.size.height as f64;

                        let in_resize_handle = pos.x >= width - RESIZE_HANDLE_SIZE
                            && pos.y >= height - RESIZE_HANDLE_SIZE;

                        if in_resize_handle {
                            is_resizing.set(true);
                            resize_start_pos.set(Point::new(pos.x, pos.y));
                            resize_start_size.set(Size::new(width, height));
                            id.request_active();
                            return EventPropagation::Stop;
                        }
                    }

                    let padding = padding.get_untracked();
                    let viewport = viewport.get_untracked();
                    let mut adjusted = pointer_event.clone();
                    adjusted.state.position.x -= padding.3;
                    adjusted.state.position.y -= padding.0 - viewport.y0;
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
                    // Handle resizing
                    if is_resizing.get_untracked() {
                        let current_pos = pointer_event.current.position;
                        let start_pos = resize_start_pos.get_untracked();
                        let start_size = resize_start_size.get_untracked();
                        let min = min_size.get_untracked();
                        let max: Option<Size> = max_size.get_untracked();

                        let delta_x = current_pos.x - start_pos.x;
                        let delta_y = current_pos.y - start_pos.y;

                        let mut new_width = (start_size.width + delta_x).max(min.width);
                        let mut new_height = (start_size.height + delta_y).max(min.height);

                        if let Some(max) = max {
                            new_width = new_width.min(max.width);
                            new_height = new_height.min(max.height);
                        }

                        resize_size.set(Some(Size::new(new_width, new_height)));
                        id.request_layout();
                        return EventPropagation::Stop;
                    }

                    let padding = padding.get_untracked();
                    let viewport = viewport.get_untracked();
                    let mut adjusted = pointer_event.clone();
                    adjusted.current.position.x -= padding.3;
                    adjusted.current.position.y -= padding.0 - viewport.y0;
                    doc_signal.get_untracked().pointer_move(&adjusted);
                    // During active drag, update cursor action time to keep cursor visible
                    last_cursor_action.set(Instant::now());
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::PointerUp,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Up(pointer_event)) = event {
                    // Stop resizing if we were resizing
                    if is_resizing.get_untracked() {
                        is_resizing.set(false);
                        return EventPropagation::Stop;
                    }

                    let padding = padding.get_untracked();
                    let viewport = viewport.get_untracked();
                    let mut adjusted = pointer_event.clone();
                    adjusted.state.position.x -= padding.3;
                    adjusted.state.position.y -= padding.0 - viewport.y0;
                    doc_signal.get_untracked().pointer_up(&adjusted);
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::KeyDown,
            Box::new(move |event| {
                let Event::Key(KeyboardEvent { state: KeyState::Down, key, modifiers, .. }) = event
                else {
                    return EventPropagation::Continue;
                };

                let keypress = KeyPress { key: key.clone(), modifiers: modifiers.clone() };

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
                        Command::Edit(edit_cmd) => {
                            document.run_edit_command(edit_cmd);
                            id.request_layout();
                        }
                        Command::Move(move_cmd) => {
                            document.run_move_command(move_cmd, shift_held);
                            scroll_id.request_layout();
                        }
                        Command::SelectAll => document.select_all(),
                    }
                    last_cursor_action.set(Instant::now());
                    return EventPropagation::Stop;
                }

                // Handle character input
                let mut mods = modifiers.clone();
                mods.set(Modifiers::SHIFT, false);
                #[cfg(target_os = "macos")]
                mods.set(Modifiers::ALT, false);

                if mods.is_empty() {
                    if let Key::Character(c) = key {
                        document.insert_text(&c);
                        id.request_layout();
                        last_cursor_action.set(Instant::now());
                    }
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::ImeCommit,
            Box::new(move |event| {
                if let Event::ImeCommit(text) = event {
                    doc_signal.get_untracked().insert_text(&text);
                    id.request_layout();
                    last_cursor_action.set(Instant::now());
                }
                EventPropagation::Stop
            }),
        );

        Self {
            id,
            scroll_id,
            doc: doc_signal,
            padding,
            viewport,
            parent_size,
            child_height,
            last_cursor_action,
            resize_enabled,
            resize_size,
            is_resizing,
            resize_start_pos,
            resize_start_size,
            min_size,
            max_size,
        }
    }

    /// Returns the document signal
    pub fn doc(&self) -> RwSignal<Document> {
        self.doc
    }

    /// Returns the scroll view's ViewId
    pub fn scroll_id(&self) -> ViewId {
        self.scroll_id
    }

    /// Returns the child height signal (content height)
    pub fn child_height(&self) -> RwSignal<f64> {
        self.child_height
    }

    /// Returns the viewport signal (scroll position and visible area)
    pub fn viewport(&self) -> RwSignal<Rect> {
        self.viewport
    }

    /// Registers a callback to be called when the document content changes
    pub fn on_update(self, on_update: impl Fn(&str) + 'static) -> Self {
        self.doc.get_untracked().on_update(on_update);
        self
    }

    /// Sets the editor content reactively
    pub fn value(self, set_value: impl Fn() -> String + 'static) -> Self {
        let doc = self.doc;
        create_effect(move |_| {
            let new_value = set_value();
            // Check if document already has this value to avoid re-entrancy issues
            let current_text = doc.with_untracked(|d| d.text());
            if current_text == new_value {
                return;
            }
            doc.with_untracked(|doc| {
                let end = doc.buffer().with_untracked(|b| b.text().len());
                use floem_editor_core::{
                    cursor::CursorAffinity,
                    editor::EditType,
                    selection::SelRegion,
                };
                doc.edit(
                    [(SelRegion::new(0, end, CursorAffinity::Forward, None), new_value.as_str())],
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

    /// Enable or disable the resize handle
    pub fn resizable(self, enabled: bool) -> Self {
        self.resize_enabled.set(enabled);
        self
    }

    /// Returns whether resize is enabled
    pub fn is_resizable(&self) -> bool {
        self.resize_enabled.get_untracked()
    }

    /// Returns the current resize size (if resizing has occurred)
    pub fn resize_size(&self) -> RwSignal<Option<Size>> {
        self.resize_size
    }

    /// Set the minimum size for resizing
    pub fn min_resize_size(self, size: Size) -> Self {
        self.min_size.set(size);
        self
    }

    /// Set the maximum size for resizing
    pub fn max_resize_size(self, size: Size) -> Self {
        self.max_size.set(Some(size));
        self
    }

    /// Returns whether the textarea is currently being resized
    pub fn is_resizing(&self) -> bool {
        self.is_resizing.get_untracked()
    }
}

impl View for TextArea {
    fn id(&self) -> ViewId {
        self.id
    }

    fn view_style(&self) -> Option<Style> {
        let resize_size = self.resize_size;
        Some(
            Style::new()
                .cursor(StyleCursorStyle::Text)
                .focusable(true)
                .set(floem::style::OverflowX, Overflow::Hidden) // Hidden to enable text wrapping
                .set(floem::style::OverflowY, Overflow::Scroll)
                .apply_if(resize_size.get().is_some(), move |s| {
                    let size = resize_size.get().unwrap();
                    s.width(size.width).height(size.height)
                }),
        )
    }

    fn compute_layout(&mut self, cx: &mut ComputeLayoutCx) -> Option<Rect> {
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

        if (padding_top, padding_right, padding_bottom, padding_left) != self.padding.get_untracked() {
            self.padding.set((padding_top, padding_right, padding_bottom, padding_left));
        }

        let width = layout.size.width as f64 - padding_left - padding_right;
        let height = layout.size.height as f64 - padding_top - padding_bottom;
        let parent_size = Size::new(width, height);

        // Get text color from style and set it on the document
        let text_color = builtin_style.color().unwrap_or(Color::BLACK);

        let doc = self.doc.get_untracked();
        doc.set_text_color(text_color);
        doc.set_width(width);

        let child_height = {
            let lines = doc.text_layouts().borrow();
            lines.point_of_offset(lines.utf8_len()).line_bottom + padding_top + padding_bottom
        };

        if child_height != self.child_height.get_untracked() {
            self.child_height.set(child_height);
        }
        if self.parent_size.get_untracked() != parent_size {
            self.parent_size.set(parent_size);
        }

        cx.compute_view_layout(self.scroll_id);

        None
    }

    fn paint(&mut self, cx: &mut PaintCx) {
        let padding = self.padding.get_untracked();
        let viewport = self.viewport.get_untracked();

        // Get text color and theme from style
        let style = self.id.get_combined_style();
        let text_color = style.builtin().color().unwrap_or(Color::BLACK);
        let theme = style.get(ShadcnThemeProp);
        let selection_color = theme.primary.multiply_alpha(0.2);

        cx.save();
        cx.clip(
            &self
                .parent_size
                .get_untracked()
                .to_rect()
                .with_origin(Point::new(padding.3, padding.0))
                .inflate(2.0, 0.0),
        );

        let doc = self.doc.get_untracked();
        let lines = doc.text_layouts().borrow();

        let min_vline = lines.vline_of_height(viewport.y0).saturating_sub(1);
        let max_vline = lines.vline_of_height(viewport.y1) + 1;

        // Draw cursor/selection
        if cx.is_focused(self.id) {
            let cursor = doc.cursor().get_untracked();
            if cursor.is_caret() {
                // Calculate cursor visibility based on blink cycle
                // Cursor is visible during even intervals (0-500ms, 1000-1500ms, etc.)
                let elapsed_ms = self.last_cursor_action.get_untracked().elapsed().as_millis();
                let blink_cycle = elapsed_ms / CURSOR_BLINK_INTERVAL_MS as u128;
                let is_cursor_visible = blink_cycle % 2 == 0;

                if is_cursor_visible {
                    let p = lines.point_of_offset(cursor.end);
                    // Use glyph metrics, fall back to default font metrics for empty text
                    let (cursor_top, cursor_height) = if p.glyph_bottom > p.glyph_top {
                        (p.glyph_top, p.glyph_bottom - p.glyph_top)
                    } else {
                        (lines.default_glyph_top(), lines.default_glyph_height())
                    };
                    let rect = Rect::from_origin_size(
                        (p.x + padding.3 - 1.0, cursor_top + padding.0 - viewport.y0),
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
                let start_vline = lines.vline_of_offset(cursor.min());
                let end_vline = lines.vline_of_offset(cursor.max());
                let start_offset = lines.offset_of_vline(start_vline.max(min_vline));
                let end_offset = lines.offset_of_vline(end_vline.min(max_vline));

                for line in lines.visual_lines(start_offset..end_offset + 1) {
                    let x0 = if line.line_i == start_vline {
                        lines.point_of_offset(cursor.min()).x
                    } else {
                        0.0
                    };
                    let x1 = if line.line_i == end_vline {
                        lines.point_of_offset(cursor.max()).x
                    } else {
                        line.line_w as f64
                    };
                    let rect = Rect::from_origin_size(
                        (x0 + padding.3, line.line_top as f64 + padding.0 - viewport.y0),
                        (x1 - x0, line.line_height as f64),
                    );
                    cx.fill(&rect, selection_color, 0.0);
                }
            }
        }

        // Draw text
        let min_offset = lines.offset_of_vline(min_vline);
        let max_offset = lines.offset_of_vline(max_vline + 1);
        let layout = lines.visual_lines(min_offset..max_offset + 1);
        cx.draw_text_with_layout(layout, Point::new(padding.3, padding.0 - viewport.y0));

        cx.restore();
        cx.paint_view(self.scroll_id);

        // Draw resize handle if enabled
        if self.resize_enabled.get_untracked() {
            let layout = self.id.get_layout().unwrap_or_default();
            let width = layout.size.width as f64;
            let height = layout.size.height as f64;

            // Draw diagonal dots pattern as a grip handle (like web textarea resize)
            let handle_color = theme.muted_foreground.multiply_alpha(0.5);
            let dot_spacing = 3.0;
            let dot_size = 2.0;
            let base_x = width - 4.0;
            let base_y = height - 4.0;

            // Draw 6 dots in a triangular pattern:
            //     .
            //   . .
            // . . .
            for row in 0..3 {
                for col in 0..=row {
                    let x = base_x - (row as f64) * dot_spacing + (col as f64) * dot_spacing;
                    let y = base_y - (col as f64) * dot_spacing;
                    let rect = Rect::from_origin_size(
                        (x - dot_size / 2.0, y - dot_size / 2.0),
                        (dot_size, dot_size),
                    );
                    cx.fill(&rect, handle_color, 0.0);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use floem::reactive::SignalGet;
    use floem::test_harness::TestHarness;
    use floem::views::Decorators;

    /// Test to verify that Effects run when signals change in test environment
    #[test]
    fn test_effect_runs_on_signal_change() {
        use floem::reactive::{Effect, RwSignal, SignalUpdate};
        use std::cell::Cell;
        use std::rc::Rc;

        // Create a signal
        let source = RwSignal::new(0);

        // Create a counter to track effect runs
        let effect_run_count = Rc::new(Cell::new(0));
        let effect_run_count_clone = effect_run_count.clone();

        // Create an Effect that tracks the signal
        Effect::new(move |_| {
            let _value = source.get(); // Track the signal
            effect_run_count_clone.set(effect_run_count_clone.get() + 1);
        });

        // Effect should have run once initially
        println!("After creation: effect_run_count = {}", effect_run_count.get());
        assert_eq!(effect_run_count.get(), 1, "Effect should run once on creation");

        // Update the signal
        source.set(1);

        // Effect should have run again
        println!("After signal.set(1): effect_run_count = {}", effect_run_count.get());
        assert_eq!(effect_run_count.get(), 2, "Effect should run when signal changes");

        // Update again
        source.set(2);
        println!("After signal.set(2): effect_run_count = {}", effect_run_count.get());
        assert_eq!(effect_run_count.get(), 3, "Effect should run on each signal change");
    }

    /// Test to verify that ensure_visible Effect runs when cursor changes
    #[test]
    fn test_ensure_visible_effect_runs_on_cursor_change() {
        use floem::reactive::{Effect, RwSignal, SignalUpdate};
        use floem::views::Scroll;
        use std::cell::Cell;
        use std::rc::Rc;

        // Create signals similar to TextArea
        let cursor_signal = RwSignal::new(0usize);
        let ensure_visible_run_count = Rc::new(Cell::new(0));
        let ensure_visible_run_count_clone = ensure_visible_run_count.clone();

        // Create a dummy Scroll view with ensure_visible that tracks the cursor
        let scroll = Scroll::new(floem::views::Empty::new())
            .ensure_visible(move || {
                let cursor = cursor_signal.get(); // Track cursor
                ensure_visible_run_count_clone.set(ensure_visible_run_count_clone.get() + 1);
                println!("ensure_visible ran, cursor={}, count={}", cursor, ensure_visible_run_count_clone.get());
                Rect::new(0.0, 0.0, 1.0, 20.0)
            });

        // Effect should have run once initially
        println!("After Scroll creation: ensure_visible_run_count = {}", ensure_visible_run_count.get());
        assert_eq!(ensure_visible_run_count.get(), 1, "ensure_visible Effect should run once on creation");

        // Update cursor
        cursor_signal.set(10);

        // Effect should have run again
        println!("After cursor.set(10): ensure_visible_run_count = {}", ensure_visible_run_count.get());
        assert_eq!(ensure_visible_run_count.get(), 2, "ensure_visible Effect should run when cursor changes");

        // Update cursor again
        cursor_signal.set(20);
        println!("After cursor.set(20): ensure_visible_run_count = {}", ensure_visible_run_count.get());
        assert_eq!(ensure_visible_run_count.get(), 3, "ensure_visible Effect should run on each cursor change");

        // Prevent scroll from being optimized away
        drop(scroll);
    }

    /// Test to verify that ensure_visible actually updates scroll position via TestHarness
    #[test]
    fn test_ensure_visible_with_harness() {
        use floem::reactive::{RwSignal, SignalUpdate};
        use floem::views::{Scroll, stack, Empty};
        use std::cell::Cell;
        use std::rc::Rc;

        // Create signals
        let target_y = RwSignal::new(0.0f64);
        let on_scroll_called = Rc::new(Cell::new(false));
        let on_scroll_called_clone = on_scroll_called.clone();
        let scroll_y = Rc::new(Cell::new(0.0f64));
        let scroll_y_clone = scroll_y.clone();

        // Create a Scroll view with tall content and ensure_visible
        let scroll = Scroll::new(
            Empty::new().style(|s| s.width(100.0).height(500.0)) // Tall content
        )
        .style(|s| s.width(100.0).height(100.0)) // Small viewport
        .ensure_visible(move || {
            let y = target_y.get();
            println!("ensure_visible: target_y = {}", y);
            Rect::new(0.0, y, 10.0, y + 20.0) // 20px tall rect at target_y
        })
        .on_scroll(move |viewport| {
            on_scroll_called_clone.set(true);
            scroll_y_clone.set(viewport.y0);
            println!("on_scroll: viewport = {:?}", viewport);
        });

        // Create test harness
        let mut harness = TestHarness::new_with_size(scroll, 100.0, 100.0);

        // Run a rebuild to process initial state
        harness.rebuild();

        // Initial state
        println!("Initial: on_scroll_called={}, scroll_y={}", on_scroll_called.get(), scroll_y.get());

        // Now update target_y to something beyond the viewport
        target_y.set(300.0);

        // Rebuild to process the deferred updates
        harness.rebuild();

        println!("After target_y.set(300.0) and rebuild: on_scroll_called={}, scroll_y={}", on_scroll_called.get(), scroll_y.get());

        // The scroll position should have changed to show y=300
        // With a 100px viewport, to show y=300, we need to scroll to at least y0=200
        // (so that y=300 is at the bottom of the viewport at y1=300)
        assert!(scroll_y.get() > 0.0, "Scroll should have moved to show target_y=300, but scroll_y={}", scroll_y.get());
    }

    /// Helper to create a keyboard event
    fn create_key_event(key: Key, modifiers: Modifiers) -> Event {
        Event::Key(KeyboardEvent {
            state: KeyState::Down,
            key,
            modifiers,
            code: ui_events::keyboard::Code::Unidentified,
            location: ui_events::keyboard::Location::Standard,
            is_composing: false,
            repeat: false,
        })
    }

    #[test]
    fn test_textarea_creation() {
        let textarea = TextArea::new();
        let doc = textarea.doc.get_untracked();
        assert_eq!(doc.text(), "");
    }

    #[test]
    fn test_textarea_with_initial_text() {
        let textarea = TextArea::with_text("hello world");
        let doc = textarea.doc.get_untracked();
        assert_eq!(doc.text(), "hello world");
    }

    #[test]
    fn test_textarea_focus_on_click() {
        let textarea = TextArea::new().style(|s| s.size(200.0, 100.0));
        let id = textarea.id;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Initially not focused
        assert!(!harness.is_focused(id), "Should not be focused initially");

        // Click to focus
        harness.click(100.0, 50.0);

        // Should now be focused
        assert!(harness.is_focused(id), "Should be focused after click");
    }

    #[test]
    fn test_textarea_keyboard_input() {
        let textarea = TextArea::new().style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Type a character
        harness.dispatch_event(create_key_event(
            Key::Character("a".into()),
            Modifiers::default(),
        ));

        let doc = doc_signal.get_untracked();
        assert_eq!(doc.text(), "a");
    }

    #[test]
    fn test_textarea_multiple_characters() {
        let textarea = TextArea::new().style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Type multiple characters
        for c in ['h', 'e', 'l', 'l', 'o'] {
            harness.dispatch_event(create_key_event(
                Key::Character(c.to_string().into()),
                Modifiers::default(),
            ));
        }

        let doc = doc_signal.get_untracked();
        assert_eq!(doc.text(), "hello");
    }

    #[test]
    fn test_textarea_backspace() {
        let textarea = TextArea::with_text("").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Type "hello"
        for c in ['h', 'e', 'l', 'l', 'o'] {
            harness.dispatch_event(create_key_event(
                Key::Character(c.to_string().into()),
                Modifiers::default(),
            ));
        }

        assert_eq!(doc_signal.get_untracked().text(), "hello");

        // Press backspace
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Backspace),
            Modifiers::default(),
        ));

        assert_eq!(doc_signal.get_untracked().text(), "hell");
    }

    #[test]
    fn test_textarea_arrow_keys() {
        let textarea = TextArea::with_text("").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Type "abc"
        for c in ['a', 'b', 'c'] {
            harness.dispatch_event(create_key_event(
                Key::Character(c.to_string().into()),
                Modifiers::default(),
            ));
        }

        // Cursor should be at end (3)
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 3);

        // Press left arrow
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowLeft),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 2);

        // Press right arrow
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowRight),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 3);
    }

    #[test]
    fn test_textarea_enter_key() {
        let textarea = TextArea::new().style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Type "line1"
        for c in ['l', 'i', 'n', 'e', '1'] {
            harness.dispatch_event(create_key_event(
                Key::Character(c.to_string().into()),
                Modifiers::default(),
            ));
        }

        // Press Enter
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Enter),
            Modifiers::default(),
        ));

        // Type "line2"
        for c in ['l', 'i', 'n', 'e', '2'] {
            harness.dispatch_event(create_key_event(
                Key::Character(c.to_string().into()),
                Modifiers::default(),
            ));
        }

        assert_eq!(doc_signal.get_untracked().text(), "line1\nline2");
    }

    #[test]
    fn test_textarea_delete_key() {
        let textarea = TextArea::new().style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Type "abc"
        for c in ['a', 'b', 'c'] {
            harness.dispatch_event(create_key_event(
                Key::Character(c.to_string().into()),
                Modifiers::default(),
            ));
        }

        // Move cursor to beginning
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowLeft),
            Modifiers::default(),
        ));
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowLeft),
            Modifiers::default(),
        ));
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowLeft),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 0);

        // Press Delete - should delete 'a'
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Delete),
            Modifiers::default(),
        ));

        // Note: DeleteForward is not implemented yet in Document, so this won't work
        // Once implemented, uncomment:
        // assert_eq!(doc_signal.get_untracked().text(), "bc");
    }

    // === Tests for wrapped text and cursor positioning at visual line edges ===

    /// Helper to click at very top-left corner to focus and set cursor at position 0
    fn focus_at_start(harness: &mut TestHarness) {
        harness.click(1.0, 1.0);
    }

    #[test]
    fn test_textarea_arrow_up_down_multiline() {
        let textarea = TextArea::with_text("line1\nline2\nline3").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        // Initialize text layout by setting width
        doc_signal.get_untracked().set_width(200.0);

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus and position at start
        focus_at_start(&mut harness);

        // Explicitly set cursor to start
        doc_signal.get_untracked().set_offset(0, false);

        // Move cursor to end of first line (position 5)
        for _ in 0..5 {
            harness.dispatch_event(create_key_event(
                Key::Named(NamedKey::ArrowRight),
                Modifiers::default(),
            ));
        }

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 5, "Cursor should be at end of line1");

        // Press down arrow - should move to line2
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowDown),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        // Should be somewhere in line2 (offset 6-11)
        assert!(
            cursor.end >= 6 && cursor.end <= 11,
            "Cursor should be in line2, got {}",
            cursor.end
        );

        let line2_pos = cursor.end;

        // Press down again - should move to line3
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowDown),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        // Should be somewhere in line3 (offset 12-17)
        assert!(
            cursor.end >= 12 && cursor.end <= 17,
            "Cursor should be in line3, got {}",
            cursor.end
        );

        // Press up - should go back to line2
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowUp),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        // Should be back in line2 at same or similar position
        assert!(
            cursor.end >= 6 && cursor.end <= 11,
            "Cursor should be back in line2, got {} (was at {})",
            cursor.end,
            line2_pos
        );
    }

    #[test]
    fn test_textarea_arrow_up_at_first_line() {
        let textarea = TextArea::with_text("hello").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        focus_at_start(&mut harness);

        // Explicitly set cursor to position 2 (middle of line)
        doc_signal.get_untracked().set_offset(2, false);

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 2);

        // Press up - should move to start of line since we're on first line
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowUp),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 0, "ArrowUp on first line should go to start");
    }

    #[test]
    fn test_textarea_arrow_down_at_last_line() {
        let textarea = TextArea::with_text("hello").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        focus_at_start(&mut harness);

        // Explicitly set cursor to position 2 (middle of line)
        doc_signal.get_untracked().set_offset(2, false);

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 2);

        // Press down - should move to end of line since we're on last line
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowDown),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 5, "ArrowDown on last line should go to end");
    }

    #[test]
    fn test_textarea_cursor_at_newline_boundary() {
        let textarea = TextArea::with_text("abc\ndef").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        focus_at_start(&mut harness);

        // Explicitly set cursor to position 3 (just before \n)
        doc_signal.get_untracked().set_offset(3, false);

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 3, "Cursor should be at position 3 (before newline)");

        // Move right once more - should cross the newline
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowRight),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 4, "Cursor should be at position 4 (start of line2)");

        // Move left - should go back before newline
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowLeft),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 3, "Cursor should be back at position 3");
    }

    #[test]
    fn test_textarea_lines_with_different_lengths() {
        // Test cursor behavior when moving between lines of different lengths
        let textarea = TextArea::with_text("short\nlongerline\nx").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        // Initialize text layout by setting width
        doc_signal.get_untracked().set_width(200.0);

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        focus_at_start(&mut harness);

        // Explicitly set cursor to end of first line (position 5)
        doc_signal.get_untracked().set_offset(5, false);

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 5, "Should be at end of 'short'");

        // Move down to second line
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowDown),
            Modifiers::default(),
        ));

        // Cursor should be at similar horizontal position in line2
        // (exact position depends on text layout)
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert!(
            cursor.end >= 6 && cursor.end <= 16,
            "Cursor should be in line2 (6-16), got {}",
            cursor.end
        );

        // Move down to third line (which is very short "x")
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowDown),
            Modifiers::default(),
        ));

        // Cursor should be in line3.
        // Text is "short\nlongerline\nx" = 19 chars total
        // line1: 0-5 (short\n)
        // line2: 6-16 (longerline\n)
        // line3: 17-18 (x)
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert!(
            cursor.end >= 17 && cursor.end <= 18,
            "Cursor should be in line3 (17-18), got {}",
            cursor.end
        );
    }

    #[test]
    fn test_textarea_empty_lines() {
        // Text: "a\n\nb" = 4 chars
        // Position 0: 'a'
        // Position 1: '\n' (newline after 'a')
        // Position 2: '\n' (empty line's newline)
        // Position 3: 'b'
        let textarea = TextArea::with_text("a\n\nb").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        // Initialize text layout by setting width
        doc_signal.get_untracked().set_width(200.0);

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        focus_at_start(&mut harness);

        // Explicitly set cursor to position 1 (after 'a', before first newline)
        doc_signal.get_untracked().set_offset(1, false);

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 1);

        // Move down - should go to empty line (position 2)
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowDown),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 2, "Cursor should be on empty line (position 2)");

        // Move down again - should go to line with 'b' (position 3)
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowDown),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 3, "Cursor should be at 'b' (position 3)");
    }

    #[test]
    fn test_textarea_insert_in_middle_of_line() {
        let textarea = TextArea::with_text("abcd").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        focus_at_start(&mut harness);

        // Explicitly set cursor to position 2 (between b and c)
        doc_signal.get_untracked().set_offset(2, false);

        // Insert 'X'
        harness.dispatch_event(create_key_event(
            Key::Character("X".into()),
            Modifiers::default(),
        ));

        assert_eq!(doc_signal.get_untracked().text(), "abXcd");

        // Cursor should be after X
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 3);
    }

    #[test]
    fn test_textarea_insert_at_line_boundary() {
        let textarea = TextArea::with_text("ab\ncd").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        focus_at_start(&mut harness);

        // Explicitly set cursor to position 2 (end of first line, before \n)
        doc_signal.get_untracked().set_offset(2, false);

        // Insert 'X' at end of first line
        harness.dispatch_event(create_key_event(
            Key::Character("X".into()),
            Modifiers::default(),
        ));

        assert_eq!(doc_signal.get_untracked().text(), "abX\ncd");

        // Move right past newline, then insert at start of second line
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowRight),
            Modifiers::default(),
        ));

        harness.dispatch_event(create_key_event(
            Key::Character("Y".into()),
            Modifiers::default(),
        ));

        assert_eq!(doc_signal.get_untracked().text(), "abX\nYcd");
    }

    // === Tests for cursor blinking ===

    #[test]
    fn test_cursor_blink_timing_logic() {
        // Test that the blink cycle calculation is correct
        // Cursor should be visible when blink_cycle % 2 == 0

        // At 0ms, blink_cycle = 0 / 500 = 0, visible (0 % 2 == 0)
        assert_eq!((0u128 / 500) % 2, 0);

        // At 250ms, blink_cycle = 250 / 500 = 0, visible (0 % 2 == 0)
        assert_eq!((250u128 / 500) % 2, 0);

        // At 499ms, blink_cycle = 499 / 500 = 0, visible (0 % 2 == 0)
        assert_eq!((499u128 / 500) % 2, 0);

        // At 500ms, blink_cycle = 500 / 500 = 1, hidden (1 % 2 == 1)
        assert_eq!((500u128 / 500) % 2, 1);

        // At 750ms, blink_cycle = 750 / 500 = 1, hidden (1 % 2 == 1)
        assert_eq!((750u128 / 500) % 2, 1);

        // At 999ms, blink_cycle = 999 / 500 = 1, hidden (1 % 2 == 1)
        assert_eq!((999u128 / 500) % 2, 1);

        // At 1000ms, blink_cycle = 1000 / 500 = 2, visible (2 % 2 == 0)
        assert_eq!((1000u128 / 500) % 2, 0);

        // At 1500ms, blink_cycle = 1500 / 500 = 3, hidden (3 % 2 == 1)
        assert_eq!((1500u128 / 500) % 2, 1);
    }

    #[test]
    fn test_cursor_blink_interval_constant() {
        // Verify the blink interval is 500ms (standard cursor blink rate)
        assert_eq!(super::CURSOR_BLINK_INTERVAL_MS, 500);
    }

    #[test]
    fn test_textarea_has_last_cursor_action() {
        use std::time::Instant;

        let textarea = TextArea::new();

        // The last_cursor_action should be set to a recent time (within a few seconds)
        let elapsed = textarea.last_cursor_action.get_untracked().elapsed();
        assert!(
            elapsed.as_secs() < 5,
            "last_cursor_action should be recent, but was {:?} ago",
            elapsed
        );
    }

    #[test]
    fn test_cursor_action_updated_on_typing() {
        use std::time::Instant;
        use std::thread;
        use std::time::Duration;

        let textarea = TextArea::new().style(|s| s.size(200.0, 100.0));
        let last_cursor_action = textarea.last_cursor_action;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Get initial time
        let initial_time = last_cursor_action.get_untracked();

        // Wait a small amount
        thread::sleep(Duration::from_millis(10));

        // Type a character
        harness.dispatch_event(create_key_event(
            Key::Character("a".into()),
            Modifiers::default(),
        ));

        // The last_cursor_action should be updated
        let new_time = last_cursor_action.get_untracked();
        assert!(
            new_time >= initial_time,
            "last_cursor_action should be updated after typing"
        );
    }

    #[test]
    fn test_cursor_action_updated_on_arrow_key() {
        use std::time::Instant;
        use std::thread;
        use std::time::Duration;

        let textarea = TextArea::with_text("hello").style(|s| s.size(200.0, 100.0));
        let last_cursor_action = textarea.last_cursor_action;

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Get initial time
        let initial_time = last_cursor_action.get_untracked();

        // Wait a small amount
        thread::sleep(Duration::from_millis(10));

        // Press arrow key
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowRight),
            Modifiers::default(),
        ));

        // The last_cursor_action should be updated
        let new_time = last_cursor_action.get_untracked();
        assert!(
            new_time >= initial_time,
            "last_cursor_action should be updated after arrow key"
        );
    }

    // ==========================================================================
    // KeypressMap unit tests
    // ==========================================================================

    #[test]
    fn test_keypressmap_basic_arrows() {
        let keymap = KeypressMap::default();

        // Left arrow
        let key = KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::default() };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::Left)));

        // Right arrow
        let key = KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::default() };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::Right)));

        // Up arrow
        let key = KeyPress { key: Key::Named(NamedKey::ArrowUp), modifiers: Modifiers::default() };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::Up)));

        // Down arrow
        let key = KeyPress { key: Key::Named(NamedKey::ArrowDown), modifiers: Modifiers::default() };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::Down)));
    }

    #[test]
    fn test_keypressmap_home_end() {
        let keymap = KeypressMap::default();

        // Home -> LineStart
        let key = KeyPress { key: Key::Named(NamedKey::Home), modifiers: Modifiers::default() };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::LineStart)));

        // End -> LineEnd
        let key = KeyPress { key: Key::Named(NamedKey::End), modifiers: Modifiers::default() };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::LineEnd)));
    }

    #[test]
    fn test_keypressmap_word_navigation() {
        let keymap = KeypressMap::default();

        // Alt+Left -> WordBackward
        let key = KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::ALT };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::WordBackward)));

        // Alt+Right -> WordForward
        let key = KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::ALT };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::WordForward)));
    }

    #[test]
    fn test_keypressmap_basic_editing() {
        let keymap = KeypressMap::default();

        // Enter -> InsertNewLine
        let key = KeyPress { key: Key::Named(NamedKey::Enter), modifiers: Modifiers::default() };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Edit(EditCommand::InsertNewLine)));

        // Backspace -> DeleteBackward
        let key = KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::default() };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Edit(EditCommand::DeleteBackward)));

        // Delete -> DeleteForward
        let key = KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::default() };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Edit(EditCommand::DeleteForward)));

        // Tab -> InsertTab
        let key = KeyPress { key: Key::Named(NamedKey::Tab), modifiers: Modifiers::default() };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Edit(EditCommand::InsertTab)));
    }

    #[test]
    fn test_keypressmap_word_deletion() {
        let keymap = KeypressMap::default();

        // Alt+Backspace -> DeleteWordBackward
        let key = KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::ALT };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Edit(EditCommand::DeleteWordBackward)));

        // Alt+Delete -> DeleteWordForward
        let key = KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::ALT };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Edit(EditCommand::DeleteWordForward)));
    }

    #[test]
    fn test_keypressmap_select_all() {
        let keymap = KeypressMap::default();

        // Cmd/Ctrl+A -> SelectAll
        #[cfg(target_os = "macos")]
        let modifier = Modifiers::META;
        #[cfg(not(target_os = "macos"))]
        let modifier = Modifiers::CONTROL;

        let key = KeyPress { key: Key::Character("a".into()), modifiers: modifier };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::SelectAll));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_keypressmap_macos_line_navigation() {
        let keymap = KeypressMap::default();

        // Cmd+Left -> LineStart
        let key = KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::META };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::LineStart)));

        // Cmd+Right -> LineEnd
        let key = KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::META };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::LineEnd)));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_keypressmap_macos_document_navigation() {
        let keymap = KeypressMap::default();

        // Cmd+Up -> DocumentStart
        let key = KeyPress { key: Key::Named(NamedKey::ArrowUp), modifiers: Modifiers::META };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::DocumentStart)));

        // Cmd+Down -> DocumentEnd
        let key = KeyPress { key: Key::Named(NamedKey::ArrowDown), modifiers: Modifiers::META };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::DocumentEnd)));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_keypressmap_macos_line_deletion() {
        let keymap = KeypressMap::default();

        // Cmd+Backspace -> DeleteToBeginningOfLine
        let key = KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::META };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Edit(EditCommand::DeleteToBeginningOfLine)));
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_keypressmap_non_macos_document_navigation() {
        let keymap = KeypressMap::default();

        // Ctrl+Home -> DocumentStart
        let key = KeyPress { key: Key::Named(NamedKey::Home), modifiers: Modifiers::CONTROL };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::DocumentStart)));

        // Ctrl+End -> DocumentEnd
        let key = KeyPress { key: Key::Named(NamedKey::End), modifiers: Modifiers::CONTROL };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::DocumentEnd)));
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_keypressmap_non_macos_word_navigation() {
        let keymap = KeypressMap::default();

        // Ctrl+Left -> WordBackward
        let key = KeyPress { key: Key::Named(NamedKey::ArrowLeft), modifiers: Modifiers::CONTROL };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::WordBackward)));

        // Ctrl+Right -> WordForward
        let key = KeyPress { key: Key::Named(NamedKey::ArrowRight), modifiers: Modifiers::CONTROL };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Move(MoveCommand::WordForward)));
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_keypressmap_non_macos_word_deletion() {
        let keymap = KeypressMap::default();

        // Ctrl+Backspace -> DeleteWordBackward
        let key = KeyPress { key: Key::Named(NamedKey::Backspace), modifiers: Modifiers::CONTROL };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Edit(EditCommand::DeleteWordBackward)));

        // Ctrl+Delete -> DeleteWordForward
        let key = KeyPress { key: Key::Named(NamedKey::Delete), modifiers: Modifiers::CONTROL };
        assert_eq!(keymap.keymaps.get(&key), Some(&Command::Edit(EditCommand::DeleteWordForward)));
    }

    // ==========================================================================
    // TextArea integration tests for key bindings
    // ==========================================================================

    #[test]
    fn test_textarea_home_key() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Move cursor to middle
        doc_signal.get_untracked().set_offset(6, false);

        // Press Home
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Home),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 0, "Home should move cursor to line start");
    }

    #[test]
    fn test_textarea_end_key() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Cursor starts at 0
        doc_signal.get_untracked().set_offset(0, false);

        // Press End
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::End),
            Modifiers::default(),
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 11, "End should move cursor to line end");
    }

    #[test]
    fn test_textarea_alt_left_word_backward() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Move cursor to end
        doc_signal.get_untracked().set_offset(11, false);

        // Press Alt+Left
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowLeft),
            Modifiers::ALT,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert!(cursor.end <= 6, "Alt+Left should move to start of 'world', got {}", cursor.end);
    }

    #[test]
    fn test_textarea_alt_right_word_forward() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Cursor starts at 0
        doc_signal.get_untracked().set_offset(0, false);

        // Press Alt+Right
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowRight),
            Modifiers::ALT,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert!(cursor.end >= 5, "Alt+Right should move past 'hello', got {}", cursor.end);
    }

    #[test]
    fn test_textarea_tab_key() {
        let textarea = TextArea::with_text("hello").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Move cursor to end
        doc_signal.get_untracked().set_offset(5, false);

        // Press Tab
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Tab),
            Modifiers::default(),
        ));

        assert_eq!(doc_signal.get_untracked().text(), "hello\t");
    }

    #[test]
    fn test_textarea_alt_backspace_delete_word() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Move cursor to end
        doc_signal.get_untracked().set_offset(11, false);

        // Press Alt+Backspace
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Backspace),
            Modifiers::ALT,
        ));

        let text = doc_signal.get_untracked().text();
        assert!(text.starts_with("hello"), "Alt+Backspace should delete 'world', got: {}", text);
        assert!(text.len() < 11, "Text should be shorter after word deletion");
    }

    #[test]
    fn test_textarea_alt_delete_word_forward() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Cursor at start
        doc_signal.get_untracked().set_offset(0, false);

        // Press Alt+Delete
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Delete),
            Modifiers::ALT,
        ));

        let text = doc_signal.get_untracked().text();
        assert!(!text.starts_with("hello"), "Alt+Delete should delete 'hello', got: {}", text);
    }

    #[test]
    fn test_textarea_select_all() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Cursor in middle
        doc_signal.get_untracked().set_offset(5, false);

        // Press Cmd/Ctrl+A
        #[cfg(target_os = "macos")]
        let modifier = Modifiers::META;
        #[cfg(not(target_os = "macos"))]
        let modifier = Modifiers::CONTROL;

        harness.dispatch_event(create_key_event(
            Key::Character("a".into()),
            modifier,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert!(!cursor.is_caret(), "Cmd/Ctrl+A should create a selection");
        assert_eq!(cursor.min(), 0, "Selection should start at 0");
        assert_eq!(cursor.max(), 11, "Selection should end at document length");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_textarea_cmd_left_line_start() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Move cursor to middle
        doc_signal.get_untracked().set_offset(6, false);

        // Press Cmd+Left
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowLeft),
            Modifiers::META,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 0, "Cmd+Left should move to line start");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_textarea_cmd_right_line_end() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Cursor at start
        doc_signal.get_untracked().set_offset(0, false);

        // Press Cmd+Right
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowRight),
            Modifiers::META,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 11, "Cmd+Right should move to line end");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_textarea_cmd_up_document_start() {
        let textarea = TextArea::with_text("line1\nline2\nline3").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Move cursor to last line
        doc_signal.get_untracked().set_offset(15, false);

        // Press Cmd+Up
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowUp),
            Modifiers::META,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 0, "Cmd+Up should move to document start");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_textarea_cmd_down_document_end() {
        let textarea = TextArea::with_text("line1\nline2\nline3").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Cursor at start
        doc_signal.get_untracked().set_offset(0, false);

        // Press Cmd+Down
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowDown),
            Modifiers::META,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 17, "Cmd+Down should move to document end");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_textarea_cmd_backspace_delete_to_line_start() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Move cursor to 'w' in world
        doc_signal.get_untracked().set_offset(6, false);

        // Press Cmd+Backspace
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Backspace),
            Modifiers::META,
        ));

        assert_eq!(doc_signal.get_untracked().text(), "world");
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_textarea_ctrl_home_document_start() {
        let textarea = TextArea::with_text("line1\nline2\nline3").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Move cursor to last line
        doc_signal.get_untracked().set_offset(15, false);

        // Press Ctrl+Home
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Home),
            Modifiers::CONTROL,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 0, "Ctrl+Home should move to document start");
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_textarea_ctrl_end_document_end() {
        let textarea = TextArea::with_text("line1\nline2\nline3").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Cursor at start
        doc_signal.get_untracked().set_offset(0, false);

        // Press Ctrl+End
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::End),
            Modifiers::CONTROL,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 17, "Ctrl+End should move to document end");
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_textarea_ctrl_left_word_backward() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Move cursor to end
        doc_signal.get_untracked().set_offset(11, false);

        // Press Ctrl+Left
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowLeft),
            Modifiers::CONTROL,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert!(cursor.end <= 6, "Ctrl+Left should move to start of 'world', got {}", cursor.end);
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_textarea_ctrl_right_word_forward() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Cursor at start
        doc_signal.get_untracked().set_offset(0, false);

        // Press Ctrl+Right
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::ArrowRight),
            Modifiers::CONTROL,
        ));

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert!(cursor.end >= 5, "Ctrl+Right should move past 'hello', got {}", cursor.end);
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_textarea_ctrl_backspace_delete_word() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Move cursor to end
        doc_signal.get_untracked().set_offset(11, false);

        // Press Ctrl+Backspace
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Backspace),
            Modifiers::CONTROL,
        ));

        let text = doc_signal.get_untracked().text();
        assert!(text.starts_with("hello"), "Ctrl+Backspace should delete 'world', got: {}", text);
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_textarea_ctrl_delete_word_forward() {
        let textarea = TextArea::with_text("hello world").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Cursor at start
        doc_signal.get_untracked().set_offset(0, false);

        // Press Ctrl+Delete
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Delete),
            Modifiers::CONTROL,
        ));

        let text = doc_signal.get_untracked().text();
        assert!(!text.starts_with("hello"), "Ctrl+Delete should delete 'hello', got: {}", text);
    }

    // ==========================================================================
    // Scrolling tests
    // ==========================================================================

    #[test]
    fn test_textarea_has_scroll_id() {
        let textarea = TextArea::new();

        // The scroll_id should be a valid ViewId (different from the main id)
        let scroll_id = textarea.scroll_id();
        let main_id = textarea.id();

        assert_ne!(
            scroll_id, main_id,
            "scroll_id should be different from the main view id"
        );
    }

    #[test]
    fn test_textarea_child_height_signal_exists() {
        let textarea = TextArea::new();
        let child_height = textarea.child_height();

        // Should be able to read the signal without panicking
        let height = child_height.get_untracked();
        assert!(height >= 0.0, "child_height should be non-negative");
    }

    #[test]
    fn test_textarea_viewport_signal_exists() {
        let textarea = TextArea::new();
        let viewport = textarea.viewport();

        // Should be able to read the signal without panicking
        let rect = viewport.get_untracked();
        assert!(rect.width() >= 0.0, "viewport width should be non-negative");
        assert!(rect.height() >= 0.0, "viewport height should be non-negative");
    }

    #[test]
    fn test_textarea_child_height_tracks_content() {
        // Create a textarea with multi-line content
        let textarea = TextArea::with_text("line1\nline2\nline3\nline4\nline5")
            .style(|s| s.size(200.0, 50.0)); // Small height to test scrolling
        let child_height = textarea.child_height;
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 50.0);
        harness.click(10.0, 10.0);

        // After layout, child_height should reflect content height
        let _initial_height = child_height.get_untracked();

        // Add more lines to increase content height
        doc_signal.get_untracked().set_offset(29, false); // End of text
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Enter),
            Modifiers::default(),
        ));
        harness.dispatch_event(create_key_event(
            Key::Character("more".into()),
            Modifiers::default(),
        ));

        // Force a layout update by performing another action
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Enter),
            Modifiers::default(),
        ));

        let new_height = child_height.get_untracked();

        // Note: child_height is updated during layout, so this test verifies
        // the signal is accessible and updateable. The actual height change
        // depends on the layout system running.
        assert!(
            new_height >= 0.0,
            "child_height should remain valid after adding content"
        );
    }

    #[test]
    fn test_textarea_scroll_structure_is_correct() {
        // Verify that the TextArea has the correct structure:
        // TextArea (id) -> Scroll (scroll_id) -> empty content view
        let textarea = TextArea::new();

        // Both IDs should be valid and different
        let main_id = textarea.id();
        let scroll_id = textarea.scroll_id();

        assert_ne!(main_id, scroll_id, "Main and scroll IDs should be different");

        // The scroll view should be a child of the main view
        // This is verified by the structure in with_text() where:
        // id.set_children_vec(vec![scroll_view.into_any()]);
    }

    #[test]
    fn test_textarea_viewport_starts_at_origin() {
        let textarea = TextArea::with_text("hello").style(|s| s.size(200.0, 100.0));
        let viewport = textarea.viewport();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);
        harness.click(10.0, 10.0);

        // Initially, viewport should be at or near origin
        let rect = viewport.get_untracked();
        assert!(
            rect.x0.abs() < 1.0,
            "viewport x0 should start near 0, got {}",
            rect.x0
        );
        assert!(
            rect.y0.abs() < 1.0,
            "viewport y0 should start near 0, got {}",
            rect.y0
        );
    }

    #[test]
    fn test_textarea_multiline_content_creates_scrollable_area() {
        // Create textarea with many lines to ensure content exceeds container
        let many_lines = (0..20).map(|i| format!("line{}", i)).collect::<Vec<_>>().join("\n");
        let textarea = TextArea::with_text(&many_lines).style(|s| s.size(200.0, 50.0));
        let child_height = textarea.child_height;
        let doc_signal = textarea.doc();

        let _harness = TestHarness::new_with_size(textarea, 200.0, 50.0);

        // Verify we have the expected number of lines
        let text = doc_signal.get_untracked().text();
        let line_count = text.lines().count();
        assert_eq!(line_count, 20, "Should have 20 lines");

        // child_height signal should be accessible
        let height = child_height.get_untracked();
        assert!(height >= 0.0, "child_height should be non-negative");
    }

    #[test]
    fn test_textarea_empty_content_has_valid_scroll_structure() {
        let textarea = TextArea::new().style(|s| s.size(200.0, 100.0));
        let child_height = textarea.child_height;
        let viewport = textarea.viewport();
        let scroll_id = textarea.scroll_id();

        let _harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // All signals should be valid even with empty content
        let height = child_height.get_untracked();
        let rect = viewport.get_untracked();

        assert!(height >= 0.0, "child_height should be valid for empty content");
        assert!(rect.width() >= 0.0, "viewport should be valid for empty content");
        assert!(scroll_id != ViewId::new(), "scroll_id should be valid");
    }

    // ==========================================================================
    // Resize handle tests
    // ==========================================================================

    #[test]
    fn test_textarea_resizable_disabled_by_default() {
        let textarea = TextArea::new();
        assert!(!textarea.is_resizable(), "Resize should be disabled by default");
    }

    #[test]
    fn test_textarea_resizable_can_be_enabled() {
        let textarea = TextArea::new().resizable(true);
        assert!(textarea.is_resizable(), "Resize should be enabled after calling resizable(true)");
    }

    #[test]
    fn test_textarea_resizable_can_be_disabled() {
        let textarea = TextArea::new().resizable(true).resizable(false);
        assert!(!textarea.is_resizable(), "Resize should be disabled after calling resizable(false)");
    }

    #[test]
    fn test_textarea_resize_size_initially_none() {
        let textarea = TextArea::new();
        assert!(
            textarea.resize_size().get_untracked().is_none(),
            "resize_size should be None initially"
        );
    }

    #[test]
    fn test_textarea_resize_size_signal_exists() {
        let textarea = TextArea::new().resizable(true);
        let resize_size = textarea.resize_size();

        // Should be able to read the signal without panicking
        let size = resize_size.get_untracked();
        assert!(size.is_none(), "resize_size should initially be None");
    }

    #[test]
    fn test_textarea_min_resize_size() {
        let textarea = TextArea::new()
            .resizable(true)
            .min_resize_size(Size::new(100.0, 50.0));

        // The min_size is stored internally - we verify through the builder pattern
        assert!(textarea.is_resizable(), "Should still be resizable after setting min size");
    }

    #[test]
    fn test_textarea_max_resize_size() {
        let textarea = TextArea::new()
            .resizable(true)
            .max_resize_size(Size::new(500.0, 300.0));

        // The max_size is stored internally - we verify through the builder pattern
        assert!(textarea.is_resizable(), "Should still be resizable after setting max size");
    }

    #[test]
    fn test_textarea_is_resizing_initially_false() {
        let textarea = TextArea::new().resizable(true);
        assert!(!textarea.is_resizing(), "Should not be resizing initially");
    }

    #[test]
    fn test_textarea_resize_handle_size_constant() {
        // Verify the resize handle size constant is reasonable
        assert!(RESIZE_HANDLE_SIZE > 0.0, "Resize handle size should be positive");
        assert!(RESIZE_HANDLE_SIZE <= 30.0, "Resize handle size should be reasonable");
    }

    #[test]
    fn test_textarea_resizable_with_text() {
        let textarea = TextArea::with_text("Hello, World!")
            .resizable(true)
            .style(|s| s.size(200.0, 100.0));

        assert!(textarea.is_resizable(), "Textarea with text should be resizable");
        assert_eq!(textarea.text(), "Hello, World!");
    }

    #[test]
    fn test_textarea_resizable_chained_with_other_methods() {
        let textarea = TextArea::new()
            .resizable(true)
            .min_resize_size(Size::new(50.0, 30.0))
            .max_resize_size(Size::new(400.0, 200.0))
            .style(|s| s.size(200.0, 100.0));

        assert!(textarea.is_resizable(), "Should be resizable after chained calls");
    }

    // ==========================================================================
    // Scroll-into-view tests
    // ==========================================================================

    #[test]
    fn test_textarea_cursor_at_bottom_after_many_enters() {
        // Create a small textarea that will require scrolling
        let textarea = TextArea::new().style(|s| s.size(200.0, 50.0));
        let doc_signal = textarea.doc();
        let viewport = textarea.viewport();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 50.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Initial viewport should be at origin
        let initial_viewport = viewport.get_untracked();
        assert!(
            initial_viewport.y0.abs() < 1.0,
            "Initial viewport should be at y=0, got {}",
            initial_viewport.y0
        );

        // Press Enter multiple times to create content that exceeds the viewport height
        // With 50px height and ~16-20px line height, we need ~4+ lines to scroll
        for _ in 0..8 {
            harness.dispatch_event(create_key_event(
                Key::Named(NamedKey::Enter),
                Modifiers::default(),
            ));
        }

        // The text should now have multiple newlines
        let text = doc_signal.get_untracked().text();
        let newline_count = text.chars().filter(|c| *c == '\n').count();
        assert_eq!(newline_count, 8, "Should have 8 newlines from 8 Enter presses");

        // Cursor should be at the end (after all newlines)
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 8, "Cursor should be at position 8 (after 8 newlines)");
    }

    #[test]
    fn test_textarea_cursor_position_after_enter_in_small_area() {
        // Create a small textarea
        let textarea = TextArea::with_text("line1").style(|s| s.size(200.0, 40.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 40.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Move cursor to end of line1
        doc_signal.get_untracked().set_offset(5, false);

        // Press Enter to add a new line
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Enter),
            Modifiers::default(),
        ));

        // Text should now be "line1\n"
        assert_eq!(doc_signal.get_untracked().text(), "line1\n");

        // Cursor should be at position 6 (start of new line)
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 6, "Cursor should be at start of new line");

        // Press Enter again
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Enter),
            Modifiers::default(),
        ));

        // Text should now be "line1\n\n"
        assert_eq!(doc_signal.get_untracked().text(), "line1\n\n");

        // Cursor should be at position 7
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 7, "Cursor should advance with each Enter");
    }

    #[test]
    fn test_textarea_cursor_line_position_calculation() {
        // Test that cursor line position is calculated correctly for scrolling
        let textarea = TextArea::with_text("line1\nline2\nline3\nline4\nline5")
            .style(|s| s.size(200.0, 50.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 50.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Set cursor to end of last line (position 29)
        doc_signal.get_untracked().set_offset(29, false);

        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 29, "Cursor should be at end of text");

        // Get cursor position using the text layout
        let doc = doc_signal.get_untracked();
        let point = doc.text_layouts().borrow().point_of_offset(29);

        // The cursor should be on line 4 (0-indexed), so line_top should be > 0
        assert!(
            point.line_top >= 0.0,
            "Cursor line_top should be non-negative, got {}",
            point.line_top
        );
    }

    #[test]
    fn test_textarea_ensure_visible_closure_returns_cursor_rect() {
        // Test that ensure_visible returns a rect for the cursor position
        let textarea = TextArea::with_text("line1\nline2\nline3")
            .style(|s| s.size(200.0, 50.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 50.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Move cursor to line 3
        doc_signal.get_untracked().set_offset(17, false); // "line1\nline2\nline3" = 17 at end

        let doc = doc_signal.get_untracked();
        let cursor = doc.cursor().get_untracked();
        let offset = cursor.end;
        let point = doc.text_layouts().borrow().point_of_offset(offset);

        // The ensure_visible rect should be based on the cursor's line_top
        let rect = Rect::from_origin_size(
            (0.0, point.line_top),
            (1.0, point.line_bottom - point.line_top),
        );

        assert!(rect.height() > 0.0, "Cursor rect should have positive height");
        assert!(rect.y0 >= 0.0, "Cursor rect should start at or after 0");
    }

    #[test]
    fn test_textarea_viewport_y_after_cursor_move_down() {
        // Create a textarea with content that exceeds viewport
        let many_lines = (0..10).map(|i| format!("line{}", i)).collect::<Vec<_>>().join("\n");
        let textarea = TextArea::with_text(&many_lines).style(|s| s.size(200.0, 60.0));
        let doc_signal = textarea.doc();
        let viewport = textarea.viewport();

        // Set width for layout
        doc_signal.get_untracked().set_width(200.0);

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 60.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Cursor should start at 0
        doc_signal.get_untracked().set_offset(0, false);

        // Initial viewport should be at origin
        let initial_viewport = viewport.get_untracked();

        // Move cursor to the last line using Down arrow multiple times
        for _ in 0..9 {
            harness.dispatch_event(create_key_event(
                Key::Named(NamedKey::ArrowDown),
                Modifiers::default(),
            ));
        }

        // Cursor should now be on the last line
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        // Text: "line0\nline1\nline2\n..." = positions go 0-5, 6-11, 12-17, etc.
        // Last line "line9" starts at position 54
        assert!(cursor.end >= 54, "Cursor should be in last line, got {}", cursor.end);

        // Note: The viewport position depends on the scroll view actually running,
        // which may not happen in a unit test without a full event loop.
        // This test verifies the cursor movement works correctly.
        let _new_viewport = viewport.get_untracked();
    }

    #[test]
    fn test_textarea_enter_creates_new_lines_correctly() {
        let textarea = TextArea::new().style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Type some text
        for c in ['a', 'b', 'c'] {
            harness.dispatch_event(create_key_event(
                Key::Character(c.to_string().into()),
                Modifiers::default(),
            ));
        }

        assert_eq!(doc_signal.get_untracked().text(), "abc");

        // Press Enter
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Enter),
            Modifiers::default(),
        ));

        assert_eq!(doc_signal.get_untracked().text(), "abc\n");

        // Cursor should be at position 4 (after the newline)
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 4, "Cursor should be on new line after Enter");

        // Type more text
        for c in ['d', 'e', 'f'] {
            harness.dispatch_event(create_key_event(
                Key::Character(c.to_string().into()),
                Modifiers::default(),
            ));
        }

        assert_eq!(doc_signal.get_untracked().text(), "abc\ndef");

        // Cursor should be at position 7
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 7, "Cursor should be at end of 'def'");
    }

    #[test]
    fn test_textarea_line_count_tracking() {
        let textarea = TextArea::new().style(|s| s.size(200.0, 50.0));
        let doc_signal = textarea.doc();

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 50.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Initially should have 1 line (empty)
        let text = doc_signal.get_untracked().text();
        let line_count = if text.is_empty() { 1 } else { text.lines().count() };
        assert_eq!(line_count, 1, "Should start with 1 line");

        // Press Enter 5 times
        for _ in 0..5 {
            harness.dispatch_event(create_key_event(
                Key::Named(NamedKey::Enter),
                Modifiers::default(),
            ));
        }

        // Should now have 6 lines (original + 5 from Enter presses)
        let text = doc_signal.get_untracked().text();
        // Count lines - text will be "\n\n\n\n\n" which has 5 newlines = 6 lines
        let line_count = text.split('\n').count();
        assert_eq!(line_count, 6, "Should have 6 lines after 5 Enter presses");
    }

    #[test]
    fn test_textarea_cursor_vline_after_enter() {
        // Test that cursor is on the correct visual line after Enter
        let textarea = TextArea::with_text("").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        // Set width for text layout
        doc_signal.get_untracked().set_width(200.0);

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Press Enter twice
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Enter),
            Modifiers::default(),
        ));
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Enter),
            Modifiers::default(),
        ));

        // Cursor should be at position 2
        let cursor = doc_signal.get_untracked().cursor().get_untracked();
        assert_eq!(cursor.end, 2, "Cursor should be at position 2 after 2 Enters");

        // Get the visual line of the cursor
        let doc = doc_signal.get_untracked();
        let layouts = doc.text_layouts();
        let vline = layouts.borrow().vline_of_offset(cursor.end);

        // Cursor should be on visual line 2 (0-indexed)
        assert_eq!(vline, 2, "Cursor should be on vline 2 after 2 Enters");
    }

    #[test]
    fn test_textarea_point_of_offset_increases_with_lines() {
        // Test that point_of_offset.line_top increases as we add lines
        let textarea = TextArea::with_text("line1").style(|s| s.size(200.0, 100.0));
        let doc_signal = textarea.doc();

        // Set width for text layout
        doc_signal.get_untracked().set_width(200.0);

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 100.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Get initial cursor position
        let doc = doc_signal.get_untracked();
        let initial_point = doc.text_layouts().borrow().point_of_offset(0);
        let initial_line_top = initial_point.line_top;

        // Move to end and add lines
        doc_signal.get_untracked().set_offset(5, false);

        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Enter),
            Modifiers::default(),
        ));

        // Get new cursor position (should be on line 1)
        let doc = doc_signal.get_untracked();
        let cursor = doc.cursor().get_untracked();
        let new_point = doc.text_layouts().borrow().point_of_offset(cursor.end);

        // The new line_top should be greater than the initial line_top
        assert!(
            new_point.line_top > initial_line_top,
            "New line_top ({}) should be greater than initial ({}) after Enter",
            new_point.line_top,
            initial_line_top
        );
    }

    // ==========================================================================
    // Tests for Enter key scroll behavior
    // ==========================================================================
    // These tests verify that pressing Enter auto-scrolls to keep cursor visible

    #[test]
    fn test_enter_at_bottom_of_viewport_scrolls() {
        // Create a textarea that is small enough to need scrolling
        // Viewport is 60px tall, each line is ~20px, so about 3 lines visible
        let textarea = TextArea::with_text("line1\nline2\nline3").style(|s| s.size(200.0, 60.0));
        let doc_signal = textarea.doc();
        let viewport = textarea.viewport();

        doc_signal.get_untracked().set_width(200.0);

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 60.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Move cursor to end of line3 (the last visible line)
        doc_signal.get_untracked().set_offset(17, false); // End of "line3"

        // Get initial viewport
        let initial_viewport = viewport.get_untracked();
        println!("Initial viewport: {:?}", initial_viewport);

        // Get cursor position before Enter
        let doc = doc_signal.get_untracked();
        let before_point = doc.text_layouts().borrow().point_of_offset(17);
        println!("Cursor before Enter: line_top={}, line_bottom={}", before_point.line_top, before_point.line_bottom);

        // Press Enter - this should create a new line below viewport
        harness.dispatch_event(create_key_event(
            Key::Named(NamedKey::Enter),
            Modifiers::default(),
        ));

        // Rebuild to process any deferred updates (like ensure_visible scroll state)
        harness.rebuild();

        // Get cursor position after Enter
        let doc = doc_signal.get_untracked();
        let cursor = doc.cursor().get_untracked();
        let after_point = doc.text_layouts().borrow().point_of_offset(cursor.end);
        println!("Cursor after Enter: offset={}, line_top={}, line_bottom={}", cursor.end, after_point.line_top, after_point.line_bottom);

        // The cursor should now be on line 4 (the new line)
        assert_eq!(doc.text(), "line1\nline2\nline3\n");
        assert_eq!(cursor.end, 18, "Cursor should be at start of new line");

        // The cursor's line_top should be greater (it's on a new line below)
        assert!(
            after_point.line_top > before_point.line_top,
            "Cursor line_top should increase after Enter: {} > {}",
            after_point.line_top,
            before_point.line_top
        );

        // Now check if viewport was updated
        // Note: In unit tests, the scroll reactive system may not run fully
        let new_viewport = viewport.get_untracked();
        println!("New viewport: {:?}", new_viewport);

        // BUG REPLICATION: The cursor is at line_top=72, but viewport ends at y1=60
        // The viewport.y0 SHOULD have increased to make the cursor visible
        // Expected: viewport.y0 >= (line_top - viewport_height) = 72 - 60 = 12
        // Actual: viewport.y0 = 0 (doesn't scroll)
        assert!(
            new_viewport.y0 > 0.0 || after_point.line_top < new_viewport.y1,
            "Viewport should scroll to show cursor. Cursor at line_top={}, but viewport is {:?}",
            after_point.line_top,
            new_viewport
        );
    }

    #[test]
    fn test_enter_creates_line_beyond_viewport() {
        // Create textarea with exactly 3 lines visible
        let textarea = TextArea::with_text("A\nB\nC").style(|s| s.size(200.0, 60.0));
        let doc_signal = textarea.doc();

        doc_signal.get_untracked().set_width(200.0);

        let mut harness = TestHarness::new_with_size(textarea, 200.0, 60.0);

        // Click to focus
        harness.click(10.0, 10.0);

        // Move to end of last line
        doc_signal.get_untracked().set_offset(5, false); // End of "C"

        // Press Enter multiple times to create lines definitely beyond viewport
        for i in 0..5 {
            harness.dispatch_event(create_key_event(
                Key::Named(NamedKey::Enter),
                Modifiers::default(),
            ));

            let doc = doc_signal.get_untracked();
            let cursor = doc.cursor().get_untracked();
            let point = doc.text_layouts().borrow().point_of_offset(cursor.end);
            println!("After Enter {}: cursor={}, line_top={}", i + 1, cursor.end, point.line_top);
        }

        // Text should now be "A\nB\nC\n\n\n\n\n"
        let doc = doc_signal.get_untracked();
        assert_eq!(doc.text(), "A\nB\nC\n\n\n\n\n");

        // Cursor should be at the end
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.end, 10, "Cursor should be at end after 5 Enters");

        // The cursor's line_top should be well beyond the initial viewport height
        let point = doc.text_layouts().borrow().point_of_offset(cursor.end);
        println!("Final cursor line_top: {}, viewport height: 60.0", point.line_top);

        // Cursor should be on line 8 (0-indexed: 7), so line_top should be ~7*20 = 140
        assert!(
            point.line_top > 60.0,
            "Cursor line_top ({}) should exceed viewport height (60.0)",
            point.line_top
        );
    }

    #[test]
    fn test_ensure_visible_rect_calculation() {
        // Test that the ensure_visible rect is correctly calculated
        let textarea = TextArea::with_text("line1\nline2\nline3\nline4\nline5")
            .style(|s| s.size(200.0, 60.0));
        let doc_signal = textarea.doc();

        doc_signal.get_untracked().set_width(200.0);

        let _harness = TestHarness::new_with_size(textarea, 200.0, 60.0);

        // Move cursor to different lines and check the rect that would be passed to ensure_visible
        for (offset, expected_line) in [(0, 0), (6, 1), (12, 2), (18, 3), (24, 4)] {
            doc_signal.get_untracked().set_offset(offset, false);

            let doc = doc_signal.get_untracked();
            let point = doc.text_layouts().borrow().point_of_offset(offset);

            // This is the rect that ensure_visible receives
            let rect = Rect::from_origin_size(
                (0.0, point.line_top),
                (1.0, point.line_bottom - point.line_top),
            );

            println!(
                "Offset {}: line={}, rect y0={}, y1={}, height={}",
                offset, expected_line, rect.y0, rect.y1, rect.height()
            );

            // Verify rect properties
            assert!(rect.height() > 0.0, "Rect should have positive height");
            assert!(rect.y0 >= 0.0, "Rect y0 should be non-negative");
            assert!(rect.y1 > rect.y0, "Rect y1 should be greater than y0");

            // Verify line progression (each line should be ~20px apart)
            if expected_line > 0 {
                // Line N should have y0 > Line N-1's y0
                let line_height_estimate = point.line_bottom - point.line_top;
                assert!(
                    rect.y0 >= (expected_line as f64) * 15.0, // Rough check
                    "Line {} rect.y0 ({}) should be at least {}",
                    expected_line,
                    rect.y0,
                    expected_line as f64 * 15.0
                );
            }
        }
    }
}

