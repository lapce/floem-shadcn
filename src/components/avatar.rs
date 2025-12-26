//! Avatar component with builder-style API
//!
//! Based on shadcn/ui Avatar - displays a user image with fallback.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::avatar::Avatar;
//!
//! // Avatar with initials fallback
//! let avatar = Avatar::new().fallback("JD");
//!
//! // Avatar with custom size
//! let avatar = Avatar::new().fallback("AB").size(48.0);
//! ```

use floem::prelude::*;
use floem::text::Weight;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// A styled avatar builder
pub struct Avatar {
    id: ViewId,
    fallback_text: Option<String>,
    size: f64,
}

impl Avatar {
    /// Create a new avatar
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            fallback_text: None,
            size: 40.0,
        }
    }

    /// Set the fallback text (usually initials)
    pub fn fallback(mut self, text: impl Into<String>) -> Self {
        self.fallback_text = Some(text.into());
        self
    }

    /// Set the avatar size (default: 40.0)
    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    /// Build the avatar view
    pub fn build(self) -> impl IntoView {
        let size = self.size;
        let fallback = self.fallback_text.unwrap_or_default();
        let font_size = size * 0.4;

        floem::views::Container::new(floem::views::Label::new(fallback).style(move |s| {
            s.font_size(font_size)
                .font_weight(Weight::MEDIUM)
                .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
        }))
        .style(move |s| {
            s.width(size)
                .height(size)
                .border_radius(size / 2.0) // Circular
                .display(floem::style::Display::Flex)
                .items_center()
                .justify_center()
                .with_shadcn_theme(|s, t| s.background(t.muted))
        })
    }
}

impl Default for Avatar {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for Avatar {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Avatar {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
