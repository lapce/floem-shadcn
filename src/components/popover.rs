//! Popover component with builder-style API
//!
//! Based on shadcn/ui Popover - displays rich content in a floating panel.
//!
//! # Example
//!
//! ```
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::popover::Popover;
//!
//! let open = RwSignal::new(false);
//!
//! Popover::new(open)
//!     .trigger(|| label(|| "Click me"))
//!     .content(|| label(|| "Popover content"));
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

#[derive(Clone, Copy, Default)]
pub enum PopoverAlign {
    Start,
    #[default]
    Center,
    End,
}
#[derive(Clone, Copy, Default)]
pub enum PopoverSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

pub struct Popover<T, C> {
    open: RwSignal<bool>,
    trigger: Option<T>,
    content: Option<C>,
    side: PopoverSide,
    align: PopoverAlign,
}

impl Popover<(), ()> {
    pub fn new(open: RwSignal<bool>) -> Self {
        Self {
            open,
            trigger: None,
            content: None,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Center,
        }
    }
}

impl<T, C> Popover<T, C> {
    pub fn trigger<T2: Fn() -> V, V: IntoView + 'static>(self, trigger: T2) -> Popover<T2, C> {
        Popover {
            open: self.open,
            trigger: Some(trigger),
            content: self.content,
            side: self.side,
            align: self.align,
        }
    }
    pub fn content<C2: Fn() -> V, V: IntoView + 'static>(self, content: C2) -> Popover<T, C2> {
        Popover {
            open: self.open,
            trigger: self.trigger,
            content: Some(content),
            side: self.side,
            align: self.align,
        }
    }
    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }
    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }
}

impl<T, C, TV, CV> Popover<T, C>
where
    T: Fn() -> TV + 'static,
    C: Fn() -> CV + 'static,
    TV: IntoView + 'static,
    CV: IntoView + 'static,
{
    pub fn build(self) -> impl IntoView {
        let open = self.open;
        let trigger = self.trigger;
        let content = self.content;
        let side = self.side;
        let align = self.align;

        let trigger_view = if let Some(trigger_fn) = trigger {
            floem::views::Container::new(trigger_fn())
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    open.update(|v| *v = !*v);
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        let content_view = if let Some(content_fn) = content {
            floem::views::Container::new(content_fn())
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let is_open = open.get();
                        let base = s
                            .p_4()
                            .background(t.popover)
                            .border_1()
                            .border_color(t.border)
                            .rounded_md()
                            .box_shadow_blur(8.0)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                            .absolute()
                            .z_index(50);
                        let positioned = match side {
                            PopoverSide::Top => base.inset_bottom_pct(100.0).mb_2(),
                            PopoverSide::Bottom => base.inset_top_pct(100.0).mt_2(),
                            PopoverSide::Left => base.inset_right_pct(100.0).mr_2(),
                            PopoverSide::Right => base.inset_left_pct(100.0).ml_2(),
                        };
                        let aligned = match (side, align) {
                            (PopoverSide::Top | PopoverSide::Bottom, PopoverAlign::Start) => {
                                positioned.inset_left(0.0)
                            }
                            (PopoverSide::Top | PopoverSide::Bottom, PopoverAlign::Center) => {
                                positioned.inset_left_pct(50.0).margin_left(-50.0)
                            }
                            (PopoverSide::Top | PopoverSide::Bottom, PopoverAlign::End) => {
                                positioned.inset_right(0.0)
                            }
                            (PopoverSide::Left | PopoverSide::Right, PopoverAlign::Start) => {
                                positioned.inset_top(0.0)
                            }
                            (PopoverSide::Left | PopoverSide::Right, PopoverAlign::Center) => {
                                positioned.inset_top_pct(50.0).margin_top(-50.0)
                            }
                            (PopoverSide::Left | PopoverSide::Right, PopoverAlign::End) => {
                                positioned.inset_bottom(0.0)
                            }
                        };
                        if is_open {
                            aligned
                        } else {
                            aligned.display(floem::style::Display::None)
                        }
                    })
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        floem::views::Container::new(floem::views::Stack::new((trigger_view, content_view)))
            .style(|s| s.relative())
    }
}

impl<T, C, TV, CV> HasViewId for Popover<T, C>
where
    T: Fn() -> TV + 'static,
    C: Fn() -> CV + 'static,
    TV: IntoView + 'static,
    CV: IntoView + 'static,
{
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}

impl<T, C, TV, CV> IntoView for Popover<T, C>
where
    T: Fn() -> TV + 'static,
    C: Fn() -> CV + 'static,
    TV: IntoView + 'static,
    CV: IntoView + 'static,
{
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// PopoverTrigger and PopoverContent follow similar style substitutions
pub struct PopoverTrigger<V> {
    id: ViewId,
    open: RwSignal<bool>,
    child: V,
}
impl<V: IntoView + 'static> PopoverTrigger<V> {
    pub fn new(open: RwSignal<bool>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            open,
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for PopoverTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for PopoverTrigger<V> {
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
                    open.update(|v| *v = !*v);
                }),
        )
    }
}

pub struct PopoverContent<V> {
    id: ViewId,
    open: RwSignal<bool>,
    child: V,
}
impl<V: IntoView + 'static> PopoverContent<V> {
    pub fn new(open: RwSignal<bool>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            open,
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for PopoverContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for PopoverContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let open = self.open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let is_open = open.get();
                    let base = s
                        .p_4()
                        .background(t.popover)
                        .border_1()
                        .border_color(t.border)
                        .rounded_md()
                        .box_shadow_blur(8.0)
                        .box_shadow_color(t.foreground.with_alpha(0.1))
                        .z_index(50);
                    if is_open {
                        base
                    } else {
                        base.display(floem::style::Display::None)
                    }
                })
            }),
        )
    }
}
