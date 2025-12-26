//! Carousel component with builder-style API
//!
//! Based on shadcn/ui Carousel - a carousel for cycling through content.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::carousel::*;
//!
//! let current = RwSignal::new(0);
//!
//! Carousel::new(current, 3)
//!     .items(content_view);
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// Carousel orientation
#[derive(Clone, Copy, Default, PartialEq)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

// ============================================================================
// Carousel
// ============================================================================

/// A carousel for cycling through items
pub struct Carousel<I> {
    id: ViewId,
    current: RwSignal<usize>,
    total: usize,
    items: I,
    orientation: CarouselOrientation,
    show_arrows: bool,
}

impl Carousel<()> {
    /// Create a new carousel
    pub fn new(current: RwSignal<usize>, total: usize) -> Self {
        Self {
            id: ViewId::new(),
            current,
            total,
            items: (),
            orientation: CarouselOrientation::Horizontal,
            show_arrows: true,
        }
    }
}

impl<I> Carousel<I> {
    /// Set carousel items
    pub fn items<I2: IntoView + 'static>(self, items: I2) -> Carousel<I2> {
        Carousel {
            id: self.id,
            current: self.current,
            total: self.total,
            items,
            orientation: self.orientation,
            show_arrows: self.show_arrows,
        }
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: CarouselOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set horizontal orientation
    pub fn horizontal(mut self) -> Self {
        self.orientation = CarouselOrientation::Horizontal;
        self
    }

    /// Set vertical orientation
    pub fn vertical(mut self) -> Self {
        self.orientation = CarouselOrientation::Vertical;
        self
    }

    /// Show/hide navigation arrows
    pub fn arrows(mut self, show: bool) -> Self {
        self.show_arrows = show;
        self
    }
}

impl<I: IntoView + 'static> HasViewId for Carousel<I> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<I: IntoView + 'static> IntoView for Carousel<I> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let current = self.current;
        let total = self.total;
        let orientation = self.orientation;
        let show_arrows = self.show_arrows;

        // Previous button
        let prev_button = if show_arrows {
            floem::views::Label::new(match orientation {
                CarouselOrientation::Horizontal => "<",
                CarouselOrientation::Vertical => "^",
            })
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width(40.0)
                        .height(40.0)
                        .font_size(18.0)
                        .color(t.foreground)
                        .background(t.background)
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(t.radius)
                        .cursor(CursorStyle::Pointer)
                        .display(floem::style::Display::Flex)
                        .items_center()
                        .justify_center()
                        .hover(|s| s.background(t.accent))
                })
            })
            .on_click_stop(move |_| {
                current.update(|c| {
                    if *c > 0 {
                        *c -= 1;
                    } else {
                        *c = total.saturating_sub(1);
                    }
                });
            })
            .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        // Next button
        let next_button = if show_arrows {
            floem::views::Label::new(match orientation {
                CarouselOrientation::Horizontal => ">",
                CarouselOrientation::Vertical => "v",
            })
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width(40.0)
                        .height(40.0)
                        .font_size(18.0)
                        .color(t.foreground)
                        .background(t.background)
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(t.radius)
                        .cursor(CursorStyle::Pointer)
                        .display(floem::style::Display::Flex)
                        .items_center()
                        .justify_center()
                        .hover(|s| s.background(t.accent))
                })
            })
            .on_click_stop(move |_| {
                current.update(|c| {
                    if *c < total.saturating_sub(1) {
                        *c += 1;
                    } else {
                        *c = 0;
                    }
                });
            })
            .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        // Content area
        let content = floem::views::Container::new(self.items).style(|s| {
            s.flex_grow(1.0)
                .display(floem::style::Display::Flex)
                .items_center()
                .justify_center()
        });

        // Layout based on orientation
        let carousel_body = match orientation {
            CarouselOrientation::Horizontal => {
                floem::views::Stack::horizontal((prev_button, content, next_button))
                    .style(|s| s.width_full().items_center().gap(8.0))
                    .into_any()
            }
            CarouselOrientation::Vertical => {
                floem::views::Stack::vertical((prev_button, content, next_button))
                    .style(|s| s.height_full().items_center().gap(8.0))
                    .into_any()
            }
        };

        Box::new(carousel_body)
    }
}

// ============================================================================
// CarouselItem
// ============================================================================

/// Individual carousel slide
pub struct CarouselItem<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CarouselItem<V> {
    /// Create a new carousel item
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CarouselItem<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CarouselItem<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.width_full()
                .display(floem::style::Display::Flex)
                .items_center()
                .justify_center()
        }))
    }
}

// ============================================================================
// CarouselContent - Shows content based on current index
// ============================================================================

/// Container that shows content based on current index using visibility
pub struct CarouselContent;

impl CarouselContent {
    /// Usage hint: Use with dyn_container for dynamic content switching
    /// Example:
    /// ```rust
    /// let items = vec!["Slide 1", "Slide 2", "Slide 3"];
    /// let current = RwSignal::new(0);
    ///
    /// // Show content based on current index
    /// Label::reactive(move || items[current.get()].to_string())
    /// ```
    pub fn usage_hint() -> &'static str {
        "Use Label::reactive or conditional rendering based on current signal"
    }
}

// ============================================================================
// CarouselPrevious / CarouselNext (standalone buttons)
// ============================================================================

/// Previous button for carousel
pub struct CarouselPrevious {
    id: ViewId,
    current: RwSignal<usize>,
    total: usize,
    wrap: bool,
}

impl CarouselPrevious {
    /// Create a previous button
    pub fn new(current: RwSignal<usize>, total: usize) -> Self {
        Self {
            id: ViewId::new(),
            current,
            total,
            wrap: true,
        }
    }

    /// Enable/disable wrapping
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
}

impl HasViewId for CarouselPrevious {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CarouselPrevious {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let current = self.current;
        let total = self.total;
        let wrap = self.wrap;

        Box::new(
            floem::views::Label::new("<")
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.width(40.0)
                            .height(40.0)
                            .font_size(18.0)
                            .color(t.foreground)
                            .background(t.background)
                            .border(1.0)
                            .border_color(t.border)
                            .border_radius(20.0)
                            .cursor(CursorStyle::Pointer)
                            .display(floem::style::Display::Flex)
                            .items_center()
                            .justify_center()
                            .hover(|s| s.background(t.accent))
                    })
                })
                .on_click_stop(move |_| {
                    current.update(|c| {
                        if *c > 0 {
                            *c -= 1;
                        } else if wrap {
                            *c = total.saturating_sub(1);
                        }
                    });
                }),
        )
    }
}

/// Next button for carousel
pub struct CarouselNext {
    id: ViewId,
    current: RwSignal<usize>,
    total: usize,
    wrap: bool,
}

impl CarouselNext {
    /// Create a next button
    pub fn new(current: RwSignal<usize>, total: usize) -> Self {
        Self {
            id: ViewId::new(),
            current,
            total,
            wrap: true,
        }
    }

    /// Enable/disable wrapping
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
}

impl HasViewId for CarouselNext {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CarouselNext {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let current = self.current;
        let total = self.total;
        let wrap = self.wrap;

        Box::new(
            floem::views::Label::new(">")
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.width(40.0)
                            .height(40.0)
                            .font_size(18.0)
                            .color(t.foreground)
                            .background(t.background)
                            .border(1.0)
                            .border_color(t.border)
                            .border_radius(20.0)
                            .cursor(CursorStyle::Pointer)
                            .display(floem::style::Display::Flex)
                            .items_center()
                            .justify_center()
                            .hover(|s| s.background(t.accent))
                    })
                })
                .on_click_stop(move |_| {
                    current.update(|c| {
                        if *c < total.saturating_sub(1) {
                            *c += 1;
                        } else if wrap {
                            *c = 0;
                        }
                    });
                }),
        )
    }
}
