//! Select component with builder-style API
//!
//! Based on shadcn/ui Select component - a dropdown for selecting from a list.
//! This wraps floem's native Dropdown with shadcn-style theming.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::select::Select;
//!
//! let selected = RwSignal::new("option1".to_string());
//!
//! let select = Select::new(selected, vec![
//!     ("option1", "Option 1"),
//!     ("option2", "Option 2"),
//!     ("option3", "Option 3"),
//! ]);
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::views::dropdown::Dropdown;

use crate::theme::ShadcnThemeExt;

/// A styled select (dropdown) builder
pub struct Select<T: Clone + PartialEq + std::fmt::Display + 'static> {
    id: ViewId,
    selected: RwSignal<T>,
    items: Vec<T>,
    placeholder: Option<String>,
    disabled: bool,
}

impl<T: Clone + PartialEq + std::fmt::Display + 'static> Select<T> {
    /// Create a new select with the given selected signal and items
    pub fn new(selected: RwSignal<T>, items: impl IntoIterator<Item = T>) -> Self { Self { id: ViewId::new(),
            selected,
            items: items.into_iter().collect(),
            placeholder: None,
            disabled: false,
        }
    }

    /// Set placeholder text (shown when nothing is selected)
    pub fn placeholder(mut self, text: impl Into<String>) -> Self { self.placeholder = Some(text.into());
        self
    }

    /// Set the select as disabled
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled;
        self
    }

    /// Build the select view
    pub fn build(self) -> impl IntoView {
        let selected = self.selected;
        let items = self.items;

        // Use floem's Dropdown with custom styling
        Dropdown::new_rw(selected, items.into_iter())
            .style(|s| s.with_shadcn_theme(|s, t| {                s.width(200.0)
                    .padding(8.0)
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .border(1.0)
                    .border_color(t.input)
                    .border_radius(6.0)
                    .background(t.background)
                    .color(t.foreground)
                    .font_size(14.0)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.border_color(t.ring))

            }))
    }
}


impl<T: Clone + PartialEq + std::fmt::Display + 'static> HasViewId for Select<T> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<T: Clone + PartialEq + std::fmt::Display + 'static> IntoView for Select<T> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

/// A simpler select builder using string key-value pairs
pub struct SimpleSelect {
    id: ViewId,
    selected: RwSignal<String>,
    items: Vec<SelectItem>,
    disabled: bool,
}

/// A select option with value and display label
#[derive(Clone, PartialEq)]
pub struct SelectItem {
    /// The value of the option
    pub value: String,
    /// The display label
    pub label: String,
}

impl SelectItem {
    /// Create a new select item
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

impl std::fmt::Display for SelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl SimpleSelect {
    /// Create a new simple select with string key-value pairs
    pub fn new(
        selected: RwSignal<String>,
        items: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self { let items: Vec<SelectItem> = items
            .into_iter()
            .map(|(v, l)| SelectItem::new(v, l))
            .collect();

        Self {
            id: ViewId::new(),
            selected,
            items,
            disabled: false,
        }
    }

    /// Set the select as disabled
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled;
        self
    }

    /// Build the select view
    pub fn build(self) -> impl IntoView {
        let selected = self.selected;
        let items = self.items.clone();
        let items_for_dropdown = self.items;

        // Find the current item based on selected value
        let current_item = RwSignal::new(
            items.iter()
                .find(|i| i.value == selected.get())
                .cloned()
                .unwrap_or_else(|| items.first().cloned().unwrap_or(SelectItem::new("", "")))
        );

        // Sync the string signal with the item signal
        let selected_for_effect = selected;
        let items_for_effect = items.clone();
        floem::reactive::Effect::new(move |_| {
            let item = current_item.get();
            if selected_for_effect.get() != item.value {
                selected_for_effect.set(item.value.clone());
            }
        });

        Dropdown::new_rw(current_item, items_for_dropdown.into_iter())
            .style(|s| s.with_shadcn_theme(|s, t| {                s.width(200.0)
                    .padding(8.0)
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .border(1.0)
                    .border_color(t.input)
                    .border_radius(6.0)
                    .background(t.background)
                    .color(t.foreground)
                    .font_size(14.0)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.border_color(t.ring))

            }))
    }
}


impl HasViewId for SimpleSelect {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SimpleSelect {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
