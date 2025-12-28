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
//!         ComboboxItemData::new("next", "Next.js"),
//!         ComboboxItemData::new("sveltekit", "SvelteKit"),
//!         ComboboxItemData::new("nuxt", "Nuxt.js"),
//!     ]);
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::text::TextInput;
use crate::theme::ShadcnThemeExt;

// ============================================================================
// ComboboxItemData (data structure)
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
    pub fn new(selected: RwSignal<Option<String>>, search: RwSignal<String>) -> Self {
        Self {
            id: ViewId::new(),
            selected,
            search,
            placeholder: "Select...".to_string(),
            items: Vec::new(),
            empty_text: "No results found.".to_string(),
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set items
    pub fn items(mut self, items: Vec<ComboboxItemData>) -> Self {
        self.items = items;
        self
    }

    /// Set empty state text
    pub fn empty_text(mut self, text: impl Into<String>) -> Self {
        self.empty_text = text.into();
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

        // Track trigger position (window coords via on_move) and size (via on_resize)
        // for positioning the Overlay dropdown below the trigger.
        let trigger_origin = RwSignal::new(floem::kurbo::Point::ZERO);
        let trigger_size = RwSignal::new(floem::kurbo::Size::ZERO);

        // Clone items for different closures
        let items_for_trigger = items.clone();
        let items_for_empty = items.clone();

        // shadcn/ui ComboboxTrigger (v4):
        // Similar to SelectTrigger - border-input rounded-md px-3 py-2 text-sm shadow-xs h-9
        let trigger = floem::views::Stack::horizontal((
            // Selected value or placeholder
            floem::views::Label::derived(move || {
                if let Some(val) = selected.get() {
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
                    // data-[placeholder]:text-muted-foreground
                    s.flex_grow(1.0).text_sm().color(if has_value {
                        t.foreground
                    } else {
                        t.muted_foreground
                    })
                })
            }),
            // ChevronDown icon - size-4 text-muted-foreground
            floem::views::Label::new("▼").style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.font_size(10.0).color(t.muted_foreground).flex_shrink(0.0)
                })
            }),
        ))
        .style(|s| {
            s.with_shadcn_theme(move |s, t| {
                // border-input rounded-md bg-transparent px-3 py-2 text-sm shadow-xs h-9
                s.min_width(200.0)
                    .h_9() // h-9 = 36px
                    .px_3() // px-3 = 12px
                    .py_2() // py-2 = 8px
                    .gap_2() // gap-2
                    .items_center()
                    .border_1() // border
                    .border_color(t.input) // border-input
                    .rounded_md() // rounded-md
                    .background(t.background)
                    .shadow_sm() // shadow-xs
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.border_color(t.ring))
            })
        })
        // Track trigger position (window coords) and size for dropdown placement
        .on_move(move |origin| {
            trigger_origin.set(origin);
        })
        .on_resize(move |rect| {
            trigger_size.set(rect.size());
        })
        .on_click_stop(move |_| {
            is_open.update(|v| *v = !*v);
        });

        // Search input in dropdown using custom TextInput
        // shadcn/ui: h-8 (32px), px-3 (12px), text-sm
        let search_input = TextInput::new()
            .placeholder("Search...")
            .value(move || search.get())
            .on_update(move |text| {
                search.set(text.to_string());
            })
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    // Input inside dropdown - no outer border, just bottom border
                    s.width_full()
                        .h_8() // h-8 = 32px
                        .px_3() // px-3 = 12px
                        .text_sm() // text-sm = 14px
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

        // shadcn/ui ComboboxEmpty (v4):
        // text-muted-foreground py-2 text-center text-sm
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
                    .padding_top(8.0) // py-2 = 8px
                    .padding_bottom(8.0)
                    .text_sm() // text-sm = 14px
                    .color(t.muted_foreground) // text-muted-foreground
                    .justify_center();
                if !has_results && !search_val.is_empty() {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
        });

        // shadcn/ui ComboboxList (v4):
        // scroll-py-1 overflow-y-auto p-1
        let items_container = floem::views::Stack::vertical((
            item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, empty_view,
        ))
        .style(|s| {
            s.max_height(300.0) // max-h-96 ~ 384px but using 300 for practicality
                .p_1() // p-1 = 4px
        });

        // Dropdown in Overlay - escapes parent clipping and z-index constraints
        let dropdown_overlay = floem::views::Overlay::new(
            floem::views::Stack::new((
                // Backdrop - closes dropdown when clicking outside
                floem::views::Empty::new()
                    .style(move |s| {
                        s.absolute().inset_0()
                        // Transparent backdrop - just for click handling
                    })
                    .on_click_stop(move |_| {
                        is_open.set(false);
                        search.set(String::new());
                    }),
                // Dropdown content - positioned relative to trigger using window coordinates
                floem::views::Stack::vertical((search_input, items_container)).style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let origin = trigger_origin.get();
                        let size = trigger_size.get();
                        // Position below the trigger using window coordinates
                        s.absolute()
                            .inset_left(origin.x)
                            .inset_top(origin.y + size.height + 6.0) // 6px gap (sideOffset=6)
                            .min_width(size.width.max(200.0))
                            .background(t.popover)
                            .color(t.popover_foreground)
                            .border_1()
                            .border_color(t.border)
                            .rounded_md()
                            .shadow_lg()
                            .z_index(100)
                    })
                }),
            ))
            .style(move |s| {
                let open = is_open.get();
                s.fixed()
                    .inset_0()
                    .width_full()
                    .height_full()
                    .apply_if(!open, |s| s.hide())
            }),
        );

        Box::new(floem::views::Stack::new((trigger, dropdown_overlay)))
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
    let items_for_click = items.clone();
    let items_for_handler = items;

    // shadcn/ui ComboboxItem (v4):
    // py-1.5 pr-8 pl-2 text-sm rounded-sm
    // data-highlighted:bg-accent data-highlighted:text-accent-foreground
    // CheckIcon size-4 at absolute right-2
    floem::views::Container::new(
        floem::views::Stack::horizontal((
            // Label
            floem::views::Label::derived(move || {
                let search_val = search.get();
                let filtered: Vec<_> = items_for_style
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
            .style(|s| s.text_sm().flex_grow(1.0)),
            // Check icon (visible when selected) - absolute positioned right-2
            floem::views::Label::new("✓").style(move |s| {
                let items = items_for_label.clone();
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
                    let is_selected = item_opt
                        .map(|i| Some(i.value.clone()) == selected.get())
                        .unwrap_or(false);
                    // Positioned at the end via flex, size-4 = 16px
                    s.size_4()
                        .text_sm()
                        .color(t.foreground)
                        .items_center()
                        .justify_center()
                        .flex_shrink(0.0)
                        .apply_if(!is_selected, |s| s.display(floem::style::Display::None))
                })
            }),
        ))
        .style(|s| s.width_full().items_center().gap_2()),
    )
    .style(move |s| {
        let items = items_for_click.clone();
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

            // py-1.5 = 6px, pl-2 = 8px, pr-2 = 8px (check icon is at end via flex)
            let base = s
                .width_full()
                .padding_top(6.0) // py-1.5 = 6px
                .padding_bottom(6.0)
                .padding_left(8.0) // pl-2 = 8px
                .padding_right(8.0) // pr-2 = 8px
                .items_center()
                .rounded_sm() // rounded-sm = 3px
                .cursor(if is_disabled {
                    CursorStyle::Default
                } else {
                    CursorStyle::Pointer
                });

            if !is_visible {
                base.display(floem::style::Display::None)
            } else if is_selected {
                // Selected/highlighted state
                base.background(t.accent).color(t.accent_foreground)
            } else if is_disabled {
                // Disabled state - opacity-50
                base.color(t.muted_foreground).opacity_50()
            } else {
                // Normal state with hover
                base.color(t.foreground)
                    .hover(|s| s.background(t.accent).color(t.accent_foreground))
            }
        })
    })
    .on_click_stop(move |_| {
        let search_val = search.get();
        let filtered: Vec<_> = items_for_handler
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
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child,
            is_open,
        }
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

        // Similar to SelectTrigger styling
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.h_9() // h-9 = 36px
                            .px_3() // px-3 = 12px
                            .py_2() // py-2 = 8px
                            .gap_2() // gap-2
                            .items_center()
                            .border_1() // border
                            .border_color(t.input) // border-input
                            .rounded_md() // rounded-md
                            .background(t.background)
                            .shadow_sm() // shadow-xs
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.border_color(t.ring))
                    })
                })
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
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child,
            is_open,
        }
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

        // shadcn/ui ComboboxContent (v4):
        // bg-popover text-popover-foreground rounded-md shadow-md ring-1
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let open = is_open.get();
                    let base = s
                        .position(floem::style::Position::Absolute)
                        .inset_top_pct(100.0)
                        .inset_left(0.0)
                        .inset_right(0.0)
                        .margin_top(6.0) // sideOffset=6
                        .p_1() // p-1 = 4px
                        .background(t.popover) // bg-popover
                        .color(t.popover_foreground) // text-popover-foreground
                        .border_1() // ring-1
                        .border_color(t.border)
                        .rounded_md() // rounded-md
                        .shadow_lg() // shadow-md
                        .z_index(100)
                        .flex_direction(floem::style::FlexDirection::Column);
                    if open {
                        base
                    } else {
                        base.display(floem::style::Display::None)
                    }
                })
            }),
        )
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
    pub fn new(search: RwSignal<String>) -> Self {
        Self {
            id: ViewId::new(),
            search,
            placeholder: "Search...".to_string(),
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
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

        // Use custom TextInput from text submodule
        Box::new(
            TextInput::new()
                .placeholder(placeholder)
                .value(move || search.get())
                .on_update(move |text| {
                    search.set(text.to_string());
                })
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.width_full()
                            .padding(8.0)
                            .text_sm() // text-sm = 14px
                            .border(0.0)
                            .border_bottom(1.0)
                            .border_color(t.border)
                            .background(floem::peniko::Color::TRANSPARENT)
                            .color(t.foreground)
                    })
                }),
        )
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
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
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
        // shadcn/ui ComboboxEmpty (v4):
        // text-muted-foreground py-2 text-center text-sm
        Box::new(floem::views::Label::with_id(self.id, self.text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .padding_top(8.0) // py-2 = 8px
                    .padding_bottom(8.0)
                    .text_sm() // text-sm = 14px
                    .color(t.muted_foreground) // text-muted-foreground
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
    pub fn new(label: impl Into<String>, child: V) -> Self {
        Self {
            id: ViewId::new(),
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

        // shadcn/ui ComboboxLabel (v4):
        // text-muted-foreground px-2 py-1.5 text-xs
        let label_view = floem::views::Label::new(label).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.px_2() // px-2 = 8px
                    .padding_top(6.0) // py-1.5 = 6px
                    .padding_bottom(6.0)
                    .text_xs() // text-xs = 12px
                    .font_medium()
                    .color(t.muted_foreground) // text-muted-foreground
            })
        });

        Box::new(floem::views::Stack::vertical((label_view, self.child)))
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
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            value: value.into(),
            label: label.into(),
            disabled: false,
            selected: None,
            is_open: None,
        }
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Connect to selection signal
    pub fn bind(mut self, selected: RwSignal<Option<String>>) -> Self {
        self.selected = Some(selected);
        self
    }

    /// Connect to open state (for auto-close)
    pub fn auto_close(mut self, is_open: RwSignal<bool>) -> Self {
        self.is_open = Some(is_open);
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

        let value_for_style = value.clone();
        let value_for_click = value.clone();

        // shadcn/ui ComboboxItem (v4):
        // py-1.5 pr-8 pl-2 text-sm rounded-sm
        // data-highlighted:bg-accent data-highlighted:text-accent-foreground
        // CheckIcon size-4 at absolute right-2
        Box::new(
            floem::views::Container::new(
                floem::views::Stack::horizontal((
                    // Label text
                    floem::views::Label::new(label).style(|s| s.text_sm().flex_grow(1.0)),
                    // Check icon (at end via flex)
                    floem::views::Label::new("✓").style(move |s| {
                        let val = value.clone();
                        s.with_shadcn_theme(move |s, t| {
                            let is_selected = selected
                                .map(|sig| sig.get() == Some(val.clone()))
                                .unwrap_or(false);
                            s.size_4() // size-4 = 16px
                                .text_sm()
                                .color(t.foreground)
                                .items_center()
                                .justify_center()
                                .flex_shrink(0.0)
                                .apply_if(!is_selected, |s| s.display(floem::style::Display::None))
                        })
                    }),
                ))
                .style(|s| s.width_full().items_center().gap_2()),
            )
            .style(move |s| {
                let val = value_for_style.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_selected = selected
                        .map(|sig| sig.get() == Some(val.clone()))
                        .unwrap_or(false);

                    // py-1.5 = 6px, pl-2 = 8px, pr-2 = 8px (check at end via flex)
                    let base = s
                        .width_full()
                        .padding_top(6.0) // py-1.5 = 6px
                        .padding_bottom(6.0)
                        .padding_left(8.0) // pl-2 = 8px
                        .padding_right(8.0) // pr-2 = 8px
                        .items_center()
                        .rounded_sm() // rounded-sm
                        .cursor(if disabled {
                            CursorStyle::Default
                        } else {
                            CursorStyle::Pointer
                        });

                    if is_selected {
                        base.background(t.accent).color(t.accent_foreground)
                    } else if disabled {
                        // Disabled state - opacity-50
                        base.color(t.muted_foreground).opacity_50()
                    } else {
                        base.color(t.foreground)
                            .hover(|s| s.background(t.accent).color(t.accent_foreground))
                    }
                })
            })
            .on_click_stop(move |_| {
                if !disabled {
                    if let Some(signal) = selected {
                        signal.set(Some(value_for_click.clone()));
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
        // shadcn/ui ComboboxSeparator (v4):
        // bg-border -mx-1 my-1 h-px
        Box::new(floem::views::Empty::new().style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .height(1.0) // h-px
                    .background(t.border) // bg-border
                    .margin_left(-4.0) // -mx-1
                    .margin_right(-4.0) // -mx-1
                    .margin_top(4.0) // my-1
                    .margin_bottom(4.0) // my-1
            })
        }))
    }
}
