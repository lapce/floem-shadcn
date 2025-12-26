//! Popover component with builder-style API
//!
//! Based on shadcn/ui Popover - displays rich content in a floating panel.
//!
//! # Example
//!
//! ```rust
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

use crate::theme::ShadcnThemeExt;

/// Popover alignment relative to trigger
#[derive(Clone, Copy, Default)]
pub enum PopoverAlign {
    #[default]
    Start,
    Center,
    End,
}

/// Popover side (where content appears)
#[derive(Clone, Copy, Default)]
pub enum PopoverSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// A styled popover builder
pub struct Popover<T, C> {
    open: RwSignal<bool>,
    trigger: Option<T>,
    content: Option<C>,
    side: PopoverSide,
    align: PopoverAlign,
}

impl Popover<(), ()> {
    /// Create a new popover with the given open signal
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
    /// Set the trigger element
    pub fn trigger<T2: Fn() -> V, V: IntoView + 'static>(self, trigger: T2) -> Popover<T2, C> {
        Popover {
            open: self.open,
            trigger: Some(trigger),
            content: self.content,
            side: self.side,
            align: self.align,
        }
    }

    /// Set the popover content
    pub fn content<C2: Fn() -> V, V: IntoView + 'static>(self, content: C2) -> Popover<T, C2> {
        Popover {
            open: self.open,
            trigger: self.trigger,
            content: Some(content),
            side: self.side,
            align: self.align,
        }
    }

    /// Set which side the popover appears on
    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    /// Set the alignment of the popover
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
    /// Build the popover view
    pub fn build(self) -> impl IntoView {
        let open = self.open;
        let trigger = self.trigger;
        let content = self.content;
        let side = self.side;
        let align = self.align;

        // Trigger wrapper
        let trigger_view = if let Some(trigger_fn) = trigger {
            floem::views::Container::new(trigger_fn())
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_click_stop(move |_| {
                    open.update(|v| *v = !*v);
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        // Content panel
        let content_view = if let Some(content_fn) = content {
            floem::views::Container::new(content_fn())
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let is_open = open.get();
                        let base = s
                            .padding(16.0)
                            .background(t.popover)
                            .border(1.0)
                            .border_color(t.border)
                            .border_radius(t.radius)
                            .box_shadow_blur(8.0)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                            .position(floem::style::Position::Absolute)
                            .z_index(50);
                        // Position based on side (use percentage + margin for offset)
                        let positioned = match side {
                            PopoverSide::Top => base.inset_bottom_pct(100.0).margin_bottom(8.0),
                            PopoverSide::Bottom => base.inset_top_pct(100.0).margin_top(8.0),
                            PopoverSide::Left => base.inset_right_pct(100.0).margin_right(8.0),
                            PopoverSide::Right => base.inset_left_pct(100.0).margin_left(8.0),
                        };
                        // Alignment
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
            .style(|s| s.position(floem::style::Position::Relative))
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// ============================================================================
// PopoverTrigger (alternative API)
// ============================================================================

/// Popover trigger button with default styling
pub struct PopoverTrigger<V> {
    id: ViewId,
    open: RwSignal<bool>,
    child: V,
}

impl<V: IntoView + 'static> PopoverTrigger<V> {
    /// Create a new popover trigger
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
                    open.update(|v| *v = !*v);
                }),
        )
    }
}

// ============================================================================
// PopoverContent
// ============================================================================

/// Styled popover content panel
pub struct PopoverContent<V> {
    id: ViewId,
    open: RwSignal<bool>,
    child: V,
}

impl<V: IntoView + 'static> PopoverContent<V> {
    /// Create new popover content
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let open = self.open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let is_open = open.get();
                    let base = s
                        .padding(16.0)
                        .background(t.popover)
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(t.radius)
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
