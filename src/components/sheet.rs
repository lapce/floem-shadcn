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
//!     Stack::vertical((
//!         label(|| "Sheet Title"),
//!         label(|| "Sheet content goes here..."),
//!     ))
//! ).side(SheetSide::Right));
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::{Decorators, Overlay};
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;
use floem_tailwind::TailwindExt;

/// Which side the sheet slides in from.
#[derive(Clone, Copy, Default)]
pub enum SheetSide {
    Top,
    Bottom,
    Left,
    #[default]
    Right,
}

/// Sheet container with backdrop.
pub struct Sheet<V> {
    id: ViewId,
    open: RwSignal<bool>,
    content: V,
}

impl<V: IntoView + 'static> Sheet<V> {
    /// Create a new sheet with the given open signal and content.
    pub fn new(open: RwSignal<bool>, content: V) -> Self {
        Self {
            id: ViewId::new(),
            open,
            content,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for Sheet<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Sheet<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let open = self.open;

        let backdrop = floem::views::Empty::new()
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    s.absolute()
                        .inset(0.0)
                        .background(t.foreground.with_alpha(0.5))
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                open.update(|v| *v = false);
            });

        let content_wrapper = floem::views::Container::new(self.content);

        let sheet_overlay = Overlay::new(
            floem::views::Stack::new((backdrop, content_wrapper)).style(|s| s.w_full().h_full()),
        )
        .style(move |s| {
            let is_open = open.get();
            s.fixed()
                .inset(0.0)
                .w_full()
                .h_full()
                .z_index(50)
                .apply_if(!is_open, |s| s.hide())
        });

        Box::new(sheet_overlay)
    }
}

/// The content panel of a sheet.
pub struct SheetContent<V> {
    id: ViewId,
    child: V,
    side: SheetSide,
}

impl<V: IntoView + 'static> SheetContent<V> {
    /// Create new sheet content.
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            side: SheetSide::Right,
        }
    }

    /// Set which side the sheet appears from.
    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let side = self.side;

        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let base = s
                        .background(t.background)
                        .border_color(t.border)
                        .p_6()
                        .position(floem::style::Position::Absolute)
                        .z_index(50)
                        .display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Column)
                        .gap_4();
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
            }),
        )
    }
}

/// Header section for sheet content.
pub struct SheetHeader<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SheetHeader<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SheetHeader<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SheetHeader<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Column)
                    .gap_1()
            }),
        )
    }
}

/// Title text for sheet.
pub struct SheetTitle {
    id: ViewId,
    text: String,
}

impl SheetTitle {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for SheetTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SheetTitle {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::with_id(self.id, text)
                .style(|s| s.with_shadcn_theme(move |s, t| s.text_lg().color(t.foreground))),
        )
    }
}

/// Description text for sheet.
pub struct SheetDescription {
    id: ViewId,
    text: String,
}

impl SheetDescription {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for SheetDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SheetDescription {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::with_id(self.id, text)
                .style(|s| s.with_shadcn_theme(move |s, t| s.text_sm().color(t.muted_foreground))),
        )
    }
}

/// Footer section for sheet (actions).
pub struct SheetFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SheetFooter<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SheetFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SheetFooter<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Row)
                    .gap_2()
                    .justify_end()
                    .margin_top(16.0)
            }),
        )
    }
}

/// Close button for sheet.
pub struct SheetClose<V> {
    id: ViewId,
    open: RwSignal<bool>,
    child: V,
}

impl<V: IntoView + 'static> SheetClose<V> {
    pub fn new(open: RwSignal<bool>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            open,
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SheetClose<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SheetClose<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let open = self.open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    open.update(|v| *v = false);
                }),
        )
    }
}
