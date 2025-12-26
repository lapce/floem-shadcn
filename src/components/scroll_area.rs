//! Scroll Area component with builder-style API
//!
//! Based on shadcn/ui Scroll Area - a custom scrollable container.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::scroll_area::ScrollArea;
//!
//! // Vertical scroll
//! ScrollArea::new(Stack::vertical((
//!     label(|| "Item 1"),
//!     label(|| "Item 2"),
//!     // ... many items
//! ))).height(200.0);
//!
//! // Horizontal scroll
//! ScrollArea::new(Stack::horizontal((
//!     label(|| "Column 1"),
//!     label(|| "Column 2"),
//!     // ... many columns
//! ))).horizontal().width(300.0);
//! ```

use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// Scroll orientation
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ScrollOrientation {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

// ============================================================================
// ScrollArea
// ============================================================================

/// A styled scrollable container
pub struct ScrollArea<V> {
    id: ViewId,
    child: V,
    orientation: ScrollOrientation,
    width: Option<f64>,
    height: Option<f64>,
}

impl<V: IntoView + 'static> ScrollArea<V> {
    /// Create a new scroll area
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            orientation: ScrollOrientation::Vertical,
            width: None,
            height: None,
        }
    }

    /// Set scroll orientation
    pub fn orientation(mut self, orientation: ScrollOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set to horizontal scrolling
    pub fn horizontal(mut self) -> Self {
        self.orientation = ScrollOrientation::Horizontal;
        self
    }

    /// Set to vertical scrolling
    pub fn vertical(mut self) -> Self {
        self.orientation = ScrollOrientation::Vertical;
        self
    }

    /// Set to both horizontal and vertical scrolling
    pub fn both(mut self) -> Self {
        self.orientation = ScrollOrientation::Both;
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Set fixed height
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }
}

impl<V: IntoView + 'static> HasViewId for ScrollArea<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for ScrollArea<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let orientation = self.orientation;
        let width = self.width;
        let height = self.height;

        // Wrap content in appropriate scroll container
        let scroll_view = match orientation {
            ScrollOrientation::Vertical => floem::views::Scroll::new(self.child)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let base = s.border_radius(t.radius).flex_grow(1.0).flex_basis(0.0);

                        match (width, height) {
                            (Some(w), Some(h)) => base.width(w).height(h),
                            (Some(w), None) => base.width(w),
                            (None, Some(h)) => base.height(h),
                            (None, None) => base,
                        }
                    })
                })
                .into_any(),
            ScrollOrientation::Horizontal => floem::views::Scroll::new(self.child)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let base = s.border_radius(t.radius).flex_grow(1.0).flex_basis(0.0);

                        match (width, height) {
                            (Some(w), Some(h)) => base.width(w).height(h),
                            (Some(w), None) => base.width(w),
                            (None, Some(h)) => base.height(h),
                            (None, None) => base,
                        }
                    })
                })
                .into_any(),
            ScrollOrientation::Both => floem::views::Scroll::new(self.child)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let base = s.border_radius(t.radius).flex_grow(1.0).flex_basis(0.0);

                        match (width, height) {
                            (Some(w), Some(h)) => base.width(w).height(h),
                            (Some(w), None) => base.width(w),
                            (None, Some(h)) => base.height(h),
                            (None, None) => base,
                        }
                    })
                })
                .into_any(),
        };

        Box::new(scroll_view)
    }
}

// ============================================================================
// ScrollAreaWithBar
// ============================================================================

/// Scroll area with visible scrollbar styling
pub struct ScrollAreaWithBar<V> {
    id: ViewId,
    child: V,
    height: Option<f64>,
    width: Option<f64>,
    show_scrollbar: bool,
}

impl<V: IntoView + 'static> ScrollAreaWithBar<V> {
    /// Create a new scroll area with visible scrollbar
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            height: None,
            width: None,
            show_scrollbar: true,
        }
    }

    /// Set fixed height
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Toggle scrollbar visibility
    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }
}

impl<V: IntoView + 'static> HasViewId for ScrollAreaWithBar<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for ScrollAreaWithBar<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let height = self.height;
        let width = self.width;

        Box::new(floem::views::Scroll::new(self.child).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius)
                    .flex_grow(1.0)
                    .flex_basis(0.0);

                match (width, height) {
                    (Some(w), Some(h)) => base.width(w).height(h),
                    (Some(w), None) => base.width(w),
                    (None, Some(h)) => base.height(h),
                    (None, None) => base,
                }
            })
        }))
    }
}

// ============================================================================
// VirtualList helper
// ============================================================================

/// A virtualized list for large datasets (uses floem's virtual_stack)
pub struct VirtualScrollArea;

impl VirtualScrollArea {
    /// Create a hint for using floem's virtual_stack
    /// Note: Virtual scrolling requires using floem's virtual_stack directly
    /// with an im::Vector and a closure for rendering items.
    ///
    /// Example:
    /// ```rust
    /// use floem::views::virtual_stack;
    /// use im::Vector;
    ///
    /// let items: Vector<String> = (0..10000)
    ///     .map(|i| format!("Item {}", i))
    ///     .collect();
    ///
    /// virtual_stack(
    ///     floem::views::VirtualDirection::Vertical,
    ///     floem::views::VirtualItemSize::Fixed(Box::new(|| 32.0)),
    ///     move || items.clone(),
    ///     move |item| item.clone(),
    ///     move |item| label(move || item.clone()),
    /// )
    /// ```
    pub fn usage_hint() -> &'static str {
        "Use floem::views::virtual_stack for virtualized scrolling of large lists"
    }
}
