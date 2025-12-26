//! Hover Card component with builder-style API
//!
//! Based on shadcn/ui Hover Card - a card that appears when hovering over a trigger.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::hover_card::HoverCard;
//!
//! HoverCard::new()
//!     .trigger(|| label(|| "@username"))
//!     .content(|| {
//!         Stack::vertical((
//!             Avatar::new().fallback("UN"),
//!             label(|| "Username"),
//!             label(|| "Software Engineer"),
//!         ))
//!     });
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

/// Side where the hover card appears
#[derive(Clone, Copy, Default)]
pub enum HoverCardSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// Alignment of the hover card
#[derive(Clone, Copy, Default)]
pub enum HoverCardAlign {
    Start,
    #[default]
    Center,
    End,
}

// ============================================================================
// HoverCard
// ============================================================================

/// A card that appears on hover
pub struct HoverCard<T, C> {
    trigger: Option<T>,
    content: Option<C>,
    side: HoverCardSide,
    align: HoverCardAlign,
}

impl HoverCard<(), ()> {
    /// Create a new hover card
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
    /// Set the trigger element
    pub fn trigger<T2: Fn() -> V, V: IntoView + 'static>(self, trigger: T2) -> HoverCard<T2, C> {
        HoverCard {
            trigger: Some(trigger),
            content: self.content,
            side: self.side,
            align: self.align,
        }
    }

    /// Set the hover card content
    pub fn content<C2: Fn() -> V, V: IntoView + 'static>(self, content: C2) -> HoverCard<T, C2> {
        HoverCard {
            trigger: self.trigger,
            content: Some(content),
            side: self.side,
            align: self.align,
        }
    }

    /// Set which side the card appears on
    pub fn side(mut self, side: HoverCardSide) -> Self {
        self.side = side;
        self
    }

    /// Set the alignment
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
    /// Build the hover card view
    pub fn build(self) -> impl IntoView {
        let trigger = self.trigger;
        let content = self.content;
        let side = self.side;
        let align = self.align;

        // Hover state
        let is_hovered = RwSignal::new(false);

        // Trigger wrapper
        let trigger_view = if let Some(trigger_fn) = trigger {
            floem::views::Container::new(trigger_fn())
                .on_event_stop(floem::event::EventListener::PointerEnter, move |_| {
                    is_hovered.set(true);
                })
                .on_event_stop(floem::event::EventListener::PointerLeave, move |_| {
                    is_hovered.set(false);
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        // Content card
        let content_view = if let Some(content_fn) = content {
            floem::views::Container::new(content_fn())
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let hovered = is_hovered.get();
                        let base = s
                            .padding(16.0)
                            .min_width(200.0)
                            .background(t.popover)
                            .border(1.0)
                            .border_color(t.border)
                            .border_radius(t.radius)
                            .box_shadow_blur(8.0)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                            .position(floem::style::Position::Absolute)
                            .z_index(50);
                        // Position based on side
                        let positioned = match side {
                            HoverCardSide::Top => base.inset_bottom_pct(100.0).margin_bottom(8.0),
                            HoverCardSide::Bottom => base.inset_top_pct(100.0).margin_top(8.0),
                            HoverCardSide::Left => base.inset_right_pct(100.0).margin_right(8.0),
                            HoverCardSide::Right => base.inset_left_pct(100.0).margin_left(8.0),
                        };
                        // Alignment
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
                .on_event_stop(floem::event::EventListener::PointerEnter, move |_| {
                    is_hovered.set(true);
                })
                .on_event_stop(floem::event::EventListener::PointerLeave, move |_| {
                    is_hovered.set(false);
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        floem::views::Container::new(floem::views::Stack::new((trigger_view, content_view)))
            .style(|s| s.position(floem::style::Position::Relative))
    }
}


impl<T, C, TV, CV> HasViewId for HoverCard<T, C> where
    T: Fn() -> TV + 'static,
    C: Fn() -> CV + 'static,
    TV: IntoView + 'static,
    CV: IntoView + 'static,
 {
    fn view_id(&self) -> ViewId {
        ViewId::new()
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// ============================================================================
// HoverCardContent
// ============================================================================

/// Styled content container for hover card
pub struct HoverCardContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> HoverCardContent<V> {
    /// Create new hover card content
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for HoverCardContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for HoverCardContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(move |s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Column)
                .gap(8.0)
        }))
    }
}

// ============================================================================
// HoverCardTrigger
// ============================================================================

/// Styled trigger for hover card
pub struct HoverCardTrigger<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> HoverCardTrigger<V> {
    /// Create new hover card trigger
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for HoverCardTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for HoverCardTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.with_shadcn_theme(move |s, t| s.color(t.primary))),
        )
    }
}
