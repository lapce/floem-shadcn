//! Select component with builder-style API
//!
//! Based on shadcn/ui Select component - a dropdown for selecting from a list.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::select::*;
//!
//! let selected = RwSignal::new(Some("option1".to_string()));
//!
//! Select::new(selected)
//!     .placeholder("Select an option...")
//!     .items(vec![
//!         SelectItemData::new("option1", "Option 1"),
//!         SelectItemData::new("option2", "Option 2"),
//!         SelectItemData::new("option3", "Option 3"),
//!     ]);
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// SelectItemData (data structure)
// ============================================================================

/// Data for a select item
#[derive(Clone)]
pub struct SelectItemData {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectItemData {
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
// Select (main component)
// ============================================================================

/// A styled select dropdown
pub struct Select {
    id: ViewId,
    selected: RwSignal<Option<String>>,
    placeholder: String,
    items: Vec<SelectItemData>,
    disabled: bool,
}

impl Select {
    /// Create a new select
    pub fn new(selected: RwSignal<Option<String>>) -> Self {
        Self {
            id: ViewId::new(),
            selected,
            placeholder: "Select...".to_string(),
            items: Vec::new(),
            disabled: false,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set items
    pub fn items(mut self, items: Vec<SelectItemData>) -> Self {
        self.items = items;
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl HasViewId for Select {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Select {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let selected = self.selected;
        let placeholder = self.placeholder;
        let items = self.items;
        let disabled = self.disabled;
        let is_open = RwSignal::new(false);

        let items_for_trigger = items.clone();

        // shadcn/ui SelectTrigger (v4 new-york):
        // border-input rounded-md bg-transparent px-3 py-2 text-sm shadow-xs
        // data-[size=default]:h-9 (36px)
        // ChevronDownIcon size-4 opacity-50
        let trigger = floem::views::h_stack((
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
            // ChevronDown icon - size-4 opacity-50
            floem::views::Label::new("▼").style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.font_size(10.0)
                        .color(t.muted_foreground)
                        .flex_shrink(0.0)
                })
            }),
        ))
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                s.min_width(120.0) // min-w-[8rem]
                    .h_9() // h-9 = 36px
                    .px_3() // px-3 = 12px
                    .py_2() // py-2 = 8px
                    .gap_2() // gap-2 = 8px
                    .items_center()
                    .border_1() // border
                    .border_color(t.input) // border-input
                    .rounded_md() // rounded-md = 6px
                    .background(t.background) // bg-transparent (using background)
                    .shadow_sm() // shadow-xs
                    .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                    .apply_if(!disabled, |s| {
                        s.cursor(CursorStyle::Pointer)
                            .hover(|s| s.border_color(t.ring))
                    })
            })
        });

        let trigger = if !disabled {
            trigger
                .on_click_stop(move |_| {
                    is_open.update(|v| *v = !*v);
                })
                .into_any()
        } else {
            trigger.into_any()
        };

        // Build items (up to 10)
        let item0 = create_select_item(0, items.clone(), selected, is_open);
        let item1 = create_select_item(1, items.clone(), selected, is_open);
        let item2 = create_select_item(2, items.clone(), selected, is_open);
        let item3 = create_select_item(3, items.clone(), selected, is_open);
        let item4 = create_select_item(4, items.clone(), selected, is_open);
        let item5 = create_select_item(5, items.clone(), selected, is_open);
        let item6 = create_select_item(6, items.clone(), selected, is_open);
        let item7 = create_select_item(7, items.clone(), selected, is_open);
        let item8 = create_select_item(8, items.clone(), selected, is_open);
        let item9 = create_select_item(9, items.clone(), selected, is_open);

        let items_container = floem::views::v_stack((
            item0, item1, item2, item3, item4, item5, item6, item7, item8, item9,
        ))
        .style(|s| s.width_full().max_height(300.0));

        // shadcn/ui SelectContent (v4 new-york):
        // bg-popover text-popover-foreground rounded-md border shadow-md
        // Viewport: p-1
        let dropdown = floem::views::Container::new(items_container)
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let open = is_open.get();
                    let base = s
                        .position(floem::style::Position::Absolute)
                        .inset_top_pct(100.0)
                        .inset_left(0.0)
                        .min_width(120.0) // Match trigger min-width explicitly
                        .margin_top(4.0) // small gap from trigger
                        .p_1() // p-1 = 4px (viewport padding)
                        .background(t.popover) // bg-popover
                        .color(t.popover_foreground) // text-popover-foreground
                        .border_1() // border
                        .border_color(t.border)
                        .rounded_md() // rounded-md
                        .shadow_lg() // shadow-md
                        .z_index(100);
                    if open {
                        base
                    } else {
                        base.display(floem::style::Display::None)
                    }
                })
            });

        // Backdrop to close when clicking outside
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
            });

        Box::new(
            floem::views::Container::new(floem::views::stack((trigger, backdrop, dropdown)))
                .style(|s| s.position(floem::style::Position::Relative).min_width(120.0)),
        )
    }
}

/// Create a select item at the given index
fn create_select_item(
    index: usize,
    items: Vec<SelectItemData>,
    selected: RwSignal<Option<String>>,
    is_open: RwSignal<bool>,
) -> impl IntoView {
    let items_for_label = items.clone();
    let items_for_style = items.clone();
    let items_for_click = items.clone();
    let items_for_handler = items;

    // shadcn/ui SelectItem (v4 new-york):
    // py-1.5 pr-8 pl-2 text-sm rounded-sm
    // focus:bg-accent focus:text-accent-foreground
    // CheckIcon size-4 at absolute right-2
    floem::views::h_stack((
        // Check icon (visible when selected)
        floem::views::Label::new("✓").style(move |s| {
            let items = items_for_label.clone();
            s.with_shadcn_theme(move |s, t| {
                let item_opt = items.get(index);
                let is_selected = item_opt
                    .map(|i| Some(i.value.clone()) == selected.get())
                    .unwrap_or(false);
                s.size_4() // size-4 = 16px
                    .text_sm()
                    .color(t.foreground)
                    .items_center()
                    .justify_center()
                    .apply_if(!is_selected, |s| s.display(floem::style::Display::None))
            })
        }),
        // Label
        floem::views::Label::derived(move || {
            items_for_style
                .get(index)
                .map(|i| i.label.clone())
                .unwrap_or_default()
        })
        .style(|s| s.text_sm()),
    ))
    .style(move |s| {
        let items = items_for_click.clone();
        s.with_shadcn_theme(move |s, t| {
            let item_opt = items.get(index);
            let is_visible = item_opt.is_some();
            let is_selected = item_opt
                .map(|i| Some(i.value.clone()) == selected.get())
                .unwrap_or(false);
            let is_disabled = item_opt.map(|i| i.disabled).unwrap_or(false);

            // py-1.5 = 6px, pl-2 = 8px, pr-8 = 32px (space for check)
            let base = s
                .width_full()
                .padding_top(6.0) // py-1.5 = 6px
                .padding_bottom(6.0)
                .padding_left(8.0) // pl-2 = 8px
                .padding_right(32.0) // pr-8 = 32px
                .gap_2() // gap-2 = 8px
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
                // Selected state
                base.background(t.accent).color(t.accent_foreground)
            } else if is_disabled {
                // Disabled state
                base.color(t.muted_foreground)
            } else {
                // Normal state with hover
                base.color(t.foreground)
                    .hover(|s| s.background(t.accent).color(t.accent_foreground))
            }
        })
    })
    .on_click_stop(move |_| {
        if let Some(item) = items_for_handler.get(index) {
            if !item.disabled {
                selected.set(Some(item.value.clone()));
                is_open.set(false);
            }
        }
    })
}

// ============================================================================
// SelectTrigger (for custom usage)
// ============================================================================

/// Standalone trigger for select
pub struct SelectTrigger<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}

impl<V: IntoView + 'static> SelectTrigger<V> {
    /// Create a new trigger
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child,
            is_open,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SelectTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SelectTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;

        // shadcn/ui SelectTrigger (v4 new-york):
        // border-input rounded-md bg-transparent px-3 py-2 text-sm shadow-xs h-9
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
// SelectContent
// ============================================================================

/// Content container for select dropdown
pub struct SelectContent<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}

impl<V: IntoView + 'static> SelectContent<V> {
    /// Create new content
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child,
            is_open,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SelectContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SelectContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;

        // shadcn/ui SelectContent (v4 new-york):
        // bg-popover text-popover-foreground rounded-md border shadow-md
        // Viewport: p-1
        Box::new(floem::views::Container::with_id(self.id, self.child).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let open = is_open.get();
                let base = s
                    .position(floem::style::Position::Absolute)
                    .inset_top_pct(100.0)
                    .inset_left(0.0)
                    .inset_right(0.0)
                    .margin_top(4.0)
                    .p_1() // p-1 = 4px
                    .background(t.popover) // bg-popover
                    .color(t.popover_foreground) // text-popover-foreground
                    .border_1() // border
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
        }))
    }
}

// ============================================================================
// SelectItem
// ============================================================================

/// Individual select item
pub struct SelectItem {
    id: ViewId,
    value: String,
    label: String,
    disabled: bool,
    selected: Option<RwSignal<Option<String>>>,
    is_open: Option<RwSignal<bool>>,
}

impl SelectItem {
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

impl HasViewId for SelectItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SelectItem {
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

        // shadcn/ui SelectItem (v4 new-york):
        // py-1.5 pr-8 pl-2 text-sm rounded-sm
        // focus:bg-accent focus:text-accent-foreground
        // CheckIcon size-4 at absolute right-2
        Box::new(
            floem::views::h_stack((
                // Check icon
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
                            .apply_if(!is_selected, |s| s.display(floem::style::Display::None))
                    })
                }),
                // Label text
                floem::views::Label::new(label).style(|s| s.text_sm()),
            ))
            .style(move |s| {
                let val = value_for_style.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_selected = selected
                        .map(|sig| sig.get() == Some(val.clone()))
                        .unwrap_or(false);

                    let base = s
                        .width_full()
                        .padding_top(6.0) // py-1.5 = 6px
                        .padding_bottom(6.0)
                        .padding_left(8.0) // pl-2 = 8px
                        .padding_right(32.0) // pr-8 = 32px
                        .gap_2() // gap-2 = 8px
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
// SelectLabel
// ============================================================================

/// Label for a group of select items
pub struct SelectLabel {
    id: ViewId,
    text: String,
}

impl SelectLabel {
    /// Create a new label
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for SelectLabel {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SelectLabel {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        // shadcn/ui SelectLabel (v4 new-york):
        // text-muted-foreground px-2 py-1.5 text-xs
        Box::new(floem::views::Label::with_id(self.id, self.text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.px_2() // px-2 = 8px
                    .padding_top(6.0) // py-1.5 = 6px
                    .padding_bottom(6.0)
                    .text_xs() // text-xs = 12px
                    .color(t.muted_foreground) // text-muted-foreground
            })
        }))
    }
}

// ============================================================================
// SelectSeparator
// ============================================================================

/// Separator between select items
pub struct SelectSeparator;

impl SelectSeparator {
    /// Create a new separator
    pub fn new() -> Self {
        Self
    }
}

impl Default for SelectSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SelectSeparator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}

impl IntoView for SelectSeparator {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        // shadcn/ui SelectSeparator (v4 new-york):
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

// ============================================================================
// SelectGroup
// ============================================================================

/// Group of related select items with a label
pub struct SelectGroup<V> {
    id: ViewId,
    label: String,
    child: V,
}

impl<V: IntoView + 'static> SelectGroup<V> {
    /// Create a new group
    pub fn new(label: impl Into<String>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SelectGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SelectGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let label_view = SelectLabel::new(self.label);
        Box::new(floem::views::v_stack((label_view, self.child)))
    }
}
