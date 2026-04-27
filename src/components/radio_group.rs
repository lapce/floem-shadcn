//! Radio Group component with builder-style API
//!
//! Based on shadcn/ui Radio Group - a set of radio buttons.
//!
//! # Example
//!
//! ```
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::radio_group::{RadioGroup, RadioGroupItem};
//!
//! let selected = RwSignal::new("option1".to_string());
//!
//! RadioGroup::new(selected, (
//!     RadioGroupItem::new("option1", "Option 1"),
//!     RadioGroupItem::new("option2", "Option 2"),
//!     RadioGroupItem::new("option3", "Option 3"),
//! ));
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

pub struct RadioGroup<V> {
    id: ViewId,
    #[allow(dead_code)]
    selected: RwSignal<String>,
    child: V,
}

impl<V: IntoView + 'static> RadioGroup<V> {
    pub fn new(selected: RwSignal<String>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            selected,
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for RadioGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for RadioGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.w_full().flex_col().gap_3()),
        )
    }
}

pub struct RadioGroupItem {
    id: ViewId,
    value: String,
    label: String,
    selected_signal: Option<RwSignal<String>>,
    disabled: bool,
}

impl RadioGroupItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            value: value.into(),
            label: label.into(),
            selected_signal: None,
            disabled: false,
        }
    }
    pub fn selected(mut self, signal: RwSignal<String>) -> Self {
        self.selected_signal = Some(signal);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn build(self) -> impl IntoView {
        let value = self.value.clone();
        let label = self.label.clone();
        let selected_signal = self.selected_signal;
        let disabled = self.disabled;
        let item_value = value.clone();
        let item_value_click = value.clone();

        let radio_circle =
            floem::views::Container::new(floem::views::Empty::new().style(move |s| {
                let val = item_value.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_selected = selected_signal
                        .map(|sig| sig.get() == val.clone())
                        .unwrap_or(false);
                    s.size_2()
                        .rounded_full()
                        .background(t.primary)
                        .apply_if(!is_selected, |s| s.display(floem::style::Display::None))
                })
            }))
            .style(move |s| {
                let _val = value.clone();
                s.with_shadcn_theme(move |s, t| {
                    s.size_4()
                        .rounded_full()
                        .border_1()
                        .border_color(t.input)
                        .box_shadow_blur(2.0)
                        .box_shadow_color(peniko::Color::from_rgba8(0, 0, 0, 25))
                        .flex()
                        .items_center()
                        .justify_center()
                        .background(peniko::Color::TRANSPARENT)
                        .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                        .apply_if(!disabled, |s| s.cursor(CursorStyle::Pointer))
                })
            });

        let label_view = floem::views::Label::new(label).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                s.text_sm()
                    .font_medium()
                    .leading_none()
                    .color(if disabled {
                        t.muted_foreground
                    } else {
                        t.foreground
                    })
                    .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                    .apply_if(!disabled, |s| s.cursor(CursorStyle::Pointer))
            })
        });

        let container = floem::views::Stack::horizontal((radio_circle, label_view))
            .style(|s| s.gap_2().items_center());

        if !disabled {
            container
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    if let Some(signal) = selected_signal {
                        signal.update(|v| *v = item_value_click.clone());
                    }
                })
                .into_any()
        } else {
            container.into_any()
        }
    }
}

impl HasViewId for RadioGroupItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for RadioGroupItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
