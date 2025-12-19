use std::{
    cmp::Ordering,
    ops::{Add, Range, Sub},
};

use floem::{
    kurbo::Point,
    text::{LayoutLine, LayoutRun, LineEnding, TextLayout},
};
use lapce_xi_rope::{
    tree::{Leaf, Node, NodeInfo, TreeBuilder, UnitAffinity, UnitConverter},
    Cursor, Delta, DeltaElement, Interval,
};

const MIN_LEAF: usize = 511;
const MAX_LEAF: usize = 1024;

pub struct TextLayoutLines {
    pub(crate) tree: Node<TextLayoutLineInfo>,
    /// Default glyph height from font metrics (ascent + descent)
    default_glyph_height: f64,
    /// Default glyph top position (centering offset within line height)
    default_glyph_top: f64,
}

impl TextLayoutLines {
    pub fn builder() -> TextLayoutLineBuilder {
        TextLayoutLineBuilder::new()
    }

    pub fn utf8_len(&self) -> usize {
        self.tree.len()
    }

    pub fn num_visual_lines(&self) -> usize {
        let info = self.tree.info();
        info.line_breaks + info.line_endings + 1
    }

    /// Returns the default glyph height from font metrics.
    /// This is used for cursor drawing when text is empty.
    pub fn default_glyph_height(&self) -> f64 {
        self.default_glyph_height
    }

    /// Returns the default glyph top position (centering offset within line height).
    /// This is used for cursor positioning when text is empty.
    pub fn default_glyph_top(&self) -> f64 {
        self.default_glyph_top
    }

    pub fn apply_delta(&mut self, delta: Delta<TextLayoutLineInfo>) {
        let mut b: TreeBuilder<TextLayoutLineInfo> = TreeBuilder::new();
        for elem in delta.els {
            match elem {
                DeltaElement::Copy(beg, end) => b.push(self.tree.subseq(Interval::new(beg, end))),
                DeltaElement::Insert(n) => b.push(n),
            }
        }
        self.tree = b.build();
    }

    pub fn visual_line(&self, actual_line: usize) -> usize {
        self.tree.count_unit::<usize, VlineLineConverter>(actual_line)
    }

    pub fn actual_line(&self, visual_line: usize) -> usize {
        self.tree
            .measure_unit::<usize, VlineLineConverter>(visual_line)
    }

    pub fn vline_of_height(&self, height: f64) -> usize {
        self.tree
            .convert::<GlyphPoint, PointConverter, usize, VlineConverter>(GlyphPoint {
                x: 0.0,
                glyph_top: 0.0,
                glyph_bottom: 0.0,
                line_bottom: 0.0,
                line_top: 0.0,
                total_height: height,
            })
    }

    pub fn height_of_vline(&self, vline: usize) -> f64 {
        self.tree
            .convert::<usize, VlineConverter, GlyphPoint, PointConverter>(vline)
            .line_bottom
    }

    pub fn offset_of_vline(&self, vline: usize) -> usize {
        self.tree
            .convert::<usize, VlineConverter, usize, BaseConverter>(vline)
    }

    pub fn vline_of_offset(&self, offset: usize) -> usize {
        self.tree
            .convert::<usize, BaseConverter, usize, VlineConverter>(offset)
    }

    pub fn point_of_offset(&self, offset: usize) -> GlyphPoint {
        self.tree
            .convert::<usize, BaseConverter, GlyphPoint, PointConverter>(offset)
    }

    pub fn offset_of_point(&self, point: Point) -> usize {
        self.tree
            .convert::<GlyphPoint, PointConverter, usize, BaseConverter>(GlyphPoint {
                x: point.x,
                glyph_top: 0.0,
                glyph_bottom: 0.0,
                line_bottom: 0.0,
                line_top: 0.0,
                total_height: point.y,
            })
    }

    // Iter over the visual lines given the range of offsets
    pub fn visual_lines(&self, range: Range<usize>) -> VisualLineIter {
        let height = self.point_of_offset(range.start).line_top as f32;
        let vline = self.vline_of_offset(range.start);
        VisualLineIter {
            cursor: Cursor::new(&self.tree, range.start),
            end: range.end,
            leaf_visual_index: None,
            vline,
            height,
        }
    }
}

#[derive(Clone)]
pub struct TextLayoutLineInfo {
    // utf8 lenght of the chars the leaf contains
    utf8_len: usize,
    // line endings the leaf contains
    line_endings: usize,
    // line breaks the leaf contains
    line_breaks: usize,
    // number of visual lines
    num_vlines: usize,
    // max width of the visual lines
    max_width: f64,
    // total line heights of the visual lines
    total_height: f64,
    // last glyph point in the leaf
    last_glyph: GlyphPoint,
}

#[derive(Clone)]
pub struct TextLayoutLine {
    line: LayoutLine,
    line_ending: LineEnding,
    // if there is line break, i.e., the next line is wrapped
    line_break: bool,
    line_height: f64,
    // utf8 lenght of the chars this line contains
    utf8_len: usize,
}

#[derive(Default, Clone)]
pub struct TextLayoutLineLeaf {
    // utf8 lenght of the chars this leaf contains
    utf8_len: usize,
    // line endings the leaf contains
    line_endings: usize,
    // line breaks the leaf contains
    line_breaks: usize,
    // the visual lines in this leaf
    visual_lines: Vec<TextLayoutLine>,
    // max width of the visual lines
    max_width: f64,
    // total line heights of the visual lines
    total_height: f64,
    // last glyph point
    last_glyph: GlyphPoint,
    // default glyph height for empty lines (from font metrics)
    default_glyph_height: f32,
    // default centering offset for empty lines
    default_centering_offset: f32,
}

impl TextLayoutLineLeaf {
    fn push_layout_line(
        &mut self,
        line: &LayoutLine,
        utf8_len: usize,
        line_ending: LineEnding,
        line_break: bool,
        line_height: f64,
        default_glyph_height: f32,
        default_centering_offset: f32,
    ) {
        let line_height = line
            .line_height_opt
            .map(|h| h as f64)
            .unwrap_or(line_height);
        self.utf8_len += utf8_len;
        if self.max_width < line.w as f64 {
            self.max_width = line.w as f64
        }

        // Store default glyph metrics
        self.default_glyph_height = default_glyph_height;
        self.default_centering_offset = default_centering_offset;

        {
            let mut glyph_height = line.max_ascent + line.max_descent;
            let mut centering_offset = (line_height as f32 - glyph_height) / 2.0;

            // Use defaults for empty lines (no glyphs)
            if glyph_height == 0.0 && default_glyph_height > 0.0 {
                glyph_height = default_glyph_height;
                centering_offset = default_centering_offset;
            }

            self.last_glyph = GlyphPoint {
                x: line.w as f64,
                line_top: self.total_height,
                line_bottom: self.total_height + line_height,
                glyph_top: self.total_height + centering_offset as f64,
                glyph_bottom: self.total_height + (centering_offset + glyph_height) as f64,
                total_height: self.total_height + line_height,
            };
            if line_ending != LineEnding::None || line_break {
                self.last_glyph.x = 0.0;
                self.last_glyph.line_top += line_height;
                self.last_glyph.line_bottom += line_height;
                self.last_glyph.glyph_top += line_height;
                self.last_glyph.glyph_bottom += line_height;
            }
        }

        self.total_height += line_height;
        if line_ending != LineEnding::None {
            self.line_endings += 1;
        }
        if line_break {
            self.line_breaks += 1;
        }

        self.visual_lines.push(TextLayoutLine {
            line: line.to_owned(),
            line_ending,
            line_break,
            line_height,
            utf8_len,
        });
    }
}

impl NodeInfo for TextLayoutLineInfo {
    type L = TextLayoutLineLeaf;

    fn accumulate(&mut self, other: &Self) {
        self.utf8_len += other.utf8_len;
        self.line_endings += other.line_endings;
        self.line_breaks += other.line_breaks;
        self.num_vlines += other.num_vlines;
        self.max_width = self.max_width.max(other.max_width);
        self.total_height += other.total_height;
        self.last_glyph = self.last_glyph.clone() + other.last_glyph.clone();
    }

    fn compute_info(leaf: &Self::L) -> Self {
        Self {
            utf8_len: leaf.utf8_len,
            line_endings: leaf.line_endings,
            line_breaks: leaf.line_breaks,
            max_width: leaf.max_width,
            num_vlines: leaf.visual_lines.len(),
            total_height: leaf.total_height,
            last_glyph: leaf.last_glyph.clone(),
        }
    }
}

impl Leaf for TextLayoutLineLeaf {
    fn len(&self) -> usize {
        self.utf8_len
    }

    fn is_ok_child(&self) -> bool {
        true
        // self.utf8_len >= MIN_LEAF
    }

    fn push_maybe_split(&mut self, other: &Self, _: lapce_xi_rope::Interval) -> Option<Self> {
        let (start, end) = (0, other.visual_lines.len());

        let mut index = start;
        for line in &other.visual_lines[start..end] {
            if self.utf8_len > MAX_LEAF {
                break;
            }
            self.push_layout_line(
                &line.line,
                line.utf8_len,
                line.line_ending,
                line.line_break,
                line.line_height,
                other.default_glyph_height,
                other.default_centering_offset,
            );
            index += 1;
        }

        if index < end {
            let mut leaf = TextLayoutLineLeaf::default();
            for line in &other.visual_lines[start..end] {
                leaf.push_layout_line(
                    &line.line,
                    line.utf8_len,
                    line.line_ending,
                    line.line_break,
                    line.line_height,
                    other.default_glyph_height,
                    other.default_centering_offset,
                );
            }
            return Some(leaf);
        }

        None
    }
}

pub struct TextLayoutLineBuilder {
    builder: TreeBuilder<TextLayoutLineInfo>,
    default_glyph_height: f64,
    default_glyph_top: f64,
}

impl Default for TextLayoutLineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TextLayoutLineBuilder {
    pub fn new() -> Self {
        Self {
            builder: TreeBuilder::new(),
            default_glyph_height: 16.0, // fallback value
            default_glyph_top: 0.0,
        }
    }

    pub fn build(self) -> TextLayoutLines {
        TextLayoutLines {
            tree: self.builder.build(),
            default_glyph_height: self.default_glyph_height,
            default_glyph_top: self.default_glyph_top,
        }
    }

    /// Sets the default glyph metrics from a TextLayout.
    /// Call this with any text layout (e.g., one containing a space) to capture
    /// the font's glyph metrics for use when text is empty.
    pub fn set_default_from_layout(&mut self, text_layout: &TextLayout) {
        let metrics = text_layout.metrics();
        let default_line_height = metrics.line_height as f64;
        for buffer_line in text_layout.lines() {
            let lines = buffer_line.layout_opt().into_iter().flatten();
            for line in lines {
                let glyph_height = (line.max_ascent + line.max_descent) as f64;
                if glyph_height > 0.0 {
                    // Use line_height_opt if available (respects attrs), otherwise use default
                    let line_height = line
                        .line_height_opt
                        .map(|h| h as f64)
                        .unwrap_or(default_line_height);
                    let centering_offset = (line_height - glyph_height) / 2.0;
                    self.default_glyph_height = glyph_height;
                    self.default_glyph_top = centering_offset;
                    return;
                }
            }
        }
    }

    pub fn push_text_layouts(&mut self, text_layouts: &[TextLayout]) {
        for text_layout in text_layouts.iter() {
            self.push_text_layout(text_layout);
        }
    }

    pub fn push_text_layout(&mut self, text_layout: &TextLayout) {
        let metrics = text_layout.metrics();
        let mut captured_default = self.default_glyph_height > 16.0; // already have a real value
        for buffer_line in text_layout.lines() {
            let line_ending = buffer_line.ending();
            let mut leaf = TextLayoutLineLeaf::default();
            let lines = buffer_line.layout_opt().into_iter().flatten();
            let mut lines = lines.peekable();
            while let Some(line) = lines.next() {
                // Capture default glyph metrics from first layout line
                if !captured_default {
                    let glyph_height = (line.max_ascent + line.max_descent) as f64;
                    if glyph_height > 0.0 {
                        let line_height = metrics.line_height as f64;
                        let centering_offset = (line_height - glyph_height) / 2.0;
                        self.default_glyph_height = glyph_height;
                        self.default_glyph_top = centering_offset;
                        captured_default = true;
                    }
                }

                let is_last_visual_line = lines.peek().is_none();
                let mut utf8_len = if let Some(next) = lines.peek() {
                    // if there's next line, that means it was wrapped
                    // we use the start offset of the next line
                    // and the start offset of the current line
                    // to get the utf8 len of this line
                    next.glyphs
                        .first()
                        .map(|g| g.start)
                        .unwrap_or(0)
                        .saturating_sub(line.glyphs.first().map(|g| g.start).unwrap_or(0))
                } else {
                    // if there's no next line, this means this is the last visual line
                    // we use the total text len and the start glyph offset
                    // to get the utf8 len of this line
                    buffer_line
                        .text()
                        .len()
                        .saturating_sub(line.glyphs.first().map(|g| g.start).unwrap_or(0))
                };

                // Add the line ending byte length to the last visual line
                if is_last_visual_line {
                    utf8_len += match line_ending {
                        LineEnding::None => 0,
                        LineEnding::Lf => 1,      // \n
                        LineEnding::CrLf => 2,    // \r\n
                        LineEnding::Cr => 1,      // \r
                        _ => 1,                   // Other endings treated as 1 byte
                    };
                }

                leaf.push_layout_line(
                    line,
                    utf8_len,
                    if is_last_visual_line {
                        line_ending
                    } else {
                        LineEnding::None
                    },
                    !is_last_visual_line,
                    metrics.line_height as f64,
                    self.default_glyph_height as f32,
                    self.default_glyph_top as f32,
                );
            }
            self.builder.push(Node::from_leaf(leaf));
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GlyphPoint {
    pub x: f64,
    pub line_top: f64,
    pub line_bottom: f64,
    pub glyph_top: f64,
    pub glyph_bottom: f64,
    total_height: f64,
}

impl Sub for GlyphPoint {
    type Output = GlyphPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x,
            line_top: self.line_top - rhs.line_top,
            line_bottom: self.line_top - rhs.line_top,
            glyph_top: self.line_top - rhs.line_top,
            glyph_bottom: self.line_top - rhs.line_top,
            total_height: self.total_height - rhs.total_height,
        }
    }
}

impl Add for GlyphPoint {
    type Output = GlyphPoint;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: rhs.x,
            line_top: self.line_top + rhs.line_top,
            line_bottom: self.line_top + rhs.line_bottom,
            glyph_top: self.line_top + rhs.glyph_top,
            glyph_bottom: self.line_top + rhs.glyph_bottom,
            total_height: self.total_height + rhs.total_height,
        }
    }
}

impl PartialEq for GlyphPoint {
    fn eq(&self, other: &Self) -> bool {
        self.total_height.eq(&other.total_height) && self.x.eq(&other.x)
    }
}

impl PartialOrd for GlyphPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total_height.partial_cmp(&other.total_height)
    }
}

pub struct BaseConverter(());

impl UnitConverter<TextLayoutLineInfo, usize> for BaseConverter {
    fn count(_: &<TextLayoutLineInfo as NodeInfo>::L, in_measured_units: usize) -> usize {
        in_measured_units
    }

    fn measure(_: &<TextLayoutLineInfo as NodeInfo>::L, in_base_units: usize) -> usize {
        in_base_units
    }

    fn base(l: &TextLayoutLineInfo) -> usize {
        l.utf8_len
    }

    fn node_measured(l: &TextLayoutLineInfo) -> usize {
        l.utf8_len
    }

    fn count_affinty() -> UnitAffinity {
        UnitAffinity::After
    }

    fn measure_affinty() -> UnitAffinity {
        UnitAffinity::After
    }
}

// Converter for visual line
pub struct VlineConverter(());

impl UnitConverter<TextLayoutLineInfo, usize> for VlineConverter {
    fn count(l: &<TextLayoutLineInfo as NodeInfo>::L, in_measured_units: usize) -> usize {
        l.visual_lines[..in_measured_units]
            .iter()
            // .filter(|l| l.line_ending != LineEnding::None || l.line_break)
            .map(|l| l.utf8_len)
            .sum()
    }

    fn measure(l: &<TextLayoutLineInfo as NodeInfo>::L, in_base_units: usize) -> usize {
        let mut base = in_base_units;
        let mut n = 0;
        for l in l.visual_lines.iter() {
            match base.cmp(&l.utf8_len) {
                Ordering::Greater => {
                    base -= l.utf8_len;
                    n += 1;
                    continue;
                }
                Ordering::Equal => {
                    if l.line_ending != LineEnding::None || l.line_break {
                        return n + 1;
                    } else {
                        return n;
                    }
                }
                Ordering::Less => {
                    return n;
                }
            }
        }
        n
    }

    fn base(l: &TextLayoutLineInfo) -> usize {
        l.utf8_len
    }

    fn node_measured(l: &TextLayoutLineInfo) -> usize {
        l.num_vlines
    }

    fn count_affinty() -> UnitAffinity {
        UnitAffinity::Before
    }

    fn measure_affinty() -> UnitAffinity {
        UnitAffinity::Before
    }
}

// Converter for glpyh point
pub struct PointConverter(());

impl UnitConverter<TextLayoutLineInfo, GlyphPoint> for PointConverter {
    fn count(l: &<TextLayoutLineInfo as NodeInfo>::L, in_measured_units: GlyphPoint) -> usize {
        let mut height = in_measured_units.total_height;
        let x = in_measured_units.x;
        let mut offset = 0;
        for l in l.visual_lines.iter() {
            if height < l.line_height {
                let line_start_offset = offset;
                for g in l.line.glyphs.iter() {
                    if x < (g.x + g.w) as f64 {
                        return offset;
                    } else {
                        offset += g.end - g.start;
                    }
                }
                if l.line_ending != LineEnding::None || l.line_break {
                    // Return position before the line ending, but not before line start
                    // For empty lines, return the line start
                    return if offset > line_start_offset {
                        offset - 1
                    } else {
                        offset
                    };
                } else {
                    return offset;
                }
            } else {
                height -= l.line_height;
                offset += l.utf8_len;
            }
        }
        offset
    }

    fn measure(leaf: &<TextLayoutLineInfo as NodeInfo>::L, in_base_units: usize) -> GlyphPoint {
        let mut base = in_base_units;
        let mut point = GlyphPoint::default();
        for l in leaf.visual_lines.iter() {
            let mut glyph_height = l.line.max_ascent + l.line.max_descent;
            let mut centering_offset = (l.line_height as f32 - glyph_height) / 2.0;

            // Use defaults for empty lines (no glyphs)
            if glyph_height == 0.0 && leaf.default_glyph_height > 0.0 {
                glyph_height = leaf.default_glyph_height;
                centering_offset = leaf.default_centering_offset;
            }

            match base.cmp(&l.utf8_len) {
                Ordering::Greater => {
                    base -= l.utf8_len;
                    point.x = 0 as f64;
                    point.line_top += l.line_height;
                    point.glyph_top += l.line_height;
                    point.glyph_bottom += l.line_height;
                    point.line_bottom += l.line_height;
                    continue;
                }
                Ordering::Equal => {
                    if l.line_ending != LineEnding::None || l.line_break {
                        base -= l.utf8_len;
                        point.x = 0 as f64;
                        point.line_top += l.line_height;
                        point.glyph_top += l.line_height;
                        point.glyph_bottom += l.line_height;
                        point.line_bottom += l.line_height;
                    } else {
                        point.x = l.line.w as f64;
                        point.glyph_top += centering_offset as f64;
                        point.glyph_bottom += (centering_offset + glyph_height) as f64;
                        point.line_bottom += l.line_height;
                        return point;
                    }
                }
                Ordering::Less => {
                    point.x = 0 as f64;
                    point.glyph_top += centering_offset as f64;
                    point.glyph_bottom += (centering_offset + glyph_height) as f64;
                    point.line_bottom += l.line_height;
                    for g in &l.line.glyphs {
                        let len = g.end - g.start;
                        if base < len {
                            point.x = g.x as f64;
                            return point;
                        }
                        base -= len;
                    }
                    point.x = l.line.w as f64;
                    return point;
                }
            }
        }

        point
    }

    fn base(l: &TextLayoutLineInfo) -> usize {
        l.utf8_len
    }

    fn node_measured(l: &TextLayoutLineInfo) -> GlyphPoint {
        l.last_glyph.clone()
    }

    fn count_affinty() -> UnitAffinity {
        UnitAffinity::Before
    }

    fn measure_affinty() -> UnitAffinity {
        UnitAffinity::After
    }
}

// the base unit is the visual line
// the measured unit is the actual line
pub struct VlineLineConverter(());

impl UnitConverter<TextLayoutLineInfo, usize> for VlineLineConverter {
    fn count(l: &<TextLayoutLineInfo as NodeInfo>::L, in_measured_units: usize) -> usize {
        let mut m = 0;
        let mut i = 0;
        for line in l.visual_lines.iter() {
            if m >= in_measured_units {
                return i;
            }
            if line.line_ending != LineEnding::None {
                m += 1;
            }
            i += 1;
        }
        i
    }

    fn measure(l: &<TextLayoutLineInfo as NodeInfo>::L, in_base_units: usize) -> usize {
        l.visual_lines[..in_base_units]
            .iter()
            .filter(|l| l.line_ending != LineEnding::None)
            .count()
    }

    fn base(l: &TextLayoutLineInfo) -> usize {
        l.line_breaks + l.line_endings
    }

    fn node_measured(l: &TextLayoutLineInfo) -> usize {
        l.line_endings
    }

    fn count_affinty() -> UnitAffinity {
        UnitAffinity::Before
    }

    fn measure_affinty() -> UnitAffinity {
        UnitAffinity::Before
    }
}

pub struct VisualLineIter<'a> {
    cursor: Cursor<'a, TextLayoutLineInfo>,
    leaf_visual_index: Option<usize>,
    vline: usize,
    height: f32,
    end: usize,
}

impl<'a> Iterator for VisualLineIter<'a> {
    type Item = LayoutRun<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.pos() >= self.end {
            return None;
        }
        let (leaf, start_pos) = self.cursor.get_leaf()?;
        if start_pos >= leaf.len() {
            return None;
        }
        let index = if let Some(index) = self.leaf_visual_index {
            index
        } else {
            let mut utf8_len = 0;
            let mut i = 0;
            for l in leaf.visual_lines.iter() {
                if start_pos <= utf8_len {
                    break;
                }
                utf8_len += l.utf8_len;
                i += 1;
            }
            self.leaf_visual_index = Some(i);
            i
        };
        let line = &leaf.visual_lines[index];
        let glyph_height = line.line.max_ascent + line.line.max_descent;
        let centering_offset = (line.line_height as f32 - glyph_height) / 2.0;
        let line_y = self.height + centering_offset + line.line.max_ascent;
        let run = LayoutRun {
            line_i: self.vline,
            text: "",
            rtl: false,
            glyphs: &line.line.glyphs,
            max_ascent: line.line.max_ascent,
            max_descent: line.line.max_descent,
            line_y,
            line_top: self.height,
            line_height: line.line_height as f32,
            line_w: line.line.w,
        };
        self.height += line.line_height as f32;
        self.vline += 1;
        if start_pos + line.utf8_len >= leaf.len() {
            self.leaf_visual_index = Some(0);
            self.cursor.next_leaf();
        } else {
            self.leaf_visual_index = Some(index + 1);
            self.cursor.set(self.cursor.pos() + line.utf8_len);
        }
        Some(run)
    }
}

#[cfg(test)]
mod test {
    use floem::{
        kurbo::Point,
        text::{Attrs, AttrsList, FamilyOwned, LineHeightValue, TextLayout, Weight, FONT_SYSTEM},
    };

    use super::TextLayoutLineBuilder;

    const DEJAVU_SERIF: &[u8] = include_bytes!("../../fonts/DejaVuSerif.ttf");

    fn default_attrs_list() -> AttrsList {
        let mut font_system = FONT_SYSTEM.lock();
        let font_db = font_system.db_mut();
        font_db.load_font_data(Vec::from(DEJAVU_SERIF));

        AttrsList::new(
            Attrs::default()
                .font_size(16.0)
                .weight(Weight::NORMAL)
                .family(&[FamilyOwned::Name("DejaVu".to_string())])
                .line_height(LineHeightValue::Px(20.0)),
        )
    }

    #[test]
    fn test_text_layout_line_wrapped() {
        let attrs_list = default_attrs_list();

        let mut builder = TextLayoutLineBuilder::new();
        let mut text_layout = TextLayout::new_with_text("test\n", attrs_list.clone(), None);
        text_layout.set_size(18.0, f32::MAX);
        builder.push_text_layout(&text_layout);
        builder.push_text_layout(&text_layout);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        // te
        // st\n
        // te
        // st\n
        // te
        // st\n
        //

        assert_eq!(lines.actual_line(0), 0);
        assert_eq!(lines.actual_line(1), 0);
        assert_eq!(lines.actual_line(2), 1);
        assert_eq!(lines.actual_line(3), 1);
        assert_eq!(lines.actual_line(4), 2);
        assert_eq!(lines.actual_line(5), 2);
        assert_eq!(lines.actual_line(6), 3);
        assert_eq!(lines.actual_line(7), 3);
        assert_eq!(lines.actual_line(8), 3);

        assert_eq!(lines.visual_line(0), 0);
        assert_eq!(lines.visual_line(1), 2);
        assert_eq!(lines.visual_line(2), 4);
        assert_eq!(lines.visual_line(3), 6);
        assert_eq!(lines.visual_line(4), 6);
        assert_eq!(lines.visual_line(5), 6);
        assert_eq!(lines.visual_line(6), 6);

        let mut builder = TextLayoutLineBuilder::new();
        let mut text_layout = TextLayout::new_with_text("test\n", attrs_list.clone(), None);
        text_layout.set_size(18.0, f32::MAX);
        builder.push_text_layout(&text_layout);
        builder.push_text_layout(&text_layout);
        let mut text_layout = TextLayout::new_with_text("test", attrs_list.clone(), None);
        text_layout.set_size(18.0, f32::MAX);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        // te
        // st\n
        // te
        // st\n
        // te
        // st

        assert_eq!(lines.actual_line(0), 0);
        assert_eq!(lines.actual_line(1), 0);
        assert_eq!(lines.actual_line(2), 1);
        assert_eq!(lines.actual_line(3), 1);
        assert_eq!(lines.actual_line(4), 2);
        assert_eq!(lines.actual_line(5), 2);
        assert_eq!(lines.actual_line(6), 2);
        assert_eq!(lines.actual_line(7), 2);

        assert_eq!(lines.visual_line(0), 0);
        assert_eq!(lines.visual_line(1), 2);
        assert_eq!(lines.visual_line(2), 4);
        assert_eq!(lines.visual_line(3), 4);
        assert_eq!(lines.visual_line(4), 4);
        assert_eq!(lines.visual_line(5), 4);
        assert_eq!(lines.visual_line(6), 4);

        let mut builder = TextLayoutLineBuilder::new();
        let mut text_layout = TextLayout::new_with_text("test\n", attrs_list.clone(), None);
        text_layout.set_size(18.0, f32::MAX);
        for _ in 0..1000 {
            builder.push_text_layout(&text_layout);
        }
        let lines = builder.build();
        assert_eq!(lines.num_visual_lines(), 2001);
        assert_eq!(lines.actual_line(0), 0);
        assert_eq!(lines.actual_line(1), 0);
        assert_eq!(lines.actual_line(2), 1);
        assert_eq!(lines.actual_line(3), 1);
        assert_eq!(lines.actual_line(1992), 996);
        assert_eq!(lines.actual_line(1993), 996);
        assert_eq!(lines.actual_line(1994), 997);
        assert_eq!(lines.actual_line(1995), 997);
        assert_eq!(lines.actual_line(1996), 998);
        assert_eq!(lines.actual_line(1997), 998);
        assert_eq!(lines.actual_line(1998), 999);
        assert_eq!(lines.actual_line(1999), 999);
        assert_eq!(lines.actual_line(2000), 1000);
        assert_eq!(lines.actual_line(2001), 1000);
        assert_eq!(lines.actual_line(2002), 1000);
        assert_eq!(lines.actual_line(2003), 1000);

        assert_eq!(lines.visual_line(0), 0);
        assert_eq!(lines.visual_line(1), 2);
        assert_eq!(lines.visual_line(2), 4);
        assert_eq!(lines.visual_line(3), 6);
        assert_eq!(lines.visual_line(499), 998);
        assert_eq!(lines.visual_line(500), 1000);
        assert_eq!(lines.visual_line(501), 1002);
        assert_eq!(lines.visual_line(994), 1988);
        assert_eq!(lines.visual_line(995), 1990);
        assert_eq!(lines.visual_line(996), 1992);
        assert_eq!(lines.visual_line(997), 1994);
        assert_eq!(lines.visual_line(998), 1996);
        assert_eq!(lines.visual_line(999), 1998);
        assert_eq!(lines.visual_line(1000), 2000);
        assert_eq!(lines.visual_line(1001), 2000);
        assert_eq!(lines.visual_line(1002), 2000);
        assert_eq!(lines.visual_line(1003), 2000);
        assert_eq!(lines.visual_line(1004), 2000);

        let mut builder = TextLayoutLineBuilder::new();
        let mut text_layout = TextLayout::new_with_text("test\n", attrs_list.clone(), None);
        text_layout.set_size(18.0, f32::MAX);
        for _ in 0..1000 {
            builder.push_text_layout(&text_layout);
        }
        let mut text_layout = TextLayout::new_with_text("test", attrs_list.clone(), None);
        text_layout.set_size(18.0, f32::MAX);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        assert_eq!(lines.actual_line(0), 0);
        assert_eq!(lines.actual_line(1), 0);
        assert_eq!(lines.actual_line(2), 1);
        assert_eq!(lines.actual_line(3), 1);
        assert_eq!(lines.actual_line(1992), 996);
        assert_eq!(lines.actual_line(1993), 996);
        assert_eq!(lines.actual_line(1994), 997);
        assert_eq!(lines.actual_line(1995), 997);
        assert_eq!(lines.actual_line(1996), 998);
        assert_eq!(lines.actual_line(1997), 998);
        assert_eq!(lines.actual_line(1998), 999);
        assert_eq!(lines.actual_line(1999), 999);
        assert_eq!(lines.actual_line(2000), 1000);
        assert_eq!(lines.actual_line(2001), 1000);
        assert_eq!(lines.actual_line(2002), 1000);
        assert_eq!(lines.actual_line(2003), 1000);

        assert_eq!(lines.visual_line(0), 0);
        assert_eq!(lines.visual_line(1), 2);
        assert_eq!(lines.visual_line(2), 4);
        assert_eq!(lines.visual_line(3), 6);
        assert_eq!(lines.visual_line(499), 998);
        assert_eq!(lines.visual_line(500), 1000);
        assert_eq!(lines.visual_line(501), 1002);
        assert_eq!(lines.visual_line(994), 1988);
        assert_eq!(lines.visual_line(995), 1990);
        assert_eq!(lines.visual_line(996), 1992);
        assert_eq!(lines.visual_line(997), 1994);
        assert_eq!(lines.visual_line(998), 1996);
        assert_eq!(lines.visual_line(999), 1998);
        assert_eq!(lines.visual_line(1000), 2000);
        assert_eq!(lines.visual_line(1001), 2000);
        assert_eq!(lines.visual_line(1002), 2000);
        assert_eq!(lines.visual_line(1003), 2000);
        assert_eq!(lines.visual_line(1004), 2000);
    }

    #[test]
    fn test_vline_of_height() {
        let attrs_list = default_attrs_list();
        let mut builder = TextLayoutLineBuilder::new();
        let mut text_layout = TextLayout::new_with_text("test\n", attrs_list.clone(), None);
        text_layout.set_size(15.0, f32::MAX);
        for _ in 0..1000 {
            builder.push_text_layout(&text_layout);
        }
        let lines = builder.build();

        // te
        // st\n
        // te
        // st\n
        // te
        // st\n
        //

        assert_eq!(lines.vline_of_height(0.0), 0);
        assert_eq!(lines.vline_of_height(10.0), 0);
        assert_eq!(lines.vline_of_height(20.0), 1);
        assert_eq!(lines.vline_of_height(30.0), 1);
        assert_eq!(lines.vline_of_height(40.0), 2);
        assert_eq!(lines.vline_of_height(50.0), 2);
        assert_eq!(lines.vline_of_height(60.0), 3);
        assert_eq!(lines.vline_of_height(20000.0), 1000);
        assert_eq!(lines.vline_of_height(20001.0), 1000);
        // assert_eq!(lines.vline_of_height(21000.0), 1000);
    }

    #[test]
    fn test_utf8_len() {
        let attrs_list = default_attrs_list();

        // Single line without newline: "test" = 4 bytes
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("test", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();
        assert_eq!(lines.utf8_len(), 4);

        // Single line with newline: "test\n" = 5 bytes
        // Note: cosmic_text may not include trailing newline in utf8_len
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("test\n", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();
        // The actual utf8_len depends on how cosmic_text handles trailing newlines
        assert!(lines.utf8_len() >= 4);

        // Multiple lines - test total length
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("hello\nworld", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();
        // "hello\nworld" = 11 bytes (or 10 if newline not counted)
        assert!(lines.utf8_len() >= 10);
    }

    #[test]
    fn test_num_visual_lines() {
        let attrs_list = default_attrs_list();

        // Single line no wrap
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("test", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();
        assert_eq!(lines.num_visual_lines(), 1);

        // Two lines (one newline)
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("a\nb", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();
        assert_eq!(lines.num_visual_lines(), 2);

        // Three lines (two newlines)
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("a\nb\nc", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();
        assert_eq!(lines.num_visual_lines(), 3);
    }

    #[test]
    fn test_height_of_vline() {
        let attrs_list = default_attrs_list();
        let mut builder = TextLayoutLineBuilder::new();
        // Use wrapped text to ensure multiple visual lines
        let mut text_layout = TextLayout::new_with_text("test\n", attrs_list.clone(), None);
        text_layout.set_size(15.0, f32::MAX); // Force wrapping: "te\nst\n"
        for _ in 0..5 {
            builder.push_text_layout(&text_layout);
        }
        let lines = builder.build();

        // With line height of 20.0px, height_of_vline returns line_bottom
        let h0 = lines.height_of_vline(0);
        assert!(h0 > 0.0, "vline 0 should have positive height: {}", h0);

        // Heights should be monotonically increasing for sequential vlines
        if lines.num_visual_lines() > 1 {
            let h1 = lines.height_of_vline(1);
            assert!(
                h1 >= h0,
                "vline 1 height {} should be >= vline 0 height {}",
                h1,
                h0
            );
        }
    }

    #[test]
    fn test_single_line_no_wrap() {
        let attrs_list = default_attrs_list();

        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("hello", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        assert_eq!(lines.utf8_len(), 5);
        assert_eq!(lines.num_visual_lines(), 1);
        assert_eq!(lines.actual_line(0), 0);
        assert_eq!(lines.visual_line(0), 0);
        assert_eq!(lines.vline_of_offset(0), 0);
        assert_eq!(lines.vline_of_offset(4), 0);
        assert_eq!(lines.offset_of_vline(0), 0);
    }

    #[test]
    fn test_empty_lines() {
        let attrs_list = default_attrs_list();

        // Multiple consecutive newlines: "\n\n\n"
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("\n\n\n", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        // Should have multiple visual lines for empty lines
        assert!(
            lines.num_visual_lines() >= 3,
            "expected at least 3 visual lines, got {}",
            lines.num_visual_lines()
        );

        // First visual line should map to actual line 0
        assert_eq!(lines.actual_line(0), 0);
    }

    #[test]
    fn test_cursor_on_empty_lines() {
        let attrs_list = default_attrs_list();

        // Text with multiple consecutive empty lines: "a\n\n\nb"
        // Offset 0 = 'a', Offset 1 = '\n' (end of "a"), Offset 2 = '\n' (empty line 2), Offset 3 = '\n' (empty line 3), Offset 4 = 'b'
        let reference_layout = TextLayout::new_with_text(" ", attrs_list.clone(), None);
        let mut builder = TextLayoutLineBuilder::new();
        builder.set_default_from_layout(&reference_layout);
        let text_layout = TextLayout::new_with_text("a\n\n\nb", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        println!("num_visual_lines: {}", lines.num_visual_lines());
        println!("utf8_len: {}", lines.utf8_len());
        println!("default_glyph_height: {}", lines.default_glyph_height());
        println!("default_glyph_top: {}", lines.default_glyph_top());

        // Check cursor position at each offset
        for offset in 0..=5 {
            let point = lines.point_of_offset(offset);
            let glyph_height = point.glyph_bottom - point.glyph_top;
            println!(
                "offset {}: x={:.2}, line_top={:.2}, glyph_top={:.2}, glyph_bottom={:.2}, glyph_height={:.2}",
                offset, point.x, point.line_top, point.glyph_top, point.glyph_bottom, glyph_height
            );

            // The glyph_height should never be 0 - empty lines should use default metrics
            assert!(
                glyph_height > 0.0,
                "glyph_height should be > 0 at offset {}, but got {}",
                offset,
                glyph_height
            );
        }

        // Verify that each empty line has a distinct line_top (they should be stacked vertically)
        let p1 = lines.point_of_offset(1); // end of 'a' line
        let p2 = lines.point_of_offset(2); // empty line 2
        let p3 = lines.point_of_offset(3); // empty line 3
        let p4 = lines.point_of_offset(4); // 'b' line

        assert!(
            p2.line_top > p1.line_top,
            "line 2 should be below line 1: {} > {}",
            p2.line_top,
            p1.line_top
        );
        assert!(
            p3.line_top > p2.line_top,
            "line 3 should be below line 2: {} > {}",
            p3.line_top,
            p2.line_top
        );
        assert!(
            p4.line_top > p3.line_top,
            "line 4 should be below line 3: {} > {}",
            p4.line_top,
            p3.line_top
        );
    }

    #[test]
    fn test_unicode_multibyte() {
        let attrs_list = default_attrs_list();

        // "héllo" - é is 2 bytes in UTF-8
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("héllo", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();
        assert_eq!(lines.utf8_len(), 6); // h(1) + é(2) + l(1) + l(1) + o(1) = 6

        // "你好" - each Chinese character is 3 bytes
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("你好", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();
        assert_eq!(lines.utf8_len(), 6); // 2 chars * 3 bytes = 6

        // Emoji: "👋" is 4 bytes
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("👋", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();
        assert_eq!(lines.utf8_len(), 4);
    }

    #[test]
    fn test_roundtrip_vline_height() {
        let attrs_list = default_attrs_list();
        let mut builder = TextLayoutLineBuilder::new();
        // Use wrapped text to ensure multiple visual lines with distinct heights
        let mut text_layout = TextLayout::new_with_text("test\n", attrs_list.clone(), None);
        text_layout.set_size(15.0, f32::MAX);
        for _ in 0..10 {
            builder.push_text_layout(&text_layout);
        }
        let lines = builder.build();

        // Test that vline_of_height(0) returns 0
        assert_eq!(lines.vline_of_height(0.0), 0);

        // Test that vline_of_height for small height returns 0
        assert_eq!(lines.vline_of_height(10.0), 0);

        // Test that larger heights return larger vlines
        let vline_at_100 = lines.vline_of_height(100.0);
        let vline_at_200 = lines.vline_of_height(200.0);
        assert!(
            vline_at_200 >= vline_at_100,
            "vline at 200px ({}) should be >= vline at 100px ({})",
            vline_at_200,
            vline_at_100
        );
    }

    #[test]
    fn test_roundtrip_offset_vline() {
        let attrs_list = default_attrs_list();
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("hello\nworld\nfoo\nbar\n", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        // For each vline, offset_of_vline -> vline_of_offset should return same or adjacent vline
        for vline in 0..lines.num_visual_lines() {
            let offset = lines.offset_of_vline(vline);
            let recovered_vline = lines.vline_of_offset(offset);
            assert_eq!(
                recovered_vline, vline,
                "roundtrip failed for vline {}: offset={}, recovered={}",
                vline, offset, recovered_vline
            );
        }
    }

    #[test]
    fn test_boundary_conditions() {
        let attrs_list = default_attrs_list();
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("abc\ndef\n", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        // Offset 0 (start of text)
        assert_eq!(lines.vline_of_offset(0), 0);
        assert_eq!(lines.point_of_offset(0).line_top, 0.0);

        // Last valid offset
        let last_offset = lines.utf8_len();
        let last_vline = lines.vline_of_offset(last_offset);
        assert!(last_vline <= lines.num_visual_lines());

        // Beyond bounds should clamp
        let beyond_offset = lines.utf8_len() + 100;
        let clamped_vline = lines.vline_of_offset(beyond_offset);
        assert_eq!(clamped_vline, lines.vline_of_offset(lines.utf8_len()));
    }

    #[test]
    fn test_long_wrapped_line() {
        let attrs_list = default_attrs_list();
        let mut builder = TextLayoutLineBuilder::new();
        // Long text that will wrap multiple times at width 50
        let mut text_layout = TextLayout::new_with_text(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            attrs_list.clone(),
            None,
        );
        text_layout.set_size(50.0, f32::MAX);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        // Should have multiple visual lines from wrapping
        assert!(
            lines.num_visual_lines() > 1,
            "expected wrapping but got {} visual lines",
            lines.num_visual_lines()
        );

        // All visual lines should map to actual line 0 (no newlines)
        for vline in 0..lines.num_visual_lines() {
            assert_eq!(
                lines.actual_line(vline),
                0,
                "vline {} should map to actual line 0",
                vline
            );
        }

        // visual_line(0) should be 0 since there's only one actual line
        assert_eq!(lines.visual_line(0), 0);
    }

    #[test]
    fn test_visual_lines_iterator() {
        let attrs_list = default_attrs_list();
        let mut builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("abc\ndef\nghi", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        // Iterate over all lines
        let runs: Vec<_> = lines.visual_lines(0..lines.utf8_len()).collect();

        // Should have at least 3 runs (one per line)
        assert!(
            runs.len() >= 3,
            "expected at least 3 runs, got {}",
            runs.len()
        );

        // Check line indices are sequential
        for (i, run) in runs.iter().enumerate() {
            assert_eq!(run.line_i, i, "line_i mismatch at index {}", i);
        }

        // Line heights should be positive
        for run in &runs {
            assert!(run.line_height > 0.0, "line_height should be positive");
        }
    }

    #[test]
    fn test_mixed_content() {
        let attrs_list = default_attrs_list();
        let mut builder = TextLayoutLineBuilder::new();

        // Mix of short lines, empty lines, and longer content
        let text_layout = TextLayout::new_with_text("a\n\nbb\nccc\n\ndddd", attrs_list.clone(), None);
        builder.push_text_layout(&text_layout);
        let lines = builder.build();

        // Should have multiple visual lines
        assert!(
            lines.num_visual_lines() >= 6,
            "expected at least 6 visual lines for mixed content, got {}",
            lines.num_visual_lines()
        );

        // utf8_len should be reasonable
        assert!(
            lines.utf8_len() >= 10,
            "expected at least 10 bytes, got {}",
            lines.utf8_len()
        );
    }

    #[test]
    fn test_default_glyph_metrics_from_reference_layout() {
        let attrs_list = default_attrs_list();

        // Create a reference layout with a space to capture default font metrics
        let reference_layout = TextLayout::new_with_text(" ", attrs_list.clone(), None);

        let mut builder = TextLayoutLineBuilder::new();
        builder.set_default_from_layout(&reference_layout);
        let empty_lines = builder.build();

        // Default glyph height should be captured (not the fallback 16.0)
        assert!(
            empty_lines.default_glyph_height() > 0.0,
            "default_glyph_height should be positive, got {}",
            empty_lines.default_glyph_height()
        );
        assert!(
            empty_lines.default_glyph_height() != 16.0,
            "default_glyph_height should not be fallback 16.0, got {}",
            empty_lines.default_glyph_height()
        );

        // Default glyph top should be captured (centering offset)
        // With line height 20.0 and glyph height < 20.0, there should be some centering
        println!(
            "default_glyph_height: {}, default_glyph_top: {}",
            empty_lines.default_glyph_height(),
            empty_lines.default_glyph_top()
        );
    }

    #[test]
    fn test_cursor_position_matches_empty_vs_nonempty() {
        let attrs_list = default_attrs_list();

        // Create a reference layout with a space to capture default font metrics
        let reference_layout = TextLayout::new_with_text(" ", attrs_list.clone(), None);

        // Build empty text layouts with reference metrics
        let mut empty_builder = TextLayoutLineBuilder::new();
        empty_builder.set_default_from_layout(&reference_layout);
        let empty_lines = empty_builder.build();

        // Build text layouts with actual text
        let mut text_builder = TextLayoutLineBuilder::new();
        let text_layout = TextLayout::new_with_text("hello", attrs_list.clone(), None);
        text_builder.push_text_layout(&text_layout);
        let text_lines = text_builder.build();

        // Get point at offset 0 for text
        let text_point = text_lines.point_of_offset(0);

        // For empty text, we use default metrics
        let empty_glyph_top = empty_lines.default_glyph_top();
        let empty_glyph_height = empty_lines.default_glyph_height();

        // For text, we get metrics from point_of_offset
        let text_glyph_top = text_point.glyph_top;
        let text_glyph_height = text_point.glyph_bottom - text_point.glyph_top;

        println!(
            "Empty: glyph_top={}, glyph_height={}",
            empty_glyph_top, empty_glyph_height
        );
        println!(
            "Text:  glyph_top={}, glyph_height={}",
            text_glyph_top, text_glyph_height
        );

        // They should match exactly
        assert!(
            (empty_glyph_top - text_glyph_top).abs() < 0.001,
            "glyph_top mismatch: empty={}, text={}",
            empty_glyph_top,
            text_glyph_top
        );
        assert!(
            (empty_glyph_height - text_glyph_height).abs() < 0.001,
            "glyph_height mismatch: empty={}, text={}",
            empty_glyph_height,
            text_glyph_height
        );
    }

    #[test]
    fn test_cursor_position_matches_after_typing_and_deleting() {
        let attrs_list = default_attrs_list();

        // Simulate: start with empty, type "a", then delete back to empty
        // The cursor position should be the same before and after

        // Create reference layout for empty text
        let reference_layout = TextLayout::new_with_text(" ", attrs_list.clone(), None);

        // Empty state
        let mut empty_builder = TextLayoutLineBuilder::new();
        empty_builder.set_default_from_layout(&reference_layout);
        let empty_lines = empty_builder.build();

        // State with "a"
        let mut text_builder = TextLayoutLineBuilder::new();
        text_builder.set_default_from_layout(&reference_layout);
        let text_layout = TextLayout::new_with_text("a", attrs_list.clone(), None);
        text_builder.push_text_layout(&text_layout);
        let text_lines = text_builder.build();

        // Get cursor position for empty (offset 0)
        let empty_glyph_top = empty_lines.default_glyph_top();
        let empty_glyph_height = empty_lines.default_glyph_height();

        // Get cursor position for text at offset 0 (before the 'a')
        let text_point = text_lines.point_of_offset(0);
        let text_glyph_top = text_point.glyph_top;
        let text_glyph_height = text_point.glyph_bottom - text_point.glyph_top;

        println!("=== Cursor at offset 0 ===");
        println!(
            "Empty: glyph_top={:.4}, glyph_height={:.4}",
            empty_glyph_top, empty_glyph_height
        );
        println!(
            "Text:  glyph_top={:.4}, glyph_height={:.4}",
            text_glyph_top, text_glyph_height
        );

        // Cursor position should match
        assert!(
            (empty_glyph_top - text_glyph_top).abs() < 0.001,
            "glyph_top mismatch at offset 0: empty={}, text={}",
            empty_glyph_top,
            text_glyph_top
        );
        assert!(
            (empty_glyph_height - text_glyph_height).abs() < 0.001,
            "glyph_height mismatch at offset 0: empty={}, text={}",
            empty_glyph_height,
            text_glyph_height
        );
    }
}
