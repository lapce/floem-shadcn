//! Combobox component with builder-style API
//!
//! Based on shadcn/ui Combobox - autocomplete/searchable select.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::combobox::*;
//!
//! let selected = RwSignal::new(None);
//! let search = RwSignal::new(String::new());
//!
//! Combobox::new(selected, search)
//!     .placeholder("Select framework...")
//!     .items(vec![
//!         ComboboxItem::new("next", "Next.js"),
//!         ComboboxItem::new("sveltekit", "SvelteKit"),
//!         ComboboxItem::new("nuxt", "Nuxt.js"),
//!     ]);
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::{Decorators, text_input};

use crate::theme::ShadcnThemeExt;

// ============================================================================
// ComboboxItem (data structure)
// ============================================================================

/// Item for combobox
#[derive(Clone)]
pub struct ComboboxItemData {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl ComboboxItemData {
    /// Create a new item
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Set as disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

// ============================================================================
// Combobox
// ============================================================================

/// Searchable select/autocomplete component
pub struct Combobox {
    id: ViewId,
    selected: RwSignal<Option<String>>,
    search: RwSignal<String>,
    placeholder: String,
    items: Vec<ComboboxItemData>,
    empty_text: String,
}

impl Combobox {
    /// Create a new combobox
    pub fn new(selected: RwSignal<Option<String>>, search: RwSignal<String>) -> Self { Self { id: ViewId::new(),
            selected,
            search,
            placeholder: "Select...".to_string(),
            items: Vec::new(),
            empty_text: "No results found.".to_string(),
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self { self.placeholder = placeholder.into();
        self
    }

    /// Set items
    pub fn items(mut self, items: Vec<ComboboxItemData>) -> Self { self.items = items;
        self
    }

    /// Set empty state text
    pub fn empty_text(mut self, text: impl Into<String>) -> Self { self.empty_text = text.into();
        self
    }
}


impl HasViewId for Combobox {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Combobox {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let selected = self.selected;
        let search = self.search;
        let placeholder = self.placeholder;
        let items = self.items;
        let empty_text = self.empty_text;
        let is_open = RwSignal::new(false);

        // Clone items for different closures
        let items_for_trigger = items.clone();
        let items_for_empty = items.clone();

        // Trigger button
        let trigger = floem::views::h_stack((
            // Selected value or placeholder
            floem::views::Label::derived(move || {
                if let Some(val) = selected.get() {
                    // Find the label for the value
                    items_for_trigger
                        .iter()
                        .find(|i| i.value == val)
                        .map(|i| i.label.clone())
                        .unwrap_or(val)
                } else {
                    placeholder.clone()
                }
            })
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let has_value = selected.get().is_some();
                    s.flex_grow(1.0).font_size(14.0).color(if has_value {
                        t.foreground
                    } else {
                        t.muted_foreground
                    })
                })
            }),
            // Dropdown indicator
            floem::views::Label::new("▼").style(|s| {
                s.with_shadcn_theme(move |s, t| s.font_size(10.0).color(t.muted_foreground))
            }),
        ))
        .style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .min_width(200.0)
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .gap(8.0)
                    .items_center()
                    .background(t.background)
                    .border(1.0)
                    .border_color(t.input)
                    .border_radius(t.radius)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.border_color(t.ring))
            })
        })
        .on_click_stop(move |_| {
            is_open.update(|v| *v = !*v);
        });

        // Search input in dropdown
        let search_input = text_input(search).placeholder("Search...").style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .padding(8.0)
                    .font_size(14.0)
                    .border(0.0)
                    .border_bottom(1.0)
                    .border_color(t.border)
                    .background(floem::peniko::Color::TRANSPARENT)
                    .color(t.foreground)
            })
        });

        // Items list (static, up to 10 items)
        let item0 = create_combobox_item(0, items.clone(), selected, search, is_open);
        let item1 = create_combobox_item(1, items.clone(), selected, search, is_open);
        let item2 = create_combobox_item(2, items.clone(), selected, search, is_open);
        let item3 = create_combobox_item(3, items.clone(), selected, search, is_open);
        let item4 = create_combobox_item(4, items.clone(), selected, search, is_open);
        let item5 = create_combobox_item(5, items.clone(), selected, search, is_open);
        let item6 = create_combobox_item(6, items.clone(), selected, search, is_open);
        let item7 = create_combobox_item(7, items.clone(), selected, search, is_open);
        let item8 = create_combobox_item(8, items.clone(), selected, search, is_open);
        let item9 = create_combobox_item(9, items.clone(), selected, search, is_open);

        // Empty state
        let empty_view = floem::views::Label::new(empty_text).style(move |s| {
            let items = items_for_empty.clone();
            s.with_shadcn_theme(move |s, t| {
                let search_val = search.get();
                let has_results = items.iter().any(|item| {
                    item.label
                        .to_lowercase()
                        .contains(&search_val.to_lowercase())
                });
                let base = s
                    .width_full()
                    .padding(16.0)
                    .font_size(14.0)
                    .color(t.muted_foreground)
                    .justify_center();
                if !has_results && !search_val.is_empty() {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
        });

        let items_container = floem::views::v_stack((
            item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, empty_view,
        ))
        .style(|s| s.max_height(200.0));

        // Dropdown content
        let dropdown = floem::views::v_stack((search_input, items_container)).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let open = is_open.get();
                let base = s
                    .position(floem::style::Position::Absolute)
                    .inset_top_pct(100.0)
                    .inset_left(0.0)
                    .inset_right(0.0)
                    .margin_top(4.0)
                    .background(t.popover)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius)
                    .box_shadow_blur(8.0)
                    .box_shadow_color(t.foreground.with_alpha(0.1))
                    .z_index(100);
                if open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
        });

        // Backdrop
        let backdrop = floem::views::Empty::new()
            .style(move |s| {
                let open = is_open.get();
                let base = s
                    .position(floem::style::Position::Absolute)
                    .inset_top(-1000.0)
                    .inset_left(-1000.0)
                    .width(3000.0)
                    .height(3000.0)
                    .z_index(99);

                if open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
            .on_click_stop(move |_| {
                is_open.set(false);
                search.set(String::new());
            });

        Box::new(
            floem::views::Container::new(floem::views::stack((trigger, backdrop, dropdown)))
                .style(|s| s.position(floem::style::Position::Relative)),
        )
    }
}

fn create_combobox_item(
    index: usize,
    items: Vec<ComboboxItemData>,
    selected: RwSignal<Option<String>>,
    search: RwSignal<String>,
    is_open: RwSignal<bool>,
) -> impl IntoView {
    let items_for_label = items.clone();
    let items_for_style = items.clone();
    let items_for_click = items;

    floem::views::Label::derived(move || {
        let search_val = search.get();
        let filtered: Vec<_> = items_for_label
            .iter()
            .filter(|item| {
                item.label
                    .to_lowercase()
                    .contains(&search_val.to_lowercase())
            })
            .collect();

        filtered
            .get(index)
            .map(|i| i.label.clone())
            .unwrap_or_default()
    })
    .style(move |s| {
        let items = items_for_style.clone();
        s.with_shadcn_theme(move |s, t| {
            let search_val = search.get();
            let filtered: Vec<_> = items
                .iter()
                .filter(|item| {
                    item.label
                        .to_lowercase()
                        .contains(&search_val.to_lowercase())
                })
                .collect();
            let item_opt = filtered.get(index);
            let is_visible = item_opt.is_some();
            let is_selected = item_opt
                .map(|i| Some(i.value.clone()) == selected.get())
                .unwrap_or(false);
            let is_disabled = item_opt.map(|i| i.disabled).unwrap_or(false);
            let base = s
                .width_full()
                .padding_left(8.0)
                .padding_right(8.0)
                .padding_top(8.0)
                .padding_bottom(8.0)
                .font_size(14.0)
                .border_radius(4.0)
                .cursor(if is_disabled {
                    CursorStyle::Default
                } else {
                    CursorStyle::Pointer
                });
            if !is_visible {
                base.display(floem::style::Display::None)
            } else if is_selected {
                base.background(t.accent).color(t.accent_foreground)
            } else if is_disabled {
                base.color(t.muted_foreground)
            } else {
                base.color(t.foreground)
                    .hover(|s| s.background(t.accent).color(t.accent_foreground))
            }
        })
    })
    .on_click_stop(move |_| {
        let search_val = search.get();
        let filtered: Vec<_> = items_for_click
            .iter()
            .filter(|item| {
                item.label
                    .to_lowercase()
                    .contains(&search_val.to_lowercase())
            })
            .collect();

        if let Some(item) = filtered.get(index) {
            if !item.disabled {
                selected.set(Some(item.value.clone()));
                is_open.set(false);
                search.set(String::new());
            }
        }
    })
}

// ============================================================================
// ComboboxTrigger
// ============================================================================

/// Standalone trigger for combobox
pub struct ComboboxTrigger<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}

impl<V: IntoView + 'static> ComboboxTrigger<V> {
    /// Create a new trigger
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self { Self { id: ViewId::new(), child, is_open }
    }
}


impl<V: IntoView + 'static> HasViewId for ComboboxTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for ComboboxTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;

        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_click_stop(move |_| {
                    is_open.update(|v| *v = !*v);
                }),
        )
    }
}

// ============================================================================
// ComboboxContent
// ============================================================================

/// Content container for combobox dropdown
pub struct ComboboxContent<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}

impl<V: IntoView + 'static> ComboboxContent<V> {
    /// Create new content
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self { Self { id: ViewId::new(), child, is_open }
    }
}


impl<V: IntoView + 'static> HasViewId for ComboboxContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for ComboboxContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;

        Box::new(floem::views::Container::with_id(self.id, self.child).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let open = is_open.get();
                let base = s
                    .position(floem::style::Position::Absolute)
                    .inset_top_pct(100.0)
                    .inset_left(0.0)
                    .inset_right(0.0)
                    .margin_top(4.0)
                    .padding(4.0)
                    .background(t.popover)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius)
                    .box_shadow_blur(8.0)
                    .box_shadow_color(t.foreground.with_alpha(0.1))
                    .z_index(100)
                    .display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Column);
                if open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
        }))
    }
}

// ============================================================================
// ComboboxInput
// ============================================================================

/// Search input for combobox
pub struct ComboboxInput {
    id: ViewId,
    search: RwSignal<String>,
    placeholder: String,
}

impl ComboboxInput {
    /// Create a new search input
    pub fn new(search: RwSignal<String>) -> Self { Self { id: ViewId::new(),
            search,
            placeholder: "Search...".to_string(),
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self { self.placeholder = placeholder.into();
        self
    }
}


impl HasViewId for ComboboxInput {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ComboboxInput {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let search = self.search;
        let placeholder = self.placeholder;

        Box::new(text_input(search).placeholder(placeholder).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .padding(8.0)
                    .font_size(14.0)
                    .border(0.0)
                    .border_bottom(1.0)
                    .border_color(t.border)
                    .background(floem::peniko::Color::TRANSPARENT)
                    .color(t.foreground)
            })
        }))
    }
}

// ============================================================================
// ComboboxEmpty
// ============================================================================

/// Empty state for combobox
pub struct ComboboxEmpty {
    id: ViewId,
    text: String,
}

impl ComboboxEmpty {
    /// Create a new empty state
    pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() }
    }
}

impl Default for ComboboxEmpty {
    fn default() -> Self {
        Self::new("No results found.")
    }
}


impl HasViewId for ComboboxEmpty {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ComboboxEmpty {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;

        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .padding(16.0)
                    .font_size(14.0)
                    .color(t.muted_foreground)
                    .justify_center()
            })
        }))
    }
}

// ============================================================================
// ComboboxGroup
// ============================================================================

/// Group of related combobox items
pub struct ComboboxGroup<V> {
    id: ViewId,
    label: String,
    child: V,
}

impl<V: IntoView + 'static> ComboboxGroup<V> {
    /// Create a new group
    pub fn new(label: impl Into<String>, child: V) -> Self { Self { id: ViewId::new(),
            label: label.into(),
            child,
        }
    }
}


impl<V: IntoView + 'static> HasViewId for ComboboxGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for ComboboxGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let label = self.label;

        // Group label
        let label_view = floem::views::Label::new(label).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.padding_left(8.0)
                    .padding_right(8.0)
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .font_size(12.0)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .color(t.muted_foreground)
            })
        });

        Box::new(floem::views::v_stack((label_view, self.child)))
    }
}

// ============================================================================
// ComboboxItem (view)
// ============================================================================

/// Individual combobox item
pub struct ComboboxItem {
    id: ViewId,
    value: String,
    label: String,
    disabled: bool,
    selected: Option<RwSignal<Option<String>>>,
    is_open: Option<RwSignal<bool>>,
}

impl ComboboxItem {
    /// Create a new item
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self { Self { id: ViewId::new(),
            value: value.into(),
            label: label.into(),
            disabled: false,
            selected: None,
            is_open: None,
        }
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled;
        self
    }

    /// Connect to selection signal
    pub fn bind(mut self, selected: RwSignal<Option<String>>) -> Self { self.selected = Some(selected);
        self
    }

    /// Connect to open state (for auto-close)
    pub fn auto_close(mut self, is_open: RwSignal<bool>) -> Self { self.is_open = Some(is_open);
        self
    }
}


impl HasViewId for ComboboxItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ComboboxItem {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let value = self.value;
        let label = self.label;
        let disabled = self.disabled;
        let selected = self.selected;
        let is_open = self.is_open;

        let value_clone = value.clone();

        Box::new(
            floem::views::Label::new(label)
                .style(move |s| {
                    let val = value.clone();
                    s.with_shadcn_theme(move |s, t| {
                        let is_selected = selected
                            .map(|s| s.get() == Some(val.clone()))
                            .unwrap_or(false);
                        let base = s
                            .width_full()
                            .padding_left(8.0)
                            .padding_right(8.0)
                            .padding_top(8.0)
                            .padding_bottom(8.0)
                            .font_size(14.0)
                            .border_radius(4.0)
                            .cursor(if disabled {
                                CursorStyle::Default
                            } else {
                                CursorStyle::Pointer
                            });
                        if is_selected {
                            base.background(t.accent).color(t.accent_foreground)
                        } else if disabled {
                            base.color(t.muted_foreground)
                        } else {
                            base.color(t.foreground)
                                .hover(|s| s.background(t.accent).color(t.accent_foreground))
                        }
                    })
                })
                .on_click_stop(move |_| {
                    if !disabled {
                        if let Some(signal) = selected {
                            signal.set(Some(value_clone.clone()));
                        }
                        if let Some(open_signal) = is_open {
                            open_signal.set(false);
                        }
                    }
                }),
        )
    }
}

// ============================================================================
// ComboboxSeparator
// ============================================================================

/// Separator between combobox items
pub struct ComboboxSeparator;

impl ComboboxSeparator {
    /// Create a new separator
    pub fn new() -> Self {
        Self
    }
}

impl Default for ComboboxSeparator {
    fn default() -> Self {
        Self::new()
    }
}


impl HasViewId for ComboboxSeparator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}

impl IntoView for ComboboxSeparator {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .height(1.0)
                    .background(t.border)
                    .margin_top(4.0)
                    .margin_bottom(4.0)
            })
        }))
    }
}
