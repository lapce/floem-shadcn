//! Radio Group component with builder-style API
//!
//! Based on shadcn/ui Radio Group - a set of radio buttons.
//!
//! # Example
//!
//! ```rust
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

// ============================================================================
// RadioGroup
// ============================================================================

/// Radio group container that manages selected state
pub struct RadioGroup<V> {
    id: ViewId,
    selected: RwSignal<String>,
    child: V,
}

impl<V: IntoView + 'static> RadioGroup<V> {
    /// Create a new radio group with the given selected signal and items
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        // shadcn/ui: grid gap-3
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.width_full()
                    .flex_direction(floem::style::FlexDirection::Column)
                    .gap_3() // gap-3 = 12px
            }),
        )
    }
}

// ============================================================================
// RadioGroupItem
// ============================================================================

/// Individual radio button item
pub struct RadioGroupItem {
    id: ViewId,
    value: String,
    label: String,
    selected_signal: Option<RwSignal<String>>,
    disabled: bool,
}

impl RadioGroupItem {
    /// Create a new radio item with value and label
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            value: value.into(),
            label: label.into(),
            selected_signal: None,
            disabled: false,
        }
    }

    /// Set the selected signal for this item
    pub fn selected(mut self, signal: RwSignal<String>) -> Self {
        self.selected_signal = Some(signal);
        self
    }

    /// Set the item as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Build the radio item view
    pub fn build(self) -> impl IntoView {
        let value = self.value.clone();
        let label = self.label.clone();
        let selected_signal = self.selected_signal;
        let disabled = self.disabled;
        let item_value = value.clone();
        let item_value_click = value.clone();

        // shadcn/ui RadioGroupItem (v4 new-york):
        // Item: size-4 shrink-0 rounded-full border border-input text-primary shadow-xs
        //       disabled:cursor-not-allowed disabled:opacity-50
        // Indicator: CircleIcon fill-primary size-2 (8px filled dot with primary color)

        // Radio circle
        let radio_circle = floem::views::Container::new(
            // Inner dot (visible when selected) - fill-primary size-2
            floem::views::Empty::new().style(move |s| {
                let val = item_value.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_selected = selected_signal
                        .map(|sig| sig.get() == val.clone())
                        .unwrap_or(false);
                    s.size_2() // size-2 = 8px
                        .rounded_full()
                        .background(t.primary) // fill-primary
                        .apply_if(!is_selected, |s| s.display(floem::style::Display::None))
                })
            }),
        )
        .style(move |s| {
            let val = value.clone();
            s.with_shadcn_theme(move |s, t| {
                let _is_selected = selected_signal
                    .map(|sig| sig.get() == val.clone())
                    .unwrap_or(false);
                // size-4 = 16px, rounded-full, border border-input, shadow-xs
                s.size_4() // size-4 = 16px
                    .flex_shrink(0.0) // shrink-0
                    .rounded_full() // rounded-full
                    .border_1() // border (1px)
                    .border_color(t.input) // border-input (same for both states)
                    .shadow_sm() // shadow-xs
                    .flex()
                    .items_center()
                    .justify_center()
                    .background(peniko::Color::TRANSPARENT) // transparent background
                    .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                    .apply_if(!disabled, |s| s.cursor(CursorStyle::Pointer))
            })
        });

        // Label
        let label_view = floem::views::Label::new(label).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                s.text_sm() // 14px
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

        // Container
        let container = floem::views::Stack::horizontal((radio_circle, label_view))
            .style(|s| s.gap_2().items_center());

        if !disabled {
            container
                .on_click_stop(move |_| {
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
