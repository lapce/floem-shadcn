//! Aspect Ratio component with builder-style API
//!
//! Based on shadcn/ui Aspect Ratio - displays content within a desired ratio.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::aspect_ratio::AspectRatio;
//!
//! // 16:9 aspect ratio
//! AspectRatio::new(16.0 / 9.0)
//!     .child(|| img("photo.jpg"));
//!
//! // Square (1:1)
//! AspectRatio::square()
//!     .child(|| avatar);
//!
//! // Video (16:9)
//! AspectRatio::video()
//!     .child(|| video_player);
//! ```

use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

// ============================================================================
// AspectRatio
// ============================================================================

/// Container that maintains a specific aspect ratio
pub struct AspectRatio<C> {
    id: ViewId,
    ratio: f64,
    child: Option<C>,
}

impl AspectRatio<()> {
    /// Create a new aspect ratio container with specified ratio (width/height)
    pub fn new(ratio: f64) -> Self {
        Self {
            id: ViewId::new(),
            ratio,
            child: None,
        }
    }

    /// Create a square (1:1) aspect ratio
    pub fn square() -> Self {
        Self::new(1.0)
    }

    /// Create a video (16:9) aspect ratio
    pub fn video() -> Self {
        Self::new(16.0 / 9.0)
    }

    /// Create a widescreen (21:9) aspect ratio
    pub fn widescreen() -> Self {
        Self::new(21.0 / 9.0)
    }

    /// Create a portrait (3:4) aspect ratio
    pub fn portrait() -> Self {
        Self::new(3.0 / 4.0)
    }

    /// Create a photo (4:3) aspect ratio
    pub fn photo() -> Self {
        Self::new(4.0 / 3.0)
    }
}

impl<C> AspectRatio<C> {
    /// Set the child content
    pub fn child<C2: Fn() -> V, V: IntoView + 'static>(self, child: C2) -> AspectRatio<C2> {
        AspectRatio {
            id: self.id,
            ratio: self.ratio,
            child: Some(child),
        }
    }

    /// Change the aspect ratio
    pub fn ratio(mut self, ratio: f64) -> Self {
        self.ratio = ratio;
        self
    }
}

impl<C, V> AspectRatio<C>
where
    C: Fn() -> V + 'static,
    V: IntoView + 'static,
{
    /// Build the aspect ratio view
    pub fn build(self) -> impl IntoView {
        let ratio = self.ratio;
        let child = self.child;

        // The padding-bottom trick to maintain aspect ratio
        // Since we can't use padding-bottom percentage in floem directly,
        // we use a fixed approach with aspect_ratio style if available,
        // or approximate with min-height based on width

        let inner = if let Some(child_fn) = child {
            floem::views::Container::new(child_fn())
                .style(|s| s.position(floem::style::Position::Absolute).inset(0.0))
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        floem::views::Container::new(inner).style(move |s| {
            s.position(floem::style::Position::Relative)
                .width_full()
                // Use aspect_ratio style property
                .aspect_ratio(ratio as f32)
        })
    }
}

impl<C, V> HasViewId for AspectRatio<C>
where
    C: Fn() -> V + 'static,
    V: IntoView + 'static,
{
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<C, V> IntoView for AspectRatio<C>
where
    C: Fn() -> V + 'static,
    V: IntoView + 'static,
{
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// ============================================================================
// AspectRatioSimple - For direct child without closure
// ============================================================================

/// Simple aspect ratio container that takes a direct child
pub struct AspectRatioSimple<V> {
    id: ViewId,
    ratio: f64,
    child: V,
}

impl<V: IntoView + 'static> AspectRatioSimple<V> {
    /// Create a new aspect ratio container
    pub fn new(ratio: f64, child: V) -> Self {
        Self {
            id: ViewId::new(),
            ratio,
            child,
        }
    }

    /// Create a square (1:1) aspect ratio
    pub fn square(child: V) -> Self {
        Self::new(1.0, child)
    }

    /// Create a video (16:9) aspect ratio
    pub fn video(child: V) -> Self {
        Self::new(16.0 / 9.0, child)
    }

    /// Change the aspect ratio
    pub fn ratio(mut self, ratio: f64) -> Self {
        self.ratio = ratio;
        self
    }
}

impl<V: IntoView + 'static> HasViewId for AspectRatioSimple<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for AspectRatioSimple<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let ratio = self.ratio;

        let inner = floem::views::Container::new(self.child)
            .style(|s| s.position(floem::style::Position::Absolute).inset(0.0));

        Box::new(floem::views::Container::new(inner).style(move |s| {
            s.position(floem::style::Position::Relative)
                .width_full()
                .aspect_ratio(ratio as f32)
        }))
    }
}
