//! Shared style utilities for text editing components.
//!
//! This module provides common functions for extracting styles from floem's
//! style system and applying them to text documents.

use floem::{
    peniko::Color,
    style::BuiltinStyle,
    text::{FamilyOwned, LineHeightValue, Weight},
    unit::PxPct,
};

use super::{Document, CURSOR_BLINK_INTERVAL_MS};

/// Padding values (top, right, bottom, left)
pub type Padding = (f64, f64, f64, f64);

/// Extracts padding values from a style, resolving percentages based on width.
pub fn extract_padding(style: &BuiltinStyle<'_>, layout_width: f64) -> Padding {
    let padding_left = match style.padding_left() {
        PxPct::Px(padding) => padding,
        PxPct::Pct(pct) => (pct / 100.) * layout_width,
    };
    let padding_right = match style.padding_right() {
        PxPct::Px(padding) => padding,
        PxPct::Pct(pct) => (pct / 100.) * layout_width,
    };
    let padding_top = match style.padding_top() {
        PxPct::Px(padding) => padding,
        PxPct::Pct(pct) => (pct / 100.) * layout_width,
    };
    let padding_bottom = match style.padding_bottom() {
        PxPct::Px(padding) => padding,
        PxPct::Pct(pct) => (pct / 100.) * layout_width,
    };
    (padding_top, padding_right, padding_bottom, padding_left)
}

/// Text styling properties extracted from a style.
#[derive(Clone, Debug, PartialEq)]
pub struct TextStyles {
    pub text_color: Color,
    pub font_size: f32,
    pub line_height: LineHeightValue,
    pub font_weight: Weight,
    pub font_family: Vec<FamilyOwned>,
}

impl Default for TextStyles {
    fn default() -> Self {
        Self {
            text_color: Color::BLACK,
            font_size: 14.0,
            line_height: LineHeightValue::Normal(1.5),
            font_weight: Weight::NORMAL,
            font_family: Vec::new(),
        }
    }
}

/// Extracts text styling properties from a style.
pub fn extract_text_styles(style: &BuiltinStyle<'_>) -> TextStyles {
    TextStyles {
        text_color: style.color().unwrap_or(Color::BLACK),
        font_size: style.font_size().unwrap_or(14.0),
        line_height: style.line_height().unwrap_or(LineHeightValue::Normal(1.5)),
        font_weight: style.font_weight().unwrap_or(Weight::NORMAL),
        font_family: style
            .font_family()
            .map(|f| vec![FamilyOwned::Name(f)])
            .unwrap_or_default(),
    }
}

/// Applies text styles to a document.
pub fn apply_styles_to_document(doc: &Document, styles: &TextStyles) {
    doc.set_text_color(styles.text_color);
    doc.set_font_size(styles.font_size);
    doc.set_line_height(styles.line_height);
    doc.set_font_weight(styles.font_weight);
    doc.set_font_family(styles.font_family.clone());
}

/// Calculates whether the cursor should be visible based on blink timing.
///
/// Returns `true` during even intervals (0-500ms, 1000-1500ms, etc.)
/// and `false` during odd intervals (500-1000ms, 1500-2000ms, etc.).
pub fn is_cursor_visible(elapsed_ms: u128) -> bool {
    let blink_cycle = elapsed_ms / CURSOR_BLINK_INTERVAL_MS as u128;
    blink_cycle.is_multiple_of(2)
}

/// Gets cursor or glyph dimensions, falling back to default font metrics.
///
/// Returns (top, height) for the cursor/glyph at the given position.
pub fn get_glyph_dimensions(
    glyph_top: f64,
    glyph_bottom: f64,
    default_top: f64,
    default_height: f64,
) -> (f64, f64) {
    if glyph_bottom > glyph_top {
        (glyph_top, glyph_bottom - glyph_top)
    } else {
        (default_top, default_height)
    }
}
