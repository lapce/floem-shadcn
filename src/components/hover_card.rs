//! Hover Card component with builder-style API
//!
//! Based on shadcn/ui Hover Card - displays rich content on hover.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::hover_card::HoverCard;
//!
//! HoverCard::new()
//!     .trigger(|| label(|| "Hover me"))
//!     .content(|| label(|| "Card content"));
//! ```
use crate::theme::ShadcnThemeExt;
/// Hover Card component with builder-style API
/// (tailwind-enhanced – complete file)
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

#[derive(Clone, Copy, Default)]
pub enum HoverCardSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Default)]
pub enum HoverCardAlign {
    Start,
    #[default]
    Center,
    End,
}

// Main HoverCard
pub struct HoverCard<T, C> {
    trigger: Option<T>,
    content: Option<C>,
    side: HoverCardSide,
    align: HoverCardAlign,
}
impl HoverCard<(), ()> {
    pub fn new() -> Self {
        Self {
            trigger: None,
            content: None,
            side: HoverCardSide::Bottom,
            align: HoverCardAlign::Center,
        }
    }
}
impl Default for HoverCard<(), ()> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T, C> HoverCard<T, C> {
    pub fn trigger<T2: Fn() -> V, V: IntoView + 'static>(self, trigger: T2) -> HoverCard<T2, C> {
        HoverCard {
            trigger: Some(trigger),
            content: self.content,
            side: self.side,
            align: self.align,
        }
    }
    pub fn content<C2: Fn() -> V, V: IntoView + 'static>(self, content: C2) -> HoverCard<T, C2> {
        HoverCard {
            trigger: self.trigger,
            content: Some(content),
            side: self.side,
            align: self.align,
        }
    }
    pub fn side(mut self, side: HoverCardSide) -> Self {
        self.side = side;
        self
    }
    pub fn align(mut self, align: HoverCardAlign) -> Self {
        self.align = align;
        self
    }
}
impl<T, C, TV, CV> HoverCard<T, C>
where
    T: Fn() -> TV + 'static,
    C: Fn() -> CV + 'static,
    TV: IntoView + 'static,
    CV: IntoView + 'static,
{
    pub fn build(self) -> impl IntoView {
        let trigger = self.trigger;
        let content = self.content;
        let side = self.side;
        let align = self.align;
        let is_hovered = RwSignal::new(false);
        let trigger_view = if let Some(trigger_fn) = trigger {
            floem::views::Container::new(trigger_fn())
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    is_hovered.set(true)
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    is_hovered.set(false)
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };
        let content_view = if let Some(content_fn) = content {
            floem::views::Container::new(content_fn())
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let hovered = is_hovered.get();
                        let base = s
                            .p_4()
                            .min_width(200.0)
                            .background(t.popover)
                            .border_1()
                            .border_color(t.border)
                            .rounded_md()
                            .box_shadow_blur(8.0)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                            .absolute()
                            .z_index(50);
                        let positioned = match side {
                            HoverCardSide::Top => base.inset_bottom_pct(100.0).mb_2(),
                            HoverCardSide::Bottom => base.inset_top_pct(100.0).mt_2(),
                            HoverCardSide::Left => base.inset_right_pct(100.0).mr_2(),
                            HoverCardSide::Right => base.inset_left_pct(100.0).ml_2(),
                        };
                        let aligned = match (side, align) {
                            (HoverCardSide::Top | HoverCardSide::Bottom, HoverCardAlign::Start) => {
                                positioned.inset_left(0.0)
                            }
                            (
                                HoverCardSide::Top | HoverCardSide::Bottom,
                                HoverCardAlign::Center,
                            ) => positioned.inset_left_pct(50.0).margin_left(-100.0),
                            (HoverCardSide::Top | HoverCardSide::Bottom, HoverCardAlign::End) => {
                                positioned.inset_right(0.0)
                            }
                            (HoverCardSide::Left | HoverCardSide::Right, HoverCardAlign::Start) => {
                                positioned.inset_top(0.0)
                            }
                            (
                                HoverCardSide::Left | HoverCardSide::Right,
                                HoverCardAlign::Center,
                            ) => positioned.inset_top_pct(50.0).margin_top(-50.0),
                            (HoverCardSide::Left | HoverCardSide::Right, HoverCardAlign::End) => {
                                positioned.inset_bottom(0.0)
                            }
                        };
                        if hovered {
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
impl<T, C, TV, CV> IntoView for HoverCard<T, C>
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

// HoverCardContent
pub struct HoverCardContent<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> HoverCardContent<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for HoverCardContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for HoverCardContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(move |s| s.flex_col().gap_2()),
        )
    }
}

// HoverCardTrigger
pub struct HoverCardTrigger<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> HoverCardTrigger<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for HoverCardTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for HoverCardTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.with_shadcn_theme(move |s, t| s.color(t.primary))),
        )
    }
}
