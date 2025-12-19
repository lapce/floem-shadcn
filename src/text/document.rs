use std::{cell::RefCell, rc::Rc};

use floem::{
    kurbo::Point,
    reactive::{RwSignal, SignalGet, SignalUpdate, SignalWith},
    text::{Attrs, AttrsList, LineHeightValue, TextLayout},
};
use floem_editor_core::{
    buffer::{
        rope_text::{RopeText, RopeTextRef},
        Buffer, InvalLines,
    },
    command::{EditCommand, MoveCommand},
    cursor::{ColPosition, CursorAffinity},
    editor::EditType,
    mode::Mode,
    selection::{InsertDrift, SelRegion, Selection},
};
use lapce_xi_rope::{Delta, Rope, RopeDelta};
use ui_events::pointer::{PointerButton, PointerButtonEvent, PointerState, PointerUpdate};

use super::TextLayoutLines;

/// A document model for text editing with visual line support.
///
/// `Document` provides:
/// - Rope-based text buffer storage
/// - Cursor and selection management
/// - Text layout synchronization with visual lines
/// - Edit commands (insert, delete, newline)
/// - Movement commands (left, right, up, down with visual line support)
/// - Mouse click handling (single, double, triple click)
#[derive(Clone)]
pub struct Document {
    buffer: RwSignal<Buffer>,
    text_layouts: Rc<RefCell<TextLayoutLines>>,
    width: RwSignal<f64>,
    active: RwSignal<bool>,
    cursor: RwSignal<SelRegion>,
    horiz: RwSignal<Option<ColPosition>>,
    on_update: Rc<RefCell<Vec<Box<dyn Fn(&str)>>>>,
}

impl Document {
    /// Creates a new document with the given initial text.
    pub fn new(text: impl Into<Rope>) -> Self {
        let text = text.into();
        let buffer = RwSignal::new(Buffer::new(text));
        let cursor = RwSignal::new(SelRegion::caret(0, CursorAffinity::Forward));
        let width = RwSignal::new(10.0);
        let horiz = RwSignal::new(None);
        let active = RwSignal::new(false);

        Self {
            buffer,
            text_layouts: Rc::new(RefCell::new(TextLayoutLines::builder().build())),
            cursor,
            active,
            width,
            horiz,
            on_update: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Returns the current text content as a string.
    pub fn text(&self) -> String {
        self.buffer
            .with_untracked(|b| b.text().slice_to_cow(..).into_owned())
    }

    /// Returns the buffer signal for reactive access.
    pub fn buffer(&self) -> RwSignal<Buffer> {
        self.buffer
    }

    /// Returns the cursor signal for reactive access.
    pub fn cursor(&self) -> RwSignal<SelRegion> {
        self.cursor
    }

    /// Returns the width signal.
    pub fn width(&self) -> RwSignal<f64> {
        self.width
    }

    /// Returns a reference to the text layouts.
    pub fn text_layouts(&self) -> &Rc<RefCell<TextLayoutLines>> {
        &self.text_layouts
    }

    /// Registers a callback to be called when the document is updated.
    pub fn on_update(&self, f: impl Fn(&str) + 'static) {
        self.on_update.borrow_mut().push(Box::new(f));
    }

    /// Sets the layout width and rebuilds text layouts.
    pub fn set_width(&self, width: f64) {
        if self.width.get_untracked() == width {
            return;
        }

        self.width.set(width);
        let mut builder = TextLayoutLines::builder();
        self.buffer.with_untracked(|buffer| {
            for line in buffer.text().lines_raw(0..buffer.text().len()) {
                let mut text_layout = TextLayout::new_with_text(
                    &line,
                    AttrsList::new(Attrs::default().line_height(LineHeightValue::Normal(1.5))),
                    None,
                );
                text_layout.set_size(width as f32, f32::MAX);
                builder.push_text_layout(&text_layout);
            }
        });
        *self.text_layouts.borrow_mut() = builder.build();
    }

    /// Inserts text at the current cursor position.
    pub fn insert_text(&self, text: &str) {
        self.edit([(self.cursor.get_untracked(), text)], EditType::InsertChars);
    }

    /// Performs an edit operation with the given edits.
    pub fn edit<'a, I>(&self, edits: I, edit_type: EditType)
    where
        I: IntoIterator<Item = (SelRegion, &'a str)>,
    {
        let edits = edits.into_iter().map(|(region, s)| {
            (
                Selection::region(region.start, region.end, CursorAffinity::Forward),
                s,
            )
        });
        let delta = self
            .buffer
            .try_update(|b| b.edit(edits, edit_type))
            .unwrap();
        self.apply_delta(&delta);
    }

    /// Applies a delta to the document, updating layouts and cursor.
    fn apply_delta(&self, delta: &(Rope, RopeDelta, InvalLines)) {
        let width = self.width.get_untracked();

        let (rope, rope_delta, inval_lines) = delta;
        {
            let mut text_layouts = self.text_layouts.borrow_mut();

            let mut builder = TextLayoutLines::builder();
            self.buffer.with_untracked(|buffer| {
                let start = buffer.offset_of_line(inval_lines.start_line);
                let end = buffer.offset_of_line(inval_lines.start_line + inval_lines.new_count);
                for line in buffer.text().lines_raw(start..end) {
                    let mut text_layout = TextLayout::new_with_text(
                        &line,
                        AttrsList::new(Attrs::default().line_height(LineHeightValue::Normal(1.5))),
                        None,
                    );
                    text_layout.set_size(width as f32, f32::MAX);
                    builder.push_text_layout(&text_layout);
                }
            });

            let new = builder.build();
            let rope_ref = RopeTextRef::new(rope);
            let start = rope_ref.offset_of_line(inval_lines.start_line);
            let end = rope_ref.offset_of_line(inval_lines.start_line + inval_lines.inval_count);
            let lines_delta = Delta::simple_edit(start..end, new.0, rope_ref.len());
            text_layouts.apply_delta(lines_delta);
        }

        self.cursor.update(|c| {
            // Wrap in Selection to use apply_delta, then extract first region
            let selection = Selection::region(c.start, c.end, c.affinity);
            let new_selection = selection.apply_delta(rope_delta, true, InsertDrift::Default);
            if let Some(region) = new_selection.regions().first() {
                *c = *region;
            }
        });

        self.buffer.with_untracked(|buffer| {
            let text = &buffer.text().slice_to_cow(..);
            for on_update in self.on_update.borrow().iter() {
                on_update(text);
            }
        });
    }

    /// Runs a movement command.
    pub fn run_move_command(&self, command: &MoveCommand) {
        match command {
            MoveCommand::Left => {
                let region = self.cursor.get_untracked();
                let region = if region.is_caret() {
                    let new_offset = self
                        .buffer
                        .with_untracked(|b| b.move_left(region.start, Mode::Insert, 1));
                    SelRegion::caret(new_offset, CursorAffinity::Forward)
                } else {
                    SelRegion::caret(region.min(), CursorAffinity::Forward)
                };
                self.cursor.set(region);
                self.horiz.set(None);
            }
            MoveCommand::Right => {
                let region = self.cursor.get_untracked();
                let region = if region.is_caret() {
                    let new_offset = self
                        .buffer
                        .with_untracked(|b| b.move_right(region.start, Mode::Insert, 1));
                    SelRegion::caret(new_offset, CursorAffinity::Forward)
                } else {
                    SelRegion::caret(region.max(), CursorAffinity::Forward)
                };
                self.cursor.set(region);
                self.horiz.set(None);
            }
            MoveCommand::Up => {
                let region = self.cursor.get_untracked();
                let offset = if region.is_caret() {
                    region.start
                } else {
                    region.min()
                };
                let lines = self.text_layouts.borrow();
                let vline = lines.vline_of_offset(offset);
                let horiz = if let Some(horiz) = self.horiz.get_untracked() {
                    horiz
                } else {
                    let point = lines.point_of_offset(offset);
                    self.horiz.set(Some(ColPosition::Col(point.x)));
                    ColPosition::Col(point.x)
                };
                let new_offset = if vline == 0 {
                    0
                } else {
                    let vline = vline - 1;
                    match horiz {
                        ColPosition::FirstNonBlank => 0, // TODO: implement
                        ColPosition::Start => lines.offset_of_vline(vline),
                        ColPosition::End => {
                            let next_vline_offset = lines.offset_of_vline(vline + 1);
                            if next_vline_offset > 0 {
                                next_vline_offset - 1
                            } else {
                                0
                            }
                        }
                        ColPosition::Col(x) => {
                            let offset = lines.offset_of_vline(vline);
                            let point = lines.point_of_offset(offset);
                            lines.offset_of_point(Point::new(x, point.glyph_top))
                        }
                    }
                };
                self.cursor
                    .set(SelRegion::caret(new_offset, CursorAffinity::Forward));
            }
            MoveCommand::Down => {
                let region = self.cursor.get_untracked();
                let offset = if region.is_caret() {
                    region.start
                } else {
                    region.max()
                };
                let lines = self.text_layouts.borrow();
                let vline = lines.vline_of_offset(offset);
                let horiz = if let Some(horiz) = self.horiz.get_untracked() {
                    horiz
                } else {
                    let point = lines.point_of_offset(offset);
                    self.horiz.set(Some(ColPosition::Col(point.x)));
                    ColPosition::Col(point.x)
                };

                let last_vline = lines.vline_of_offset(lines.utf8_len());
                let new_offset = if last_vline == vline {
                    lines.utf8_len()
                } else {
                    let vline = vline + 1;
                    match horiz {
                        ColPosition::FirstNonBlank => lines.offset_of_vline(vline), // TODO: implement
                        ColPosition::Start => lines.offset_of_vline(vline),
                        ColPosition::End => {
                            let next_vline_offset = lines.offset_of_vline(vline + 1);
                            if next_vline_offset > 0 {
                                next_vline_offset - 1
                            } else {
                                lines.utf8_len()
                            }
                        }
                        ColPosition::Col(x) => {
                            let offset = lines.offset_of_vline(vline);
                            let point = lines.point_of_offset(offset);
                            lines.offset_of_point(Point::new(x, point.glyph_top))
                        }
                    }
                };

                self.cursor
                    .set(SelRegion::caret(new_offset, CursorAffinity::Forward));
            }
            _ => {}
        }
    }

    /// Runs an edit command.
    pub fn run_edit_command(&self, command: &EditCommand) {
        match command {
            EditCommand::InsertNewLine => {
                self.edit(
                    [(self.cursor.get_untracked(), "\n")],
                    EditType::InsertNewline,
                );
            }
            EditCommand::DeleteBackward => {
                let region = self.cursor.get_untracked();
                let region = if region.is_caret() {
                    let new_offset = self
                        .buffer
                        .with_untracked(|b| b.move_left(region.start, Mode::Insert, 1));
                    SelRegion::new(region.start, new_offset, CursorAffinity::Forward, None)
                } else {
                    region
                };
                self.edit([(region, "")], EditType::Delete);
            }
            _ => {}
        }
    }

    /// Sets the cursor offset, optionally extending the selection.
    pub fn set_offset(&self, offset: usize, modify: bool) {
        let region = self.cursor.get_untracked();
        let region = if modify {
            SelRegion::new(region.start, offset, CursorAffinity::Forward, None)
        } else {
            SelRegion::caret(offset, CursorAffinity::Forward)
        };
        self.cursor.set(region);
    }

    /// Handles pointer down events.
    pub fn pointer_down(&self, event: &PointerButtonEvent) {
        if event.button == Some(PointerButton::Primary) {
            self.active.set(true);
            self.left_click(&event.state);
        } else if event.button == Some(PointerButton::Secondary) {
            self.double_click(&event.state);
        }
    }

    fn left_click(&self, state: &PointerState) {
        match state.count {
            1 => {
                self.single_click(state);
            }
            2 => {
                self.double_click(state);
            }
            3 => {
                self.triple_click(state);
            }
            _ => {}
        }
    }

    fn single_click(&self, state: &PointerState) {
        let lines = self.text_layouts.borrow();
        let pos = state.logical_point();
        let new_offset = lines.offset_of_point(pos);
        let shift = state.modifiers.shift();
        self.set_offset(new_offset, shift);
        self.horiz.set(None);
    }

    fn double_click(&self, state: &PointerState) {
        let lines = self.text_layouts.borrow();
        let pos = state.logical_point();
        let mouse_offset = lines.offset_of_point(pos);
        let (start, end) = self.buffer.with_untracked(|b| b.select_word(mouse_offset));

        self.cursor
            .set(SelRegion::new(start, end, CursorAffinity::Forward, None));
        self.horiz.set(None);
    }

    fn triple_click(&self, state: &PointerState) {
        let lines = self.text_layouts.borrow();
        let pos = state.logical_point();
        let mouse_offset = lines.offset_of_point(pos);

        let vline = lines.vline_of_offset(mouse_offset);
        let start = lines.offset_of_vline(vline);
        let end = lines.offset_of_vline(vline + 1);

        self.cursor
            .set(SelRegion::new(start, end, CursorAffinity::Forward, None));
        self.horiz.set(None);
    }

    /// Handles pointer move events (for drag selection).
    pub fn pointer_move(&self, event: &PointerUpdate) {
        if self.active.get_untracked() {
            let lines = self.text_layouts.borrow();
            let pos = event.current.logical_point();
            let offset = lines.offset_of_point(pos);
            let cursor = self.cursor.get_untracked();
            if cursor.end != offset {
                self.cursor.set(SelRegion::new(
                    cursor.start,
                    offset,
                    CursorAffinity::Forward,
                    None,
                ));
            }
        }
    }

    /// Handles pointer up events.
    pub fn pointer_up(&self, _event: &PointerButtonEvent) {
        self.active.set(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use floem::reactive::SignalGet;

    #[test]
    fn test_new_empty_document() {
        let doc = Document::new("");
        assert_eq!(doc.text(), "");
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 0);
        assert_eq!(cursor.end, 0);
        assert!(cursor.is_caret());
    }

    #[test]
    fn test_new_document_with_text() {
        let doc = Document::new("hello world");
        assert_eq!(doc.text(), "hello world");
        // Cursor should be at start
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 0);
    }

    #[test]
    fn test_insert_text_at_start() {
        let doc = Document::new("");
        doc.insert_text("hello");
        assert_eq!(doc.text(), "hello");
        // Cursor should be at end of inserted text
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 5);
        assert_eq!(cursor.end, 5);
    }

    #[test]
    fn test_insert_text_multiple() {
        let doc = Document::new("");
        doc.insert_text("hello");
        doc.insert_text(" ");
        doc.insert_text("world");
        assert_eq!(doc.text(), "hello world");
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 11);
    }

    #[test]
    fn test_insert_newline() {
        let doc = Document::new("");
        doc.insert_text("line1");
        doc.run_edit_command(&EditCommand::InsertNewLine);
        doc.insert_text("line2");
        assert_eq!(doc.text(), "line1\nline2");
    }

    #[test]
    fn test_delete_backward_single_char() {
        let doc = Document::new("");
        doc.insert_text("hello");
        doc.run_edit_command(&EditCommand::DeleteBackward);
        assert_eq!(doc.text(), "hell");
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 4);
    }

    #[test]
    fn test_delete_backward_at_start() {
        let doc = Document::new("hello");
        // Cursor is at 0, delete backward should do nothing
        doc.run_edit_command(&EditCommand::DeleteBackward);
        assert_eq!(doc.text(), "hello");
    }

    #[test]
    fn test_delete_backward_multiple() {
        let doc = Document::new("");
        doc.insert_text("hello");
        doc.run_edit_command(&EditCommand::DeleteBackward);
        doc.run_edit_command(&EditCommand::DeleteBackward);
        doc.run_edit_command(&EditCommand::DeleteBackward);
        assert_eq!(doc.text(), "he");
    }

    #[test]
    fn test_move_left() {
        let doc = Document::new("");
        doc.insert_text("hello");
        // Cursor at 5
        doc.run_move_command(&MoveCommand::Left);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 4);
        assert!(cursor.is_caret());
    }

    #[test]
    fn test_move_left_at_start() {
        let doc = Document::new("hello");
        // Cursor at 0
        doc.run_move_command(&MoveCommand::Left);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 0);
    }

    #[test]
    fn test_move_right() {
        let doc = Document::new("hello");
        // Cursor at 0
        doc.run_move_command(&MoveCommand::Right);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 1);
    }

    #[test]
    fn test_move_right_at_end() {
        let doc = Document::new("");
        doc.insert_text("hello");
        // Cursor at 5 (end)
        doc.run_move_command(&MoveCommand::Right);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 5); // Should stay at end
    }

    #[test]
    fn test_move_left_collapses_selection() {
        let doc = Document::new("hello");
        // Create a selection from 1 to 4
        doc.set_offset(1, false);
        doc.set_offset(4, true);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 1);
        assert_eq!(cursor.end, 4);
        assert!(!cursor.is_caret());

        // Move left should collapse to min
        doc.run_move_command(&MoveCommand::Left);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 1);
        assert!(cursor.is_caret());
    }

    #[test]
    fn test_move_right_collapses_selection() {
        let doc = Document::new("hello");
        // Create a selection from 1 to 4
        doc.set_offset(1, false);
        doc.set_offset(4, true);

        // Move right should collapse to max
        doc.run_move_command(&MoveCommand::Right);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 4);
        assert!(cursor.is_caret());
    }

    #[test]
    fn test_set_offset_without_modify() {
        let doc = Document::new("hello");
        doc.set_offset(3, false);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 3);
        assert_eq!(cursor.end, 3);
        assert!(cursor.is_caret());
    }

    #[test]
    fn test_set_offset_with_modify() {
        let doc = Document::new("hello");
        doc.set_offset(1, false);
        doc.set_offset(4, true);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 1);
        assert_eq!(cursor.end, 4);
        assert!(!cursor.is_caret());
    }

    #[test]
    fn test_delete_selection() {
        let doc = Document::new("hello world");
        // Select "llo w"
        doc.set_offset(2, false);
        doc.set_offset(7, true);
        doc.run_edit_command(&EditCommand::DeleteBackward);
        assert_eq!(doc.text(), "heorld");
    }

    #[test]
    fn test_insert_replaces_selection() {
        let doc = Document::new("hello world");
        // Select "world"
        doc.set_offset(6, false);
        doc.set_offset(11, true);
        doc.insert_text("there");
        assert_eq!(doc.text(), "hello there");
    }

    #[test]
    fn test_on_update_callback() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let updates = Rc::new(RefCell::new(Vec::new()));
        let updates_clone = updates.clone();

        let doc = Document::new("");
        doc.on_update(move |text| {
            updates_clone.borrow_mut().push(text.to_string());
        });

        doc.insert_text("a");
        doc.insert_text("b");
        doc.insert_text("c");

        let updates = updates.borrow();
        assert_eq!(updates.len(), 3);
        assert_eq!(updates[0], "a");
        assert_eq!(updates[1], "ab");
        assert_eq!(updates[2], "abc");
    }

    #[test]
    fn test_unicode_insert() {
        let doc = Document::new("");
        doc.insert_text("こんにちは");
        assert_eq!(doc.text(), "こんにちは");
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 15); // 5 chars * 3 bytes each
    }

    #[test]
    fn test_unicode_delete() {
        let doc = Document::new("");
        doc.insert_text("日本語");
        doc.run_edit_command(&EditCommand::DeleteBackward);
        assert_eq!(doc.text(), "日本");
    }

    #[test]
    fn test_emoji_insert() {
        let doc = Document::new("");
        doc.insert_text("👋🌍");
        assert_eq!(doc.text(), "👋🌍");
    }

    // Tests for up/down movement
    #[test]
    fn test_move_up_from_first_line() {
        let doc = Document::new("hello");
        doc.set_width(200.0);
        doc.set_offset(2, false); // Middle of line

        doc.run_move_command(&MoveCommand::Up);

        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.end, 0, "Up on first line should go to start");
    }

    #[test]
    fn test_move_down_from_last_line() {
        let doc = Document::new("hello");
        doc.set_width(200.0);
        doc.set_offset(2, false); // Middle of line

        doc.run_move_command(&MoveCommand::Down);

        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.end, 5, "Down on last line should go to end");
    }

    #[test]
    fn test_move_down_up_multiline() {
        let doc = Document::new("line1\nline2\nline3");
        doc.set_width(200.0);
        doc.set_offset(0, false);

        // Move to line2
        doc.run_move_command(&MoveCommand::Down);
        let cursor = doc.cursor().get_untracked();
        // Should be in line2 (offsets 6-11)
        assert!(
            cursor.end >= 6 && cursor.end <= 11,
            "After Down from line1, cursor should be in line2, got {}",
            cursor.end
        );
        let line2_pos = cursor.end;

        // Move to line3
        doc.run_move_command(&MoveCommand::Down);
        let cursor = doc.cursor().get_untracked();
        // Should be in line3 (offsets 12-17)
        assert!(
            cursor.end >= 12 && cursor.end <= 17,
            "After Down from line2, cursor should be in line3, got {}",
            cursor.end
        );

        // Move back up to line2
        doc.run_move_command(&MoveCommand::Up);
        let cursor = doc.cursor().get_untracked();
        // Should be back in line2
        assert!(
            cursor.end >= 6 && cursor.end <= 11,
            "After Up from line3, cursor should be in line2, got {} (was at {})",
            cursor.end,
            line2_pos
        );
    }

    #[test]
    fn test_move_down_through_empty_line() {
        // "a\n\nb" - positions: 0='a', 1='\n', 2='\n', 3='b'
        let doc = Document::new("a\n\nb");
        doc.set_width(200.0);
        doc.set_offset(0, false); // Start at 'a'

        // Move down - should go to empty line (position 2)
        doc.run_move_command(&MoveCommand::Down);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.end, 2, "After Down from 'a', cursor should be at position 2 (empty line)");

        // Move down again - should go to 'b' (position 3)
        doc.run_move_command(&MoveCommand::Down);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.end, 3, "After Down from empty line, cursor should be at position 3 ('b')");
    }
}
