//! Spinner component – animated loading indicator
//!
//! Based on shadcn/ui Spinner.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::spinner::*;
//!
//! let spinner = Spinner::new().size(SpinnerSize::Md).color(SpinnerColor::Primary);
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

/// Spinner sizes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpinnerSize {
    /// Small spinner (16×16).
    Sm,
    /// Medium spinner (20×20).
    #[default]
    Md,
    /// Large spinner (24×24).
    Lg,
}

/// Spinner color variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpinnerColor {
    /// Uses the foreground colour.
    #[default]
    Default,
    /// Primary theme colour.
    Primary,
    /// Secondary theme colour.
    Secondary,
    /// Destructive colour.
    Destructive,
}

/// A loading spinner with configurable size and colour.
pub struct Spinner {
    id: ViewId,
    size: SpinnerSize,
    color: SpinnerColor,
}

impl Spinner {
    /// Create a new spinner (default: Md, Default colour).
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            size: SpinnerSize::default(),
            color: SpinnerColor::default(),
        }
    }

    /// Set the size variant.
    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    /// Set the colour variant.
    pub fn color(mut self, color: SpinnerColor) -> Self {
        self.color = color;
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for Spinner {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Spinner {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let size = self.size;
        let color = self.color;

        let (width, height) = match size {
            SpinnerSize::Sm => (16.0, 16.0),
            SpinnerSize::Md => (20.0, 20.0),
            SpinnerSize::Lg => (24.0, 24.0),
        };

        Box::new(
            floem::views::svg(move || SPINNER_SVG.to_string()).style(move |s| {
                s.width(width)
                    .height(height)
                    .with_shadcn_theme(move |s, t| {
                        let c = match color {
                            SpinnerColor::Default => t.foreground,
                            SpinnerColor::Primary => t.primary,
                            SpinnerColor::Secondary => t.secondary,
                            SpinnerColor::Destructive => t.destructive,
                        };
                        s.color(c)
                    })
            }),
        )
    }
}

const SPINNER_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"#;
