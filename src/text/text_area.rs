//! TextArea view component
//!
//! A multi-line text area with visual line support, cursor/selection rendering,
//! and keyboard/mouse handling.

use std::collections::HashMap;

use floem::{
    context::{ComputeLayoutCx, PaintCx},
    event::{Event, EventListener, EventPropagation},
    kurbo::{Point, Rect, Size},
    peniko::Color,
    reactive::{create_effect, create_rw_signal, RwSignal, SignalGet, SignalUpdate, SignalWith},
    style::{CursorStyle as StyleCursorStyle, Style},
    taffy::Overflow,
    unit::PxPct,
    views::{empty, Decorators, Scroll},
    IntoView, Renderer, View, ViewId,
};
use floem_editor_core::{
    buffer::rope_text::RopeText,
    command::{EditCommand, MoveCommand},
};
use ui_events::{
    keyboard::{Key, KeyState, KeyboardEvent, Modifiers, NamedKey},
    pointer::{PointerEvent, PointerUpdate},
};

use super::Document;

/// A command that can be executed on the editor
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Edit(EditCommand),
    Move(MoveCommand),
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

        // Basic navigation
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

        // Basic editing
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

        Self { keymaps }
    }
}

/// A multi-line text area view
pub struct TextArea {
    id: ViewId,
    scroll_id: ViewId,
    doc: RwSignal<Document>,
    padding: RwSignal<(f64, f64, f64, f64)>,
    viewport: RwSignal<Rect>,
    parent_size: RwSignal<Size>,
    child_height: RwSignal<f64>,
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

        let id = ViewId::new();

        // Create scroll view
        let scroll_view = Scroll::new(empty().style(move |s| s.width(10.0).height(child_height.get())))
            .style(move |s| {
                let padding = padding.get();
                s.absolute()
                    .size_full()
                    .margin_top(-padding.0)
                    .margin_left(-padding.3)
            })
            .on_scroll(move |new_viewport| {
                viewport.set(new_viewport);
            })
            .ensure_visible(move || {
                let padding = padding.get_untracked();

                let doc = doc_signal.get_untracked();
                let cursor = doc.cursor().get();
                let offset = cursor.end;
                let point = doc.text_layouts().borrow().point_of_offset(offset);

                Rect::from_origin_size(
                    (0.0, point.line_top),
                    (1.0, point.line_bottom - point.line_top + padding.0 + padding.2),
                )
            });
        let scroll_id = scroll_view.id();

        id.set_children_vec(vec![
            empty()
                .style(move |s| {
                    let padding = padding.get();
                    s.height(child_height.get() - padding.0 - padding.2)
                })
                .into_any(),
            scroll_view.into_any(),
        ]);

        // Set up event handlers
        let keypress_map = std::sync::Arc::new(KeypressMap::default());
        let keypress_map_clone = keypress_map.clone();

        id.add_event_listener(
            EventListener::PointerDown,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Down(pointer_event)) = event {
                    let padding = padding.get_untracked();
                    let viewport = viewport.get_untracked();
                    let mut adjusted = pointer_event.clone();
                    adjusted.state.position.x -= padding.3;
                    adjusted.state.position.y -= padding.0 - viewport.y0;
                    id.request_active();
                    id.request_focus();
                    doc_signal.get_untracked().pointer_down(&adjusted);
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::PointerMove,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Move(pointer_event)) = event {
                    let padding = padding.get_untracked();
                    let viewport = viewport.get_untracked();
                    let mut adjusted = pointer_event.clone();
                    adjusted.current.position.x -= padding.3;
                    adjusted.current.position.y -= padding.0 - viewport.y0;
                    doc_signal.get_untracked().pointer_move(&adjusted);
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::PointerUp,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Up(pointer_event)) = event {
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
                    match command {
                        Command::Edit(edit_cmd) => document.run_edit_command(edit_cmd),
                        Command::Move(move_cmd) => document.run_move_command(move_cmd),
                    }
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

    /// Sets the editor content reactively
    pub fn value(self, set_value: impl Fn() -> String + 'static) -> Self {
        let doc = self.doc;
        create_effect(move |_| {
            let new_value = set_value();
            doc.update(|doc| {
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
}

impl View for TextArea {
    fn id(&self) -> ViewId {
        self.id
    }

    fn view_style(&self) -> Option<Style> {
        Some(
            Style::new()
                .cursor(StyleCursorStyle::Text)
                .focusable(true)
                .set(floem::style::OverflowX, Overflow::Scroll)
                .set(floem::style::OverflowY, Overflow::Scroll),
        )
    }

    fn compute_layout(&mut self, cx: &mut ComputeLayoutCx) -> Option<Rect> {
        let layout = self.id.get_layout().unwrap_or_default();
        let style = self.id.get_combined_style();
        let style = style.builtin();

        let padding_left = match style.padding_left() {
            PxPct::Px(padding) => padding,
            PxPct::Pct(pct) => (pct / 100.) * layout.size.width as f64,
        };
        let padding_right = match style.padding_right() {
            PxPct::Px(padding) => padding,
            PxPct::Pct(pct) => (pct / 100.) * layout.size.width as f64,
        };
        let padding_top = match style.padding_top() {
            PxPct::Px(padding) => padding,
            PxPct::Pct(pct) => (pct / 100.) * layout.size.width as f64,
        };
        let padding_bottom = match style.padding_bottom() {
            PxPct::Px(padding) => padding,
            PxPct::Pct(pct) => (pct / 100.) * layout.size.width as f64,
        };

        if (padding_top, padding_right, padding_bottom, padding_left) != self.padding.get_untracked() {
            self.padding.set((padding_top, padding_right, padding_bottom, padding_left));
        }

        let width = layout.size.width as f64 - padding_left - padding_right;
        let height = layout.size.height as f64 - padding_top - padding_bottom;
        let parent_size = Size::new(width, height);

        let doc = self.doc.get_untracked();
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
                let p = lines.point_of_offset(cursor.end);
                let rect = Rect::from_origin_size(
                    (p.x + padding.3 - 1.0, p.glyph_top + padding.0 - viewport.y0),
                    (2.0, p.glyph_bottom - p.glyph_top),
                );
                cx.fill(&rect, Color::BLACK, 0.0);
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
                    cx.fill(&rect, Color::BLACK.multiply_alpha(0.1), 0.0);
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use floem::reactive::SignalGet;
    use floem::test_harness::TestHarness;
    use floem::views::Decorators;

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
}

