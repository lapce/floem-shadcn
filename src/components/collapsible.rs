//! Collapsible component with builder-style API
//!
//! Based on shadcn/ui Collapsible - an interactive component that expands/collapses content.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::collapsible::*;
//!
//! let open = RwSignal::new(false);
//!
//! Collapsible::new(open)
//!     .trigger(|| label(|| "Toggle"))
//!     .content(|| {
//!         Stack::vertical((
//!             label(|| "Item 1"),
//!             label(|| "Item 2"),
//!             label(|| "Item 3"),
//!         ))
//!     });
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Collapsible
// ============================================================================

/// A component that can be expanded or collapsed
pub struct Collapsible<T, C> {
    id: ViewId,
    open: RwSignal<bool>,
    trigger: Option<T>,
    content: Option<C>,
    disabled: bool,
}

impl Collapsible<(), ()> {
    /// Create a new collapsible with open state
    pub fn new(open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            open,
            trigger: None,
            content: None,
            disabled: false,
        }
    }
}

impl<T, C> Collapsible<T, C> {
    /// Set the trigger element (clickable header)
    pub fn trigger<T2: Fn() -> V, V: IntoView + 'static>(self, trigger: T2) -> Collapsible<T2, C> {
        Collapsible {
            id: self.id,
            open: self.open,
            trigger: Some(trigger),
            content: self.content,
            disabled: self.disabled,
        }
    }

    /// Set the collapsible content
    pub fn content<C2: Fn() -> V, V: IntoView + 'static>(self, content: C2) -> Collapsible<T, C2> {
        Collapsible {
            id: self.id,
            open: self.open,
            trigger: self.trigger,
            content: Some(content),
            disabled: self.disabled,
        }
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<T, C, TV, CV> Collapsible<T, C>
where
    T: Fn() -> TV + 'static,
    C: Fn() -> CV + 'static,
    TV: IntoView + 'static,
    CV: IntoView + 'static,
{
    /// Build the collapsible view
    pub fn build(self) -> impl IntoView {
        let open = self.open;
        let trigger = self.trigger;
        let content = self.content;
        let disabled = self.disabled;

        // Trigger wrapper
        let trigger_view = if let Some(trigger_fn) = trigger {
            let view = floem::views::Container::new(trigger_fn()).style(move |s| {
                s.cursor(if disabled {
                    CursorStyle::Default
                } else {
                    CursorStyle::Pointer
                })
            });

            if disabled {
                view.into_any()
            } else {
                view.on_click_stop(move |_| {
                    open.update(|v| *v = !*v);
                })
                .into_any()
            }
        } else {
            floem::views::Empty::new().into_any()
        };

        // Content wrapper (hidden when collapsed)
        let content_view = if let Some(content_fn) = content {
            floem::views::Container::new(content_fn())
                .style(move |s| {
                    let is_open = open.get();
                    if is_open {
                        s
                    } else {
                        s.display(floem::style::Display::None)
                    }
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        floem::views::Stack::vertical((trigger_view, content_view))
    }
}

impl<T, C, TV, CV> HasViewId for Collapsible<T, C>
where
    T: Fn() -> TV + 'static,
    C: Fn() -> CV + 'static,
    TV: IntoView + 'static,
    CV: IntoView + 'static,
{
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<T, C, TV, CV> IntoView for Collapsible<T, C>
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
// CollapsibleTrigger
// ============================================================================

/// Styled trigger for collapsible
pub struct CollapsibleTrigger<V> {
    id: ViewId,
    child: V,
    open: Option<RwSignal<bool>>,
}

impl<V: IntoView + 'static> CollapsibleTrigger<V> {
    /// Create a new collapsible trigger
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            open: None,
        }
    }

    /// Connect to open signal
    pub fn open(mut self, open: RwSignal<bool>) -> Self {
        self.open = Some(open);
        self
    }
}

impl<V: IntoView + 'static> HasViewId for CollapsibleTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CollapsibleTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let open = self.open;

        let container = floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Row)
                    .items_center()
                    .justify_content(floem::style::JustifyContent::SpaceBetween)
                    .width_full()
                    .padding(8.0)
                    .border_radius(t.radius)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.background(t.muted))
            })
        });

        if let Some(signal) = open {
            Box::new(container.on_click_stop(move |_| {
                signal.update(|v| *v = !*v);
            }))
        } else {
            Box::new(container)
        }
    }
}

// ============================================================================
// CollapsibleContent
// ============================================================================

/// Styled content container for collapsible
pub struct CollapsibleContent<V> {
    id: ViewId,
    child: V,
    open: Option<RwSignal<bool>>,
}

impl<V: IntoView + 'static> CollapsibleContent<V> {
    /// Create a new collapsible content
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            open: None,
        }
    }

    /// Connect to open signal
    pub fn open(mut self, open: RwSignal<bool>) -> Self {
        self.open = Some(open);
        self
    }
}

impl<V: IntoView + 'static> HasViewId for CollapsibleContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CollapsibleContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let open = self.open;

        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                let is_open = open.map(|sig| sig.get()).unwrap_or(true);
                let base = s.padding_top(8.0);

                if is_open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            }),
        )
    }
}
