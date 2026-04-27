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
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::styled::ShadcnStyleExt;

/// A styled avatar builder.
///
/// Displays initials as a fallback when no image is provided. The size can be
/// customised with `size()`.
pub struct Avatar {
    id: ViewId,
    fallback_text: Option<String>,
    size: f64,
}

impl Avatar {
    /// Create a new avatar with default size (40px).
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            fallback_text: None,
            size: 40.0,
        }
    }

    /// Set the fallback text (e.g. initials).
    pub fn fallback(mut self, text: impl Into<String>) -> Self {
        self.fallback_text = Some(text.into());
        self
    }

    /// Set the size (width and height) in pixels.
    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    /// Build the avatar view.
    pub fn build(self) -> impl IntoView {
        let size = self.size;
        let fallback = self.fallback_text.unwrap_or_default();
        let font_size = size * 0.4;

        floem::views::Container::new(
            floem::views::Label::new(fallback)
                .style(move |s| s.font_size(font_size).font_medium().text_muted_foreground()),
        )
        .style(move |s| {
            s.width(size)
                .height(size)
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .bg_muted()
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
