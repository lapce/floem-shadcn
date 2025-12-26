//! Resizable component with builder-style API
//!
//! Based on shadcn/ui Resizable - resizable panel groups with drag handles.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::resizable::*;
//!
//! let size = RwSignal::new(50.0); // percentage
//!
//! ResizablePanelGroup::horizontal((
//!     ResizablePanel::new(panel1_content).default_size(30.0),
//!     ResizableHandle::new(),
//!     ResizablePanel::new(panel2_content).default_size(70.0),
//! ));
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::RwSignal;
use floem::style::CursorStyle;
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

/// Direction of the resizable panel group
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ResizableDirection {
    #[default]
    Horizontal,
    Vertical,
}

// ============================================================================
// ResizablePanelGroup
// ============================================================================

/// Container for resizable panels
pub struct ResizablePanelGroup<V> {
    id: ViewId,
    direction: ResizableDirection,
    child: V,
}

impl<V: IntoView + 'static> ResizablePanelGroup<V> {
    /// Create a horizontal panel group
    pub fn horizontal(child: V) -> Self { Self { id: ViewId::new(),
            direction: ResizableDirection::Horizontal,
            child,
        }
    }

    /// Create a vertical panel group
    pub fn vertical(child: V) -> Self { Self { id: ViewId::new(),
            direction: ResizableDirection::Vertical,
            child,
        }
    }

    /// Set the direction
    pub fn direction(mut self, direction: ResizableDirection) -> Self { self.direction = direction;
        self
    }
}


impl<V: IntoView + 'static> HasViewId for ResizablePanelGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}


impl<V: IntoView + 'static> HasViewId for ResizablePanel<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for ResizablePanelGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let direction = self.direction;

        Box::new(floem::views::Container::with_id(self.id, self.child).style(move |s| {
            let base = s
                .width_full()
                .height_full()
                .display(floem::style::Display::Flex);

            match direction {
                ResizableDirection::Horizontal => {
                    base.flex_direction(floem::style::FlexDirection::Row)
                }
                ResizableDirection::Vertical => {
                    base.flex_direction(floem::style::FlexDirection::Column)
                }
            }
        }))
    }
}

// ============================================================================
// ResizablePanel
// ============================================================================

/// Individual resizable panel
pub struct ResizablePanel<V> {
    id: ViewId,
    child: V,
    default_size: Option<f64>,
    min_size: Option<f64>,
    max_size: Option<f64>,
    size_signal: Option<RwSignal<f64>>,
}

impl<V: IntoView + 'static> ResizablePanel<V> {
    /// Create a new resizable panel
    pub fn new(child: V) -> Self { Self { id: ViewId::new(),
            child,
            default_size: None,
            min_size: None,
            max_size: None,
            size_signal: None,
        }
    }

    /// Set default size (percentage 0-100)
    pub fn default_size(mut self, size: f64) -> Self { self.default_size = Some(size);
        self
    }

    /// Set minimum size (percentage)
    pub fn min_size(mut self, size: f64) -> Self { self.min_size = Some(size);
        self
    }

    /// Set maximum size (percentage)
    pub fn max_size(mut self, size: f64) -> Self { self.max_size = Some(size);
        self
    }

    /// Connect to a size signal for controlled sizing
    pub fn size(mut self, signal: RwSignal<f64>) -> Self { self.size_signal = Some(signal);
        self
    }
}

impl<V: IntoView + 'static> IntoView for ResizablePanel<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let default_size = self.default_size.unwrap_or(50.0);
        let min_size = self.min_size;
        let max_size = self.max_size;
        let size_signal = self.size_signal;

        Box::new(floem::views::Container::with_id(self.id, self.child).style(move |s| {
            let size = size_signal.map(|sig| sig.get()).unwrap_or(default_size);

            let base = s
                .flex_basis(floem::unit::PxPctAuto::Pct(size))
                .flex_grow(0.0)
                .flex_shrink(0.0);

            let with_min = if let Some(min) = min_size {
                base.min_width_pct(min)
            } else {
                base
            };

            if let Some(max) = max_size {
                with_min.max_width_pct(max)
            } else {
                with_min
            }
        }))
    }
}

// ============================================================================
// ResizableHandle
// ============================================================================

/// Drag handle between panels
pub struct ResizableHandle {
    id: ViewId,
    direction: ResizableDirection,
    with_handle: bool,
}

impl ResizableHandle {
    /// Create a new resize handle
    pub fn new() -> Self { Self { id: ViewId::new(),
            direction: ResizableDirection::Horizontal,
            with_handle: false,
        }
    }

    /// Set the direction (determines cursor and styling)
    pub fn direction(mut self, direction: ResizableDirection) -> Self { self.direction = direction;
        self
    }

    /// Show a visual handle indicator
    pub fn with_handle(mut self) -> Self { self.with_handle = true;
        self
    }
}

impl Default for ResizableHandle {
    fn default() -> Self {
        Self::new()
    }
}


impl HasViewId for ResizableHandle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ResizableHandle {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let direction = self.direction;
        let with_handle = self.with_handle;

        // Handle indicator (dots)
        let handle_indicator = if with_handle {
            floem::views::Label::new("⋮⋮")
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| s.font_size(10.0).color(t.muted_foreground))
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        Box::new(
            floem::views::Container::new(handle_indicator).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let base = s
                        .background(t.border)
                        .display(floem::style::Display::Flex)
                        .items_center()
                        .justify_center();
                    match direction {
                        ResizableDirection::Horizontal => base
                            .width(4.0)
                            .height_full()
                            .cursor(CursorStyle::ColResize)
                            .hover(|s| s.background(t.primary)),
                        ResizableDirection::Vertical => base
                            .width_full()
                            .height(4.0)
                            .cursor(CursorStyle::RowResize)
                            .hover(|s| s.background(t.primary)),
                    }
                })
            }),
        )
    }
}

// ============================================================================
// Helpers
// ============================================================================

/// Create a horizontal resizable group
pub fn resizable_horizontal<V: IntoView + 'static>(child: V) -> ResizablePanelGroup<V> {
    ResizablePanelGroup::horizontal(child)
}

/// Create a vertical resizable group
pub fn resizable_vertical<V: IntoView + 'static>(child: V) -> ResizablePanelGroup<V> {
    ResizablePanelGroup::vertical(child)
}
