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

use crate::styled::ShadcnStyleExt;
use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

/// A styled skeleton (loading placeholder) builder.
///
/// Use the `width()`, `height()`, `circle()`, or `text()` convenience methods.
pub struct Skeleton {
    id: ViewId,
    width: Option<f64>,
    height: Option<f64>,
    border_radius: Option<f64>,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            width: None,
            height: None,
            border_radius: None,
        }
    }
    /// Create a skeleton suitable for a single line of text.
    pub fn text() -> Self {
        Self {
            id: ViewId::new(),
            width: None,
            height: Some(16.0),
            border_radius: Some(4.0),
        }
    }
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }
    pub fn radius(mut self, radius: f64) -> Self {
        self.border_radius = Some(radius);
        self
    }
    /// Create a circular skeleton (e.g. for avatars).
    pub fn circle(mut self, size: f64) -> Self {
        self.width = Some(size);
        self.height = Some(size);
        self.border_radius = Some(size / 2.0);
        self
    }

    pub fn build(self) -> impl IntoView {
        let width = self.width;
        let height = self.height;

        floem::views::Empty::new().style(move |s| {
            let mut style = s.rounded_sm();
            if let Some(w) = width {
                style = style.width(w);
            } else {
                style = style.w_full();
            }
            if let Some(h) = height {
                style = style.height(h);
            } else {
                style = style.h_5();
            }
            style.bg_muted()
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
