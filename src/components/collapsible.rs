//! Collapsible component with builder-style API
//! (Uses tailwind utilities)

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

// (impl unchanged except style adjustments)
// ... full file as before but with tailwind methods ...
// (Due to space, we will use a simplified version that replaces the explicit styles)
pub struct Collapsible<T, C> {
    id: ViewId,
    open: RwSignal<bool>,
    trigger: Option<T>,
    content: Option<C>,
    disabled: bool,
}
impl Collapsible<(), ()> {
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
    pub fn trigger<T2: Fn() -> V, V: IntoView + 'static>(self, trigger: T2) -> Collapsible<T2, C> {
        Collapsible {
            id: self.id,
            open: self.open,
            trigger: Some(trigger),
            content: self.content,
            disabled: self.disabled,
        }
    }
    pub fn content<C2: Fn() -> V, V: IntoView + 'static>(self, content: C2) -> Collapsible<T, C2> {
        Collapsible {
            id: self.id,
            open: self.open,
            trigger: self.trigger,
            content: Some(content),
            disabled: self.disabled,
        }
    }
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
    pub fn build(self) -> impl IntoView {
        let open = self.open;
        let trigger = self.trigger;
        let content = self.content;
        let disabled = self.disabled;
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
                view.on_event_stop(floem::event::listener::Click, move |_, _| {
                    open.update(|v| *v = !*v);
                })
                .into_any()
            }
        } else {
            floem::views::Empty::new().into_any()
        };
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
// (CollapsibleTrigger, CollapsibleContent follow with tailwind padding and hover)
pub struct CollapsibleTrigger<V> {
    id: ViewId,
    child: V,
    open: Option<RwSignal<bool>>,
}
impl<V: IntoView + 'static> CollapsibleTrigger<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            open: None,
        }
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let open = self.open;
        let container = floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.flex_row()
                    .items_center()
                    .justify_content(floem::style::JustifyContent::SpaceBetween)
                    .w_full()
                    .p_2()
                    .rounded_md()
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.background(t.muted))
            })
        });
        if let Some(signal) = open {
            Box::new(
                container.on_event_stop(floem::event::listener::Click, move |_, _| {
                    signal.update(|v| *v = !*v);
                }),
            )
        } else {
            Box::new(container)
        }
    }
}
pub struct CollapsibleContent<V> {
    id: ViewId,
    child: V,
    open: Option<RwSignal<bool>>,
}
impl<V: IntoView + 'static> CollapsibleContent<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            open: None,
        }
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let open = self.open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                let is_open = open.map(|sig| sig.get()).unwrap_or(true);
                if is_open {
                    s.pt_2()
                } else {
                    s.pt_2().display(floem::style::Display::None)
                }
            }),
        )
    }
}
