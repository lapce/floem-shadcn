//! TextInput view component
//!
//! A single-line text input with horizontal scrolling, cursor/selection rendering,
//! and keyboard/mouse handling. Reuses Document for text editing logic.

use std::time::{Duration, Instant};

use floem::{
    Renderer, View, ViewId,
    action::exec_after,
    context::{ComputeLayoutCx, PaintCx},
    event::{Event, EventListener, EventPropagation},
    kurbo::{Point, Rect, Size},
    reactive::{Effect, RwSignal, SignalGet, SignalUpdate, SignalWith},
    style::{CursorStyle as StyleCursorStyle, Style},
};

use crate::theme::ShadcnThemeProp;
use floem_editor_core::buffer::rope_text::RopeText;
use ui_events::{
    keyboard::{Key, KeyState, KeyboardEvent, Modifiers, NamedKey},
    pointer::PointerEvent,
};

use super::{
    CURSOR_BLINK_INTERVAL_MS, Command, Document, Keymap, apply_styles_to_document, extract_padding,
    extract_text_styles, get_glyph_dimensions, is_cursor_visible,
};

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
        let padding = RwSignal::new((0.0, 0.0, 0.0, 0.0));
        let scroll_offset = RwSignal::new(0.0);
        let visible_width = RwSignal::new(0.0);
        let doc = Document::new(text.into());
        let doc_signal = RwSignal::new(doc);
        let last_cursor_action = RwSignal::new(Instant::now());
        let placeholder = RwSignal::new(None);
        let on_enter: RwSignal<Option<Box<dyn Fn(&str)>>> = RwSignal::new(None);

        // Capture cursor signal for reactive tracking
        let cursor_signal = doc_signal.get_untracked().cursor();

        // Effect to ensure cursor is visible (horizontal scrolling)
        Effect::new(move |_| {
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
        let keymap = std::sync::Arc::new(Keymap::single_line());
        let keymap_clone = keymap.clone();

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

                // Try to find command
                let command = keymap_clone.get(key, modifiers);

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
                            id.request_paint();
                        }
                        Command::SelectAll => {
                            document.select_all();
                            id.request_paint();
                        }
                        Command::Copy => {
                            document.copy();
                        }
                        Command::Cut => {
                            if document.cut() {
                                id.request_layout();
                            }
                        }
                        Command::Paste => {
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
                let mut mods = *modifiers;
                mods.set(Modifiers::SHIFT, false);
                #[cfg(target_os = "macos")]
                mods.set(Modifiers::ALT, false);

                if mods.is_empty() {
                    if let Key::Character(c) = key {
                        // Filter out newlines for single-line input
                        let filtered: String =
                            c.chars().filter(|&ch| ch != '\n' && ch != '\r').collect();
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
                    let filtered: String = text
                        .chars()
                        .filter(|&ch| ch != '\n' && ch != '\r')
                        .collect();
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
        Effect::new(move |_| {
            let new_value = set_value();
            // Filter out newlines
            let filtered: String = new_value
                .chars()
                .filter(|&ch| ch != '\n' && ch != '\r')
                .collect();
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
        Some(Style::new().cursor(StyleCursorStyle::Text).focusable(true))
    }

    fn compute_layout(&mut self, _cx: &mut ComputeLayoutCx) -> Option<Rect> {
        let layout = self.id.get_layout().unwrap_or_default();
        let style = self.id.get_combined_style();
        let builtin_style = style.builtin();

        let padding = extract_padding(&builtin_style, layout.size.width as f64);
        if padding != self.padding.get_untracked() {
            self.padding.set(padding);
        }

        let width = layout.size.width as f64 - padding.3 - padding.1;
        if width != self.visible_width.get_untracked() {
            self.visible_width.set(width);
        }

        // Get text styling from style and set them on the document
        let text_styles = extract_text_styles(&builtin_style);
        let doc = self.doc.get_untracked();
        apply_styles_to_document(&doc, &text_styles);
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
        let text_styles = extract_text_styles(&style.builtin());
        let theme = style.get(ShadcnThemeProp);
        let selection_color = theme.primary.multiply_alpha(0.2);
        let placeholder_color = text_styles.text_color.multiply_alpha(0.5);

        let layout = self.id.get_layout().unwrap_or_default();
        let height = layout.size.height as f64 - padding.0 - padding.2;

        cx.save();
        cx.clip(&Rect::from_origin_size(
            Point::new(padding.3, padding.0),
            Size::new(visible_width, height),
        ));

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
                    .font_size(text_styles.font_size)
                    .color(placeholder_color)
                    .line_height(text_styles.line_height)
                    .weight(text_styles.font_weight);

                if !text_styles.font_family.is_empty() {
                    placeholder_attrs = placeholder_attrs.family(&text_styles.font_family);
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
                let elapsed_ms = self
                    .last_cursor_action
                    .get_untracked()
                    .elapsed()
                    .as_millis();

                if is_cursor_visible(elapsed_ms) {
                    let p = lines.point_of_offset(cursor.end);
                    let (cursor_top, cursor_height) = get_glyph_dimensions(
                        p.glyph_top,
                        p.glyph_bottom,
                        lines.default_glyph_top(),
                        lines.default_glyph_height(),
                    );
                    let rect = Rect::from_origin_size(
                        (
                            p.x + padding.3 - scroll_offset - 1.0,
                            cursor_top + padding.0 + y_offset,
                        ),
                        (2.0, cursor_height),
                    );
                    cx.fill(&rect, text_styles.text_color, 0.0);
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
                let (sel_top, sel_height) = get_glyph_dimensions(
                    p.glyph_top,
                    p.glyph_bottom,
                    lines.default_glyph_top(),
                    lines.default_glyph_height(),
                );

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
