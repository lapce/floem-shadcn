//! Skeleton component with builder-style API
//!
//! Based on shadcn/ui Skeleton - a loading placeholder.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::skeleton::Skeleton;
//!
//! // Basic skeleton
//! let skeleton = Skeleton::new().width(200.0).height(20.0);
//!
//! // Circular skeleton (for avatars)
//! let skeleton = Skeleton::new().circle(40.0);
//!
//! // Text line skeleton
//! let skeleton = Skeleton::text();
//! ```

use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// A styled skeleton (loading placeholder) builder
pub struct Skeleton {
    id: ViewId,
    width: Option<f64>,
    height: Option<f64>,
    border_radius: Option<f64>,
}

impl Skeleton {
    /// Create a new skeleton
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            width: None,
            height: None,
            border_radius: None,
        }
    }

    /// Create a text-line skeleton (full width, standard text height)
    pub fn text() -> Self {
        Self {
            id: ViewId::new(),
            width: None,
            height: Some(16.0),
            border_radius: Some(4.0),
        }
    }

    /// Set the width
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the border radius
    pub fn radius(mut self, radius: f64) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Make a circular skeleton (for avatar placeholders)
    pub fn circle(mut self, size: f64) -> Self {
        self.width = Some(size);
        self.height = Some(size);
        self.border_radius = Some(size / 2.0);
        self
    }

    /// Build the skeleton view
    pub fn build(self) -> impl IntoView {
        let width = self.width;
        let height = self.height;
        let border_radius = self.border_radius.unwrap_or(4.0);

        floem::views::Empty::new().style(move |s| {
            let mut style = s.border_radius(border_radius);

            if let Some(w) = width {
                style = style.width(w);
            } else {
                style = style.width_full();
            }

            if let Some(h) = height {
                style = style.height(h);
            } else {
                style = style.height(20.0);
            }

            style.with_shadcn_theme(|s, t| s.background(t.muted))
        })
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for Skeleton {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Skeleton {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
