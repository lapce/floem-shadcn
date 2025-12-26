//! Sheet component with builder-style API
//!
//! Based on shadcn/ui Sheet - a slide-out side panel overlay.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::sheet::{Sheet, SheetContent, SheetSide};
//!
//! let open = RwSignal::new(false);
//!
//! Sheet::new(open, SheetContent::new(
//!     v_stack((
//!         label(|| "Sheet Title"),
//!         label(|| "Sheet content goes here..."),
//!     ))
//! ).side(SheetSide::Right));
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::{Decorators, Overlay};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// Which side the sheet slides in from
#[derive(Clone, Copy, Default)]
pub enum SheetSide {
    Top,
    Bottom,
    Left,
    #[default]
    Right,
}

// ============================================================================
// Sheet
// ============================================================================

/// Sheet container with backdrop
pub struct Sheet<V> {
    id: ViewId,
    open: RwSignal<bool>,
    content: V,
}

impl<V: IntoView + 'static> Sheet<V> {
    /// Create a new sheet with the given open signal and content
    pub fn new(open: RwSignal<bool>, content: V) -> Self { Self { id: ViewId::new(), open, content }
    }
}


impl<V: IntoView + 'static> HasViewId for Sheet<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Sheet<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let open = self.open;

        // Backdrop
        let backdrop = floem::views::Empty::new()
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    s.absolute()
                        .inset_0()
                        .background(t.foreground.with_alpha(0.5))
                })
            })
            .on_click_stop(move |_| {
                open.update(|v| *v = false);
            });

        // Content wrapper
        let content_wrapper = floem::views::Container::new(self.content);

        // Use Overlay with fixed positioning
        let sheet_overlay = Overlay::new(
            floem::views::stack((backdrop, content_wrapper))
                .style(|s| s.width_full().height_full()),
        )
        .style(move |s| {
            let is_open = open.get();
            s.fixed()
                .inset_0()
                .width_full()
                .height_full()
                .z_index(50)
                .apply_if(!is_open, |s| s.hide())
        });

        Box::new(sheet_overlay)
    }
}

// ============================================================================
// SheetContent
// ============================================================================

/// The content panel of a sheet
pub struct SheetContent<V> {
    id: ViewId,
    child: V,
    side: SheetSide,
}

impl<V: IntoView + 'static> SheetContent<V> {
    /// Create new sheet content
    pub fn new(child: V) -> Self { Self { id: ViewId::new(),
            child,
            side: SheetSide::Right,
        }
    }

    /// Set which side the sheet appears from
    pub fn side(mut self, side: SheetSide) -> Self { self.side = side;
        self
    }
}


impl<V: IntoView + 'static> HasViewId for SheetContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SheetContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let side = self.side;

        Box::new(floem::views::Container::with_id(self.id, self.child).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .background(t.background)
                    .border_color(t.border)
                    .padding(24.0)
                    .position(floem::style::Position::Absolute)
                    .z_index(50)
                    .display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Column)
                    .gap(16.0);
                match side {
                    SheetSide::Top => base
                        .inset_top(0.0)
                        .inset_left(0.0)
                        .inset_right(0.0)
                        .border_bottom(1.0)
                        .min_height(200.0),
                    SheetSide::Bottom => base
                        .inset_bottom(0.0)
                        .inset_left(0.0)
                        .inset_right(0.0)
                        .border_top(1.0)
                        .min_height(200.0),
                    SheetSide::Left => base
                        .inset_top(0.0)
                        .inset_bottom(0.0)
                        .inset_left(0.0)
                        .border_right(1.0)
                        .width(320.0),
                    SheetSide::Right => base
                        .inset_top(0.0)
                        .inset_bottom(0.0)
                        .inset_right(0.0)
                        .border_left(1.0)
                        .width(320.0),
                }
            })
        }))
    }
}

// ============================================================================
// SheetHeader
// ============================================================================

/// Header section for sheet content
pub struct SheetHeader<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SheetHeader<V> {
    /// Create new sheet header
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for SheetHeader<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SheetHeader<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Column)
                .gap(4.0)
        }))
    }
}

// ============================================================================
// SheetTitle
// ============================================================================

/// Title text for sheet
pub struct SheetTitle {
    id: ViewId,
    text: String,
}

impl SheetTitle {
    /// Create new sheet title
    pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() }
    }
}


impl HasViewId for SheetTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SheetTitle {
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
// SheetDescription
// ============================================================================

/// Description text for sheet
pub struct SheetDescription {
    id: ViewId,
    text: String,
}

impl SheetDescription {
    /// Create new sheet description
    pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() }
    }
}


impl HasViewId for SheetDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SheetDescription {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| s.font_size(14.0).color(t.muted_foreground))
        }))
    }
}

// ============================================================================
// SheetFooter
// ============================================================================

/// Footer section for sheet (typically for actions)
pub struct SheetFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SheetFooter<V> {
    /// Create new sheet footer
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for SheetFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SheetFooter<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Row)
                .gap(8.0)
                .justify_end()
                .margin_top(16.0)
        }))
    }
}

// ============================================================================
// SheetClose
// ============================================================================

/// Close button for sheet
pub struct SheetClose<V> {
    id: ViewId,
    open: RwSignal<bool>,
    child: V,
}

impl<V: IntoView + 'static> SheetClose<V> {
    /// Create new sheet close button
    pub fn new(open: RwSignal<bool>, child: V) -> Self { Self { id: ViewId::new(), open, child }
    }
}


impl<V: IntoView + 'static> HasViewId for SheetClose<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SheetClose<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let open = self.open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_click_stop(move |_| {
                    open.update(|v| *v = false);
                }),
        )
    }
}
