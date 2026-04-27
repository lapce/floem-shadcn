//! Drawer component with builder-style API
//!
//! Based on shadcn/ui Drawer - a slide-out panel from the edge of the screen.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::drawer::{Drawer, DrawerSide, DrawerContent, DrawerTitle, DrawerDescription, DrawerFooter};
//! use floem_shadcn::components::button::Button;
//!
//! let open = RwSignal::new(false);
//!
//! Drawer::new(open)
//!     .side(DrawerSide::Bottom)
//!     .content(DrawerContent::new((
//!         DrawerTitle::new("Edit profile"),
//!         DrawerDescription::new("Make changes to your profile here."),
//!         DrawerFooter::new((
//!             Button::new("Cancel").outline(),
//!             Button::new("Save").primary(),
//!         )),
//!     )));
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::{Decorators, Overlay};
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// Which side the drawer slides in from.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum DrawerSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

/// Drawer component – a slide‑out panel with backdrop.
pub struct Drawer<V> {
    id: ViewId,
    is_open: RwSignal<bool>,
    side: DrawerSide,
    content: Option<V>,
}

impl Drawer<()> {
    /// Create a new drawer controlled by the given open signal.
    pub fn new(is_open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            is_open,
            side: DrawerSide::Bottom,
            content: None,
        }
    }
}

impl<V> Drawer<V> {
    /// Set the side from which the drawer appears.
    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }
    /// Provide the drawer content.
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;
        let side = self.side;
        let handle = floem::views::Empty::new().style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                if side == DrawerSide::Bottom || side == DrawerSide::Top {
                    s.w_10()
                        .h_1()
                        .background(t.muted_foreground)
                        .rounded_full()
                        .mt_2()
                        .mb_2()
                } else {
                    s.display(floem::style::Display::None)
                }
            })
        });
        let content_view = if let Some(content) = self.content {
            floem::views::Container::new(content).into_any()
        } else {
            floem::views::Empty::new().into_any()
        };
        let drawer_panel = floem::views::Stack::vertical((handle, content_view)).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .absolute()
                    .background(t.background)
                    .border_1()
                    .border_color(t.border)
                    .z_index(10)
                    .items_center();
                match side {
                    DrawerSide::Bottom => base
                        .inset_bottom(0.0)
                        .inset_left(0.0)
                        .inset_right(0.0)
                        .min_height(200.0)
                        .max_height_pct(90.0)
                        .rounded_md()
                        .border_bottom(0.0),
                    DrawerSide::Top => base
                        .inset_top(0.0)
                        .inset_left(0.0)
                        .inset_right(0.0)
                        .min_height(200.0)
                        .max_height_pct(90.0)
                        .rounded_md()
                        .border_top(0.0),
                    DrawerSide::Left => base
                        .inset_top(0.0)
                        .inset_bottom(0.0)
                        .inset_left(0.0)
                        .min_width(300.0)
                        .max_width_pct(90.0)
                        .rounded_md()
                        .border_left(0.0),
                    DrawerSide::Right => base
                        .inset_top(0.0)
                        .inset_bottom(0.0)
                        .inset_right(0.0)
                        .min_width(300.0)
                        .max_width_pct(90.0)
                        .rounded_md()
                        .border_right(0.0),
                }
            })
        });
        let backdrop = floem::views::Empty::new()
            .style(move |s| {
                s.absolute()
                    .inset(0.0)
                    .background(peniko::Color::from_rgba8(0, 0, 0, 128))
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                is_open.set(false);
            });
        let drawer_overlay = Overlay::new(
            floem::views::Stack::new((backdrop, drawer_panel)).style(|s| s.w_full().h_full()),
        )
        .style(move |s| {
            let open = is_open.get();
            s.fixed()
                .inset_0()
                .w_full()
                .h_full()
                .apply_if(!open, |s| s.hide())
        });
        Box::new(drawer_overlay)
    }
}

/// When clicked, opens the nearest parent Drawer.
pub struct DrawerTrigger<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}
impl<V: IntoView + 'static> DrawerTrigger<V> {
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child,
            is_open,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for DrawerTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for DrawerTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let is_open = self.is_open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    is_open.set(true);
                }),
        )
    }
}

/// Content container for a drawer.
pub struct DrawerContent<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> DrawerContent<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for DrawerContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for DrawerContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.w_full().p_4().flex_col()),
        )
    }
}

/// Header section for a drawer.
pub struct DrawerHeader<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> DrawerHeader<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for DrawerHeader<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for DrawerHeader<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.w_full().pb_4().flex_col().items_center()),
        )
    }
}

/// Title text for a drawer.
pub struct DrawerTitle {
    id: ViewId,
    text: String,
}
impl DrawerTitle {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for DrawerTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for DrawerTitle {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_lg().font_semibold().color(t.foreground))
        }))
    }
}

/// Description text for a drawer.
pub struct DrawerDescription {
    id: ViewId,
    text: String,
}
impl DrawerDescription {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for DrawerDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for DrawerDescription {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_sm().color(t.muted_foreground).mt_1())
        }))
    }
}

/// Footer section for a drawer (action buttons).
pub struct DrawerFooter<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> DrawerFooter<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for DrawerFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for DrawerFooter<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.w_full().pt_4().flex_col().gap_2()),
        )
    }
}

/// When clicked, closes the nearest parent Drawer.
pub struct DrawerClose<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}
impl<V: IntoView + 'static> DrawerClose<V> {
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child,
            is_open,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for DrawerClose<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for DrawerClose<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let is_open = self.is_open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    is_open.set(false);
                }),
        )
    }
}
