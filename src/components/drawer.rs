//! Drawer component with builder-style API
//!
//! Based on shadcn/ui Drawer - a slide-out panel from screen edges.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::drawer::*;
//!
//! let is_open = RwSignal::new(false);
//!
//! Drawer::new(is_open)
//!     .side(DrawerSide::Bottom)
//!     .content(drawer_content_view);
//! ```

use floem::prelude::*;
use floem::views::{Decorators, Overlay};
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// Side from which the drawer appears
#[derive(Clone, Copy, Default, PartialEq)]
pub enum DrawerSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

// ============================================================================
// Drawer
// ============================================================================

/// Slide-out panel from screen edge
pub struct Drawer<V> {
    id: ViewId,
    is_open: RwSignal<bool>,
    side: DrawerSide,
    content: Option<V>,
}

impl Drawer<()> {
    /// Create a new drawer
    pub fn new(is_open: RwSignal<bool>) -> Self { Self { id: ViewId::new(),
            is_open,
            side: DrawerSide::Bottom,
            content: None,
        }
    }
}

impl<V> Drawer<V> {
    /// Set the side from which the drawer appears
    pub fn side(mut self, side: DrawerSide) -> Self { self.side = side;
        self
    }

    /// Set the drawer content
    pub fn content<V2: IntoView + 'static>(self, content: V2) -> Drawer<V2> {
        Drawer {
            id: self.id,
            is_open: self.is_open,
            side: self.side,
            content: Some(content),
        }
    }
}


impl<V: IntoView + 'static> HasViewId for Drawer<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Drawer<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;
        let side = self.side;

        // Drawer handle (for top/bottom drawer)
        let handle = floem::views::Empty::new().style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                if side == DrawerSide::Bottom || side == DrawerSide::Top {
                    s.width(100.0)
                        .height(4.0)
                        .background(t.muted_foreground)
                        .border_radius(2.0)
                        .margin_top(8.0)
                        .margin_bottom(8.0)
                } else {
                    s.display(floem::style::Display::None)
                }
            })
        });

        // Content container
        let content_view = if let Some(content) = self.content {
            floem::views::Container::new(content).into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        // Drawer panel
        let drawer_panel = floem::views::Stack::vertical((handle, content_view)).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .absolute()
                    .background(t.background)
                    .border(1.0)
                    .border_color(t.border)
                    .z_index(10)
                    .items_center();
                match side {
                    DrawerSide::Bottom => base
                        .inset_bottom(0.0)
                        .inset_left(0.0)
                        .inset_right(0.0)
                        .min_height(200.0)
                        .max_height_pct(90.0f64)
                        .border_radius(t.radius)
                        .border_bottom(0.0),
                    DrawerSide::Top => base
                        .inset_top(0.0)
                        .inset_left(0.0)
                        .inset_right(0.0)
                        .min_height(200.0)
                        .max_height_pct(90.0f64)
                        .border_radius(t.radius)
                        .border_top(0.0),
                    DrawerSide::Left => base
                        .inset_top(0.0)
                        .inset_bottom(0.0)
                        .inset_left(0.0)
                        .min_width(300.0)
                        .max_width_pct(90.0f64)
                        .border_radius(t.radius)
                        .border_left(0.0),
                    DrawerSide::Right => base
                        .inset_top(0.0)
                        .inset_bottom(0.0)
                        .inset_right(0.0)
                        .min_width(300.0)
                        .max_width_pct(90.0f64)
                        .border_radius(t.radius)
                        .border_right(0.0),
                }
            })
        });

        // Backdrop
        let backdrop = floem::views::Empty::new()
            .style(move |s| {
                s.absolute()
                    .inset_0()
                    .background(floem::peniko::Color::from_rgba8(0, 0, 0, 128))
            })
            .on_click_stop(move |_| {
                is_open.set(false);
            });

        // Use Overlay with fixed positioning
        let drawer_overlay = Overlay::new(
            floem::views::Stack::new((backdrop, drawer_panel))
                .style(|s| s.width_full().height_full()),
        )
        .style(move |s| {
            let open = is_open.get();
            s.fixed()
                .inset_0()
                .width_full()
                .height_full()
                .apply_if(!open, |s| s.hide())
        });

        Box::new(drawer_overlay)
    }
}

// ============================================================================
// DrawerTrigger
// ============================================================================

/// Trigger to open a drawer
pub struct DrawerTrigger<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}

impl<V: IntoView + 'static> DrawerTrigger<V> {
    /// Create a new trigger
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self { Self { id: ViewId::new(), child, is_open }
    }
}


impl<V: IntoView + 'static> HasViewId for DrawerTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DrawerTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;

        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_click_stop(move |_| {
                    is_open.set(true);
                }),
        )
    }
}

// ============================================================================
// DrawerContent
// ============================================================================

/// Content container for drawer
pub struct DrawerContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DrawerContent<V> {
    /// Create new content
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for DrawerContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DrawerContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.width_full()
                .padding(16.0)
                .display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Column)
        }))
    }
}

// ============================================================================
// DrawerHeader
// ============================================================================

/// Header section of drawer
pub struct DrawerHeader<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DrawerHeader<V> {
    /// Create a new header
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for DrawerHeader<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DrawerHeader<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.width_full()
                .padding_bottom(16.0)
                .display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Column)
                .items_center()
        }))
    }
}

// ============================================================================
// DrawerTitle
// ============================================================================

/// Title for drawer
pub struct DrawerTitle {
    id: ViewId,
    text: String,
}

impl DrawerTitle {
    /// Create a new title
    pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() }
    }
}


impl HasViewId for DrawerTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for DrawerTitle {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;

        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(18.0)
                    .font_weight(floem::text::Weight::SEMIBOLD)
                    .color(t.foreground)
            })
        }))
    }
}

// ============================================================================
// DrawerDescription
// ============================================================================

/// Description for drawer
pub struct DrawerDescription {
    id: ViewId,
    text: String,
}

impl DrawerDescription {
    /// Create a new description
    pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() }
    }
}


impl HasViewId for DrawerDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for DrawerDescription {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;

        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(14.0).color(t.muted_foreground).margin_top(4.0)
            })
        }))
    }
}

// ============================================================================
// DrawerFooter
// ============================================================================

/// Footer section of drawer
pub struct DrawerFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DrawerFooter<V> {
    /// Create a new footer
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for DrawerFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DrawerFooter<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.width_full()
                .padding_top(16.0)
                .display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Column)
                .gap(8.0)
        }))
    }
}

// ============================================================================
// DrawerClose
// ============================================================================

/// Close button for drawer
pub struct DrawerClose<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}

impl<V: IntoView + 'static> DrawerClose<V> {
    /// Create a new close button
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self { Self { id: ViewId::new(), child, is_open }
    }
}


impl<V: IntoView + 'static> HasViewId for DrawerClose<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DrawerClose<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;

        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_click_stop(move |_| {
                    is_open.set(false);
                }),
        )
    }
}
