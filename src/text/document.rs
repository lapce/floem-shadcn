use std::{cell::RefCell, rc::Rc};

use floem::{
    kurbo::Point,
    peniko::Color,
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
    text_color: RwSignal<Color>,
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
        let text_color = RwSignal::new(Color::BLACK);

        Self {
            buffer,
            text_layouts: Rc::new(RefCell::new(TextLayoutLines::builder().build())),
            cursor,
            active,
            width,
            horiz,
            text_color,
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
        self.rebuild_layouts(width);
    }

    /// Sets the text color and rebuilds layouts.
    pub fn set_text_color(&self, color: Color) {
        if self.text_color.get_untracked() == color {
            return;
        }
        self.text_color.set(color);
        let width = self.width.get_untracked();
        if width > 0.0 {
            self.rebuild_layouts(width);
        }
    }

    /// Rebuilds text layouts with current settings.
    fn rebuild_layouts(&self, width: f64) {
        let text_color = self.text_color.get_untracked();
        let attrs = AttrsList::new(
            Attrs::default()
                .line_height(LineHeightValue::Normal(1.5))
                .color(text_color),
        );
        let mut builder = TextLayoutLines::builder();

        // Create a reference layout with a space to capture default font metrics
        // This ensures cursor height is correct even when text is empty
        let reference_layout = TextLayout::new_with_text(" ", attrs.clone(), None);
        builder.set_default_from_layout(&reference_layout);

        self.buffer.with_untracked(|buffer| {
            for line in buffer.text().lines_raw(0..buffer.text().len()) {
                let mut text_layout = TextLayout::new_with_text(&line, attrs.clone(), None);
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
        let text_color = self.text_color.get_untracked();
        let attrs = AttrsList::new(
            Attrs::default()
                .line_height(LineHeightValue::Normal(1.5))
                .color(text_color),
        );

        let (rope, rope_delta, inval_lines) = delta;
        {
            let mut text_layouts = self.text_layouts.borrow_mut();

            let mut builder = TextLayoutLines::builder();
            self.buffer.with_untracked(|buffer| {
                let start = buffer.offset_of_line(inval_lines.start_line);
                let end = buffer.offset_of_line(inval_lines.start_line + inval_lines.new_count);
                for line in buffer.text().lines_raw(start..end) {
                    let mut text_layout = TextLayout::new_with_text(&line, attrs.clone(), None);
                    text_layout.set_size(width as f32, f32::MAX);
                    builder.push_text_layout(&text_layout);
                }
            });

            let new = builder.build();
            let rope_ref = RopeTextRef::new(rope);
            let start = rope_ref.offset_of_line(inval_lines.start_line);
            let end = rope_ref.offset_of_line(inval_lines.start_line + inval_lines.inval_count);
            let lines_delta = Delta::simple_edit(start..end, new.tree, rope_ref.len());
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
    /// If `modify` is true, extends the selection instead of moving the cursor.
    pub fn run_move_command(&self, command: &MoveCommand, modify: bool) {
        match command {
            MoveCommand::Left => {
                let region = self.cursor.get_untracked();
                let new_offset = if modify || region.is_caret() {
                    self.buffer
                        .with_untracked(|b| b.move_left(region.end, Mode::Insert, 1))
                } else {
                    region.min()
                };
                self.set_offset(new_offset, modify);
                self.horiz.set(None);
            }
            MoveCommand::Right => {
                let region = self.cursor.get_untracked();
                let new_offset = if modify || region.is_caret() {
                    self.buffer
                        .with_untracked(|b| b.move_right(region.end, Mode::Insert, 1))
                } else {
                    region.max()
                };
                self.set_offset(new_offset, modify);
                self.horiz.set(None);
            }
            MoveCommand::Up => {
                let region = self.cursor.get_untracked();
                let offset = region.end;
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
                drop(lines);
                self.set_offset(new_offset, modify);
            }
            MoveCommand::Down => {
                let region = self.cursor.get_untracked();
                let offset = region.end;
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
                drop(lines);
                self.set_offset(new_offset, modify);
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
            self.left_click(&event.state);
        } else if event.button == Some(PointerButton::Secondary) {
            self.double_click(&event.state);
        }
    }

    fn left_click(&self, state: &PointerState) {
        match state.count {
            1 => {
                // Only enable drag selection for single clicks
                self.active.set(true);
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
        doc.run_move_command(&MoveCommand::Left, false);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 4);
        assert!(cursor.is_caret());
    }

    #[test]
    fn test_move_left_at_start() {
        let doc = Document::new("hello");
        // Cursor at 0
        doc.run_move_command(&MoveCommand::Left, false);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 0);
    }

    #[test]
    fn test_move_right() {
        let doc = Document::new("hello");
        // Cursor at 0
        doc.run_move_command(&MoveCommand::Right, false);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 1);
    }

    #[test]
    fn test_move_right_at_end() {
        let doc = Document::new("");
        doc.insert_text("hello");
        // Cursor at 5 (end)
        doc.run_move_command(&MoveCommand::Right, false);
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
        doc.run_move_command(&MoveCommand::Left, false);
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
        doc.run_move_command(&MoveCommand::Right, false);
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

        doc.run_move_command(&MoveCommand::Up, false);

        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.end, 0, "Up on first line should go to start");
    }

    #[test]
    fn test_move_down_from_last_line() {
        let doc = Document::new("hello");
        doc.set_width(200.0);
        doc.set_offset(2, false); // Middle of line

        doc.run_move_command(&MoveCommand::Down, false);

        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.end, 5, "Down on last line should go to end");
    }

    #[test]
    fn test_move_down_up_multiline() {
        let doc = Document::new("line1\nline2\nline3");
        doc.set_width(200.0);
        doc.set_offset(0, false);

        // Move to line2
        doc.run_move_command(&MoveCommand::Down, false);
        let cursor = doc.cursor().get_untracked();
        // Should be in line2 (offsets 6-11)
        assert!(
            cursor.end >= 6 && cursor.end <= 11,
            "After Down from line1, cursor should be in line2, got {}",
            cursor.end
        );
        let line2_pos = cursor.end;

        // Move to line3
        doc.run_move_command(&MoveCommand::Down, false);
        let cursor = doc.cursor().get_untracked();
        // Should be in line3 (offsets 12-17)
        assert!(
            cursor.end >= 12 && cursor.end <= 17,
            "After Down from line2, cursor should be in line3, got {}",
            cursor.end
        );

        // Move back up to line2
        doc.run_move_command(&MoveCommand::Up, false);
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
        doc.run_move_command(&MoveCommand::Down, false);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.end, 2, "After Down from 'a', cursor should be at position 2 (empty line)");

        // Move down again - should go to 'b' (position 3)
        doc.run_move_command(&MoveCommand::Down, false);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.end, 3, "After Down from empty line, cursor should be at position 3 ('b')");
    }

    // ==========================================================================
    // Selection tests
    // ==========================================================================

    #[test]
    fn test_shift_right_extends_selection() {
        let doc = Document::new("hello");
        doc.set_width(100.0);

        // Start at position 0
        let cursor = doc.cursor().get_untracked();
        assert!(cursor.is_caret());
        assert_eq!(cursor.end, 0);

        // Shift+Right should extend selection
        doc.run_move_command(&MoveCommand::Right, true);
        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Selection should not be a caret after Shift+Right");
        assert_eq!(cursor.start, 0, "Selection start should remain at 0");
        assert_eq!(cursor.end, 1, "Selection end should be at 1");

        // Shift+Right again should extend further
        doc.run_move_command(&MoveCommand::Right, true);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 0);
        assert_eq!(cursor.end, 2);
    }

    #[test]
    fn test_shift_left_extends_selection() {
        let doc = Document::new("hello");
        doc.set_width(100.0);

        // Move to position 3
        doc.set_offset(3, false);
        let cursor = doc.cursor().get_untracked();
        assert!(cursor.is_caret());
        assert_eq!(cursor.end, 3);

        // Shift+Left should extend selection backwards
        doc.run_move_command(&MoveCommand::Left, true);
        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret());
        assert_eq!(cursor.start, 3, "Selection start (anchor) should remain at 3");
        assert_eq!(cursor.end, 2, "Selection end should be at 2");

        // Selection min/max
        assert_eq!(cursor.min(), 2);
        assert_eq!(cursor.max(), 3);
    }

    #[test]
    fn test_right_without_shift_collapses_selection() {
        let doc = Document::new("hello");
        doc.set_width(100.0);

        // Create a selection from 1 to 3
        doc.set_offset(1, false);
        doc.set_offset(3, true);
        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret());
        assert_eq!(cursor.min(), 1);
        assert_eq!(cursor.max(), 3);

        // Right without Shift should collapse to max
        doc.run_move_command(&MoveCommand::Right, false);
        let cursor = doc.cursor().get_untracked();
        assert!(cursor.is_caret());
        assert_eq!(cursor.end, 3);
    }

    #[test]
    fn test_left_without_shift_collapses_selection() {
        let doc = Document::new("hello");
        doc.set_width(100.0);

        // Create a selection from 1 to 3
        doc.set_offset(1, false);
        doc.set_offset(3, true);
        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret());

        // Left without Shift should collapse to min
        doc.run_move_command(&MoveCommand::Left, false);
        let cursor = doc.cursor().get_untracked();
        assert!(cursor.is_caret());
        assert_eq!(cursor.end, 1);
    }

    #[test]
    fn test_typing_replaces_selection() {
        let doc = Document::new("hello");
        doc.set_width(100.0);

        // Select "ell" (positions 1-4)
        doc.set_offset(1, false);
        doc.set_offset(4, true);
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.min(), 1);
        assert_eq!(cursor.max(), 4);

        // Type 'X' - should replace selection
        doc.insert_text("X");
        assert_eq!(doc.text(), "hXo");

        // Cursor should be after inserted text
        let cursor = doc.cursor().get_untracked();
        assert!(cursor.is_caret());
        assert_eq!(cursor.end, 2);
    }

    #[test]
    fn test_shift_up_extends_selection_multiline() {
        let doc = Document::new("line1\nline2\nline3");
        doc.set_width(100.0);

        // Move to middle of line2 (offset 8 = "line1\nli")
        doc.set_offset(8, false);
        let cursor = doc.cursor().get_untracked();
        assert!(cursor.is_caret());
        assert_eq!(cursor.end, 8);

        // Shift+Up should extend selection to line1
        doc.run_move_command(&MoveCommand::Up, true);
        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret());
        assert_eq!(cursor.start, 8, "Anchor should remain at 8");
        // End should be somewhere in line1
        assert!(cursor.end < 6, "Selection end should be in line1");
    }

    #[test]
    fn test_shift_down_extends_selection_multiline() {
        let doc = Document::new("line1\nline2\nline3");
        doc.set_width(100.0);

        // Start at position 2 in line1
        doc.set_offset(2, false);

        // Shift+Down should extend selection to line2
        doc.run_move_command(&MoveCommand::Down, true);
        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret());
        assert_eq!(cursor.start, 2, "Anchor should remain at 2");
        // End should be in line2
        assert!(cursor.end >= 6, "Selection end should be in line2 or beyond");
    }

    #[test]
    fn test_delete_backward_removes_selection() {
        let doc = Document::new("hello world");
        doc.set_width(100.0);

        // Select "lo wo" (positions 3-8)
        doc.set_offset(3, false);
        doc.set_offset(8, true);

        // Delete backward should remove selection
        doc.run_edit_command(&EditCommand::DeleteBackward);
        assert_eq!(doc.text(), "helrld");

        let cursor = doc.cursor().get_untracked();
        assert!(cursor.is_caret());
        assert_eq!(cursor.end, 3);
    }

    #[test]
    fn test_bidirectional_selection() {
        let doc = Document::new("hello");
        doc.set_width(100.0);

        // Start at position 3
        doc.set_offset(3, false);

        // Select left twice, then right three times
        doc.run_move_command(&MoveCommand::Left, true);  // anchor=3, end=2
        doc.run_move_command(&MoveCommand::Left, true);  // anchor=3, end=1

        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 3);
        assert_eq!(cursor.end, 1);

        // Now extend right
        doc.run_move_command(&MoveCommand::Right, true); // anchor=3, end=2
        doc.run_move_command(&MoveCommand::Right, true); // anchor=3, end=3
        doc.run_move_command(&MoveCommand::Right, true); // anchor=3, end=4

        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 3);
        assert_eq!(cursor.end, 4);
        assert_eq!(cursor.min(), 3);
        assert_eq!(cursor.max(), 4);
    }

    // ==========================================================================
    // Double-click and triple-click tests
    // ==========================================================================

    /// Helper to create a PointerState for testing clicks
    fn create_pointer_state(x: f64, y: f64, count: u8) -> PointerState {
        use dpi::PhysicalPosition;
        use ui_events::pointer::{ContactGeometry, PointerButtons, PointerOrientation};
        use ui_events::keyboard::Modifiers;

        PointerState {
            time: 0,
            position: PhysicalPosition::new(x, y),
            buttons: PointerButtons::default(),
            modifiers: Modifiers::default(),
            count,
            contact_geometry: ContactGeometry::default(),
            orientation: PointerOrientation::default(),
            pressure: 0.5,
            tangential_pressure: 0.0,
            scale_factor: 1.0,
        }
    }

    #[test]
    fn test_double_click_selects_word() {
        // "hello world" - clicking on 'e' should select "hello"
        let doc = Document::new("hello world");
        doc.set_width(200.0);

        // Get the x position for offset 1 (the 'e' in hello)
        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(1);
        drop(lines);

        // Double-click at position of 'e'
        let state = create_pointer_state(point.x, point.glyph_top, 2);
        doc.double_click(&state);

        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Double-click should create a selection");
        assert_eq!(cursor.min(), 0, "Selection should start at beginning of 'hello'");
        assert_eq!(cursor.max(), 5, "Selection should end after 'hello'");
    }

    #[test]
    fn test_double_click_selects_word_in_middle() {
        // "one two three" - clicking on 'w' should select "two"
        let doc = Document::new("one two three");
        doc.set_width(200.0);

        // Get the x position for offset 5 (the 'w' in two)
        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(5);
        drop(lines);

        // Double-click at position of 'w'
        let state = create_pointer_state(point.x, point.glyph_top, 2);
        doc.double_click(&state);

        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Double-click should create a selection");
        assert_eq!(cursor.min(), 4, "Selection should start at beginning of 'two'");
        assert_eq!(cursor.max(), 7, "Selection should end after 'two'");
    }

    #[test]
    fn test_double_click_on_space() {
        // "hello world" - clicking on space should select the space (or word boundary behavior)
        let doc = Document::new("hello world");
        doc.set_width(200.0);

        // Get the x position for offset 5 (the space between hello and world)
        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(5);
        drop(lines);

        // Double-click at the space
        let state = create_pointer_state(point.x, point.glyph_top, 2);
        doc.double_click(&state);

        let cursor = doc.cursor().get_untracked();
        // Behavior may vary - just verify it creates some selection
        assert!(!cursor.is_caret(), "Double-click should create a selection");
    }

    #[test]
    fn test_triple_click_selects_line() {
        // "line1\nline2\nline3" - triple-clicking on line2 should select "line2\n"
        let doc = Document::new("line1\nline2\nline3");
        doc.set_width(200.0);

        // Get the position for offset 8 (somewhere in line2)
        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(8);
        drop(lines);

        // Triple-click on line2
        let state = create_pointer_state(point.x, point.glyph_top, 3);
        doc.triple_click(&state);

        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Triple-click should create a selection");
        // line2 starts at offset 6 and ends at offset 12 (including newline)
        assert_eq!(cursor.min(), 6, "Selection should start at beginning of line2");
        assert_eq!(cursor.max(), 12, "Selection should end after line2 (including newline)");
    }

    #[test]
    fn test_triple_click_selects_first_line() {
        let doc = Document::new("first\nsecond\nthird");
        doc.set_width(200.0);

        // Triple-click on first line
        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(2); // 'r' in first
        drop(lines);

        let state = create_pointer_state(point.x, point.glyph_top, 3);
        doc.triple_click(&state);

        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Triple-click should create a selection");
        assert_eq!(cursor.min(), 0, "Selection should start at 0");
        assert_eq!(cursor.max(), 6, "Selection should end at 6 (after 'first\\n')");
    }

    #[test]
    fn test_triple_click_selects_last_line() {
        let doc = Document::new("first\nsecond\nthird");
        doc.set_width(200.0);

        // Triple-click on last line (offset 14 = 't' in third)
        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(14);
        drop(lines);

        let state = create_pointer_state(point.x, point.glyph_top, 3);
        doc.triple_click(&state);

        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Triple-click should create a selection");
        assert_eq!(cursor.min(), 13, "Selection should start at beginning of 'third'");
        // Last line doesn't have a newline, so selection goes to end
        assert_eq!(cursor.max(), 18, "Selection should end at end of text");
    }

    #[test]
    fn test_triple_click_single_line() {
        let doc = Document::new("single line text");
        doc.set_width(200.0);

        // Triple-click somewhere in the middle
        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(7);
        drop(lines);

        let state = create_pointer_state(point.x, point.glyph_top, 3);
        doc.triple_click(&state);

        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Triple-click should create a selection");
        assert_eq!(cursor.min(), 0, "Selection should start at 0");
        assert_eq!(cursor.max(), 16, "Selection should cover entire line");
    }

    #[test]
    fn test_single_click_sets_cursor() {
        let doc = Document::new("hello world");
        doc.set_width(200.0);

        // Get position for offset 3
        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(3);
        drop(lines);

        // Single-click
        let state = create_pointer_state(point.x, point.glyph_top, 1);
        doc.single_click(&state);

        let cursor = doc.cursor().get_untracked();
        assert!(cursor.is_caret(), "Single-click should set a caret");
        assert_eq!(cursor.end, 3, "Cursor should be at clicked position");
    }

    #[test]
    fn test_single_click_with_shift_extends_selection() {
        let doc = Document::new("hello world");
        doc.set_width(200.0);

        // First, position cursor at offset 2
        doc.set_offset(2, false);

        // Get position for offset 8
        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(8);
        drop(lines);

        // Shift-click at offset 8
        let mut state = create_pointer_state(point.x, point.glyph_top, 1);
        state.modifiers = ui_events::keyboard::Modifiers::SHIFT;
        doc.single_click(&state);

        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Shift-click should create a selection");
        assert_eq!(cursor.start, 2, "Selection anchor should stay at 2");
        assert_eq!(cursor.end, 8, "Selection end should be at 8");
    }

    /// Helper to create a PointerButtonEvent for testing
    fn create_pointer_button_event(x: f64, y: f64, count: u8, button: PointerButton) -> PointerButtonEvent {
        use dpi::PhysicalPosition;
        use ui_events::pointer::{ContactGeometry, PointerButtons, PointerInfo, PointerOrientation, PointerType};

        PointerButtonEvent {
            button: Some(button),
            state: PointerState {
                time: 0,
                position: PhysicalPosition::new(x, y),
                buttons: PointerButtons::default(),
                modifiers: ui_events::keyboard::Modifiers::default(),
                count,
                contact_geometry: ContactGeometry::default(),
                orientation: PointerOrientation::default(),
                pressure: 0.5,
                tangential_pressure: 0.0,
                scale_factor: 1.0,
            },
            pointer: PointerInfo {
                pointer_id: None,
                persistent_device_id: None,
                pointer_type: PointerType::Mouse,
            },
        }
    }

    #[test]
    fn test_pointer_down_double_click_via_event() {
        // Test that pointer_down correctly routes double-clicks
        let doc = Document::new("hello world");
        doc.set_width(200.0);

        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(1);
        drop(lines);

        // Create a PointerButtonEvent with count=2 (double-click)
        let event = create_pointer_button_event(point.x, point.glyph_top, 2, PointerButton::Primary);
        doc.pointer_down(&event);

        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Double-click via pointer_down should create a selection");
        assert_eq!(cursor.min(), 0, "Selection should start at beginning of 'hello'");
        assert_eq!(cursor.max(), 5, "Selection should end after 'hello'");
    }

    #[test]
    fn test_pointer_down_triple_click_via_event() {
        // Test that pointer_down correctly routes triple-clicks
        let doc = Document::new("first line\nsecond line");
        doc.set_width(200.0);

        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(3); // 's' in 'first'
        drop(lines);

        // Create a PointerButtonEvent with count=3 (triple-click)
        let event = create_pointer_button_event(point.x, point.glyph_top, 3, PointerButton::Primary);
        doc.pointer_down(&event);

        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Triple-click via pointer_down should create a selection");
        assert_eq!(cursor.min(), 0, "Selection should start at 0");
        assert_eq!(cursor.max(), 11, "Selection should end after first line (including newline)");
    }

    #[test]
    fn test_pointer_down_count_zero_does_nothing() {
        // Test that count=0 doesn't crash or change cursor
        let doc = Document::new("hello world");
        doc.set_width(200.0);
        doc.set_offset(5, false); // Set cursor at position 5

        let lines = doc.text_layouts().borrow();
        let point = lines.point_of_offset(1);
        drop(lines);

        // Create a PointerButtonEvent with count=0 (edge case)
        let event = create_pointer_button_event(point.x, point.glyph_top, 0, PointerButton::Primary);
        doc.pointer_down(&event);

        // Cursor should be unchanged since count=0 doesn't match any case
        let cursor = doc.cursor().get_untracked();
        assert!(cursor.is_caret());
        assert_eq!(cursor.end, 5, "Cursor should be unchanged when count=0");
    }

    // ==========================================================================
    // Tests for double/triple click not being affected by pointer move/up
    // ==========================================================================

    /// Helper to create a PointerUpdate for testing pointer move
    fn create_pointer_update(x: f64, y: f64) -> PointerUpdate {
        use dpi::PhysicalPosition;
        use ui_events::pointer::{ContactGeometry, PointerButtons, PointerInfo, PointerOrientation, PointerType};

        let state = PointerState {
            time: 0,
            position: PhysicalPosition::new(x, y),
            buttons: PointerButtons::default(),
            modifiers: ui_events::keyboard::Modifiers::default(),
            count: 0,
            contact_geometry: ContactGeometry::default(),
            orientation: PointerOrientation::default(),
            pressure: 0.5,
            tangential_pressure: 0.0,
            scale_factor: 1.0,
        };

        PointerUpdate {
            current: state,
            pointer: PointerInfo {
                pointer_id: None,
                persistent_device_id: None,
                pointer_type: PointerType::Mouse,
            },
            coalesced: vec![],
            predicted: vec![],
        }
    }

    #[test]
    fn test_double_click_not_affected_by_pointer_move() {
        let doc = Document::new("hello world");
        doc.set_width(200.0);

        // Get positions
        let lines = doc.text_layouts().borrow();
        let click_point = lines.point_of_offset(1); // 'e' in hello
        let move_point = lines.point_of_offset(8);  // 'o' in world
        drop(lines);

        // Double-click to select "hello"
        let down_event = create_pointer_button_event(click_point.x, click_point.glyph_top, 2, PointerButton::Primary);
        doc.pointer_down(&down_event);

        // Verify word is selected
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.min(), 0, "Selection should start at 0");
        assert_eq!(cursor.max(), 5, "Selection should end at 5");

        // Simulate pointer move to a different location
        let move_event = create_pointer_update(move_point.x, move_point.glyph_top);
        doc.pointer_move(&move_event);

        // Selection should NOT change because active is false for double-click
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.min(), 0, "Selection start should remain at 0 after move");
        assert_eq!(cursor.max(), 5, "Selection end should remain at 5 after move");
    }

    #[test]
    fn test_triple_click_not_affected_by_pointer_move() {
        let doc = Document::new("first line\nsecond line");
        doc.set_width(200.0);

        // Get positions
        let lines = doc.text_layouts().borrow();
        let click_point = lines.point_of_offset(3);  // 's' in first
        let move_point = lines.point_of_offset(15);  // somewhere in second line
        drop(lines);

        // Triple-click to select first line
        let down_event = create_pointer_button_event(click_point.x, click_point.glyph_top, 3, PointerButton::Primary);
        doc.pointer_down(&down_event);

        // Verify line is selected
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.min(), 0, "Selection should start at 0");
        assert_eq!(cursor.max(), 11, "Selection should end at 11");

        // Simulate pointer move to second line
        let move_event = create_pointer_update(move_point.x, move_point.glyph_top);
        doc.pointer_move(&move_event);

        // Selection should NOT change because active is false for triple-click
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.min(), 0, "Selection start should remain at 0 after move");
        assert_eq!(cursor.max(), 11, "Selection end should remain at 11 after move");
    }

    #[test]
    fn test_double_click_not_affected_by_pointer_up() {
        let doc = Document::new("hello world");
        doc.set_width(200.0);

        let lines = doc.text_layouts().borrow();
        let click_point = lines.point_of_offset(1);
        let up_point = lines.point_of_offset(8);
        drop(lines);

        // Double-click to select "hello"
        let down_event = create_pointer_button_event(click_point.x, click_point.glyph_top, 2, PointerButton::Primary);
        doc.pointer_down(&down_event);

        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.min(), 0);
        assert_eq!(cursor.max(), 5);

        // Pointer up at different location
        let up_event = create_pointer_button_event(up_point.x, up_point.glyph_top, 2, PointerButton::Primary);
        doc.pointer_up(&up_event);

        // Selection should remain unchanged
        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.min(), 0, "Selection should remain at 0-5 after pointer up");
        assert_eq!(cursor.max(), 5);
    }

    #[test]
    fn test_single_click_drag_does_extend_selection() {
        // Verify that single-click + drag DOES extend the selection (as expected)
        let doc = Document::new("hello world");
        doc.set_width(200.0);

        let lines = doc.text_layouts().borrow();
        let click_point = lines.point_of_offset(2);  // 'l' in hello
        let drag_point = lines.point_of_offset(8);   // 'o' in world
        drop(lines);

        // Single-click
        let down_event = create_pointer_button_event(click_point.x, click_point.glyph_top, 1, PointerButton::Primary);
        doc.pointer_down(&down_event);

        // Verify cursor is set
        let cursor = doc.cursor().get_untracked();
        assert!(cursor.is_caret(), "Single click should create a caret");
        assert_eq!(cursor.end, 2);

        // Drag to different position
        let move_event = create_pointer_update(drag_point.x, drag_point.glyph_top);
        doc.pointer_move(&move_event);

        // Selection SHOULD be extended for single-click drag
        let cursor = doc.cursor().get_untracked();
        assert!(!cursor.is_caret(), "Drag should create a selection");
        assert_eq!(cursor.start, 2, "Selection anchor should be at click position");
        assert_eq!(cursor.end, 8, "Selection end should be at drag position");
    }

    #[test]
    fn test_single_click_drag_stops_on_pointer_up() {
        let doc = Document::new("hello world");
        doc.set_width(200.0);

        let lines = doc.text_layouts().borrow();
        let click_point = lines.point_of_offset(2);
        let drag_point = lines.point_of_offset(6);
        let after_up_point = lines.point_of_offset(10);
        drop(lines);

        // Single-click and drag
        let down_event = create_pointer_button_event(click_point.x, click_point.glyph_top, 1, PointerButton::Primary);
        doc.pointer_down(&down_event);

        let move_event = create_pointer_update(drag_point.x, drag_point.glyph_top);
        doc.pointer_move(&move_event);

        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 2);
        assert_eq!(cursor.end, 6);

        // Pointer up
        let up_event = create_pointer_button_event(drag_point.x, drag_point.glyph_top, 1, PointerButton::Primary);
        doc.pointer_up(&up_event);

        // Moving after pointer up should NOT change selection
        let move_after_up = create_pointer_update(after_up_point.x, after_up_point.glyph_top);
        doc.pointer_move(&move_after_up);

        let cursor = doc.cursor().get_untracked();
        assert_eq!(cursor.start, 2, "Selection should not change after pointer up");
        assert_eq!(cursor.end, 6, "Selection should not change after pointer up");
    }
}
