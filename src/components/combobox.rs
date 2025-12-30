//! Combobox component with builder-style API
//!
//! Based on shadcn/ui Combobox - autocomplete/searchable select.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem::view::ParentView;
//! use floem_shadcn::components::combobox::*;
//!
//! let selected = RwSignal::new(None::<String>);
//! let search = RwSignal::new(String::new());
//!
//! Combobox::new(selected, search)
//!     .child(ComboboxTrigger::new("Select framework..."))
//!     .child(ComboboxContent::new((
//!         ComboboxInput::new(),
//!         ComboboxList::new()
//!             .child(ComboboxItem::new("next", "Next.js"))
//!             .child(ComboboxItem::new("sveltekit", "SvelteKit"))
//!             .child(ComboboxItem::new("nuxt", "Nuxt.js")),
//!         ComboboxEmpty::new("No results found."),
//!     )));
//! ```

use floem::prelude::*;
use floem::reactive::{Context, RwSignal, Scope, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::text::TextInput;
use crate::theme::ShadcnThemeExt;

// ============================================================================
// ComboboxContext - passes signals to children via reactive Context
// ============================================================================

/// Combobox context that holds the shared signals
///
/// This is provided via `Scope::provide_context` and can be accessed by child
/// components using `Context::get::<ComboboxContext>()`.
#[derive(Clone, Copy)]
pub struct ComboboxContext {
    pub selected: RwSignal<Option<String>>,
    pub search: RwSignal<String>,
    pub is_open: RwSignal<bool>,
}

// ============================================================================
// Combobox
// ============================================================================

/// Combobox root component that provides context to children
///
/// Contains trigger and content. Uses internal state management
/// that is shared via context with child components.
///
/// Implements ParentView so children can be added with `.child()`.
pub struct Combobox {
    id: ViewId,
    selected: RwSignal<Option<String>>,
    search: RwSignal<String>,
    is_open: RwSignal<bool>,
    scope: Scope,
}

impl Combobox {
    /// Create a new combobox with the given selection and search signals
    ///
    /// # Example
    /// ```rust
    /// Combobox::new(selected, search)
    ///     .child(ComboboxTrigger::new("Select..."))
    ///     .child(ComboboxContent::new((
    ///         ComboboxInput::new(),
    ///         ComboboxList::new()
    ///             .child(ComboboxItem::new("a", "Option A"))
    ///             .child(ComboboxItem::new("b", "Option B")),
    ///         ComboboxEmpty::new("No results"),
    ///     )))
    /// ```
    pub fn new(selected: RwSignal<Option<String>>, search: RwSignal<String>) -> Self {
        let is_open = RwSignal::new(false);
        let scope = Scope::current().create_child();

        // Provide the combobox context in the child scope
        scope.provide_context(ComboboxContext {
            selected,
            search,
            is_open,
        });

        Self {
            id: ViewId::new(),
            selected,
            search,
            is_open,
            scope,
        }
    }

    /// Get the open signal for external control
    pub fn is_open_signal(&self) -> RwSignal<bool> {
        self.is_open
    }

    /// Get the selected signal
    pub fn selected_signal(&self) -> RwSignal<Option<String>> {
        self.selected
    }

    /// Get the search signal
    pub fn search_signal(&self) -> RwSignal<String> {
        self.search
    }
}

impl HasViewId for Combobox {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Combobox {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let scope = self.scope;
        let id = self.id;

        // Build the Stem within the combobox's scope so children have access to context
        scope.enter(move || floem::views::Stem::with_id(id))
    }
}

impl ParentView for Combobox {}

// ============================================================================
// ComboboxTrigger
// ============================================================================

/// Trigger button that opens/closes the combobox dropdown
///
/// Reads the combobox signals from context and displays the selected value
/// or placeholder text.
pub struct ComboboxTrigger {
    id: ViewId,
    placeholder: String,
    items: Vec<(String, String)>, // (value, label) pairs for display
}

impl ComboboxTrigger {
    /// Create a new trigger with placeholder text
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            placeholder: placeholder.into(),
            items: Vec::new(),
        }
    }

    /// Add item mappings for displaying selected label
    ///
    /// This is needed so the trigger can show the label for the selected value.
    pub fn items(
        mut self,
        items: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.items = items
            .into_iter()
            .map(|(v, l)| (v.into(), l.into()))
            .collect();
        self
    }
}

impl HasViewId for ComboboxTrigger {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ComboboxTrigger {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let ctx = Context::get::<ComboboxContext>();
        let placeholder = self.placeholder;
        let items = self.items;

        if let Some(ctx) = ctx {
            let selected = ctx.selected;
            let is_open = ctx.is_open;
            let items_for_label = items.clone();

            Box::new(
                floem::views::Stack::horizontal((
                    // Selected value or placeholder
                    floem::views::Label::derived(move || {
                        if let Some(val) = selected.get() {
                            items_for_label
                                .iter()
                                .find(|(v, _)| v == &val)
                                .map(|(_, l)| l.clone())
                                .unwrap_or(val)
                        } else {
                            placeholder.clone()
                        }
                    })
                    .style(move |s| {
                        s.with_shadcn_theme(move |s, t| {
                            let has_value = selected.get().is_some();
                            s.flex_grow(1.0).text_sm().color(if has_value {
                                t.foreground
                            } else {
                                t.muted_foreground
                            })
                        })
                    }),
                    // ChevronDown icon
                    floem::views::Label::new("▼").style(|s| {
                        s.with_shadcn_theme(move |s, t| {
                            s.font_size(10.0).color(t.muted_foreground).flex_shrink(0.0)
                        })
                    }),
                ))
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.min_width(200.0)
                            .h_9()
                            .px_3()
                            .py_2()
                            .gap_2()
                            .items_center()
                            .border_1()
                            .border_color(t.input)
                            .rounded_md()
                            .background(t.background)
                            .shadow_sm()
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.border_color(t.ring))
                    })
                })
                .on_click_stop(move |_| {
                    is_open.update(|v| *v = !*v);
                }),
            )
        } else {
            // No context - render static trigger
            Box::new(floem::views::Label::new(placeholder).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.min_width(200.0)
                        .h_9()
                        .px_3()
                        .py_2()
                        .items_center()
                        .border_1()
                        .border_color(t.input)
                        .rounded_md()
                        .background(t.background)
                        .color(t.muted_foreground)
                })
            }))
        }
    }
}

// ============================================================================
// ComboboxContent
// ============================================================================

/// Dropdown content container with overlay positioning
///
/// Contains the search input, list, and empty state.
pub struct ComboboxContent {
    id: ViewId,
    children: Vec<Box<dyn View>>,
}

impl ComboboxContent {
    /// Create new content with children
    pub fn new(children: impl floem::view::IntoViewIter) -> Self {
        Self {
            id: ViewId::new(),
            children: children.into_view_iter().collect(),
        }
    }
}

impl HasViewId for ComboboxContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ComboboxContent {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let id = self.id;
        let children = self.children;
        let ctx = Context::get::<ComboboxContext>();

        if let Some(ctx) = ctx {
            let is_open = ctx.is_open;
            let search = ctx.search;

            // Track trigger position for overlay positioning
            let trigger_origin = RwSignal::new(floem::kurbo::Point::ZERO);
            let trigger_size = RwSignal::new(floem::kurbo::Size::ZERO);

            Box::new(
                floem::views::Overlay::with_id(
                    id,
                    floem::views::Stack::new((
                        // Backdrop - closes dropdown when clicking outside
                        floem::views::Empty::new()
                            .style(move |s| s.absolute().inset_0())
                            .on_click_stop(move |_| {
                                is_open.set(false);
                                search.set(String::new());
                            }),
                        // Dropdown content
                        floem::views::Stack::vertical_from_iter(children).style(move |s| {
                            s.with_shadcn_theme(move |s, t| {
                                let origin = trigger_origin.get();
                                let size = trigger_size.get();
                                s.absolute()
                                    .inset_left(origin.x)
                                    .inset_top(origin.y + size.height + 6.0)
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
                )
                .on_move(move |origin| {
                    trigger_origin.set(origin);
                })
                .on_resize(move |rect| {
                    trigger_size.set(rect.size());
                }),
            )
        } else {
            // No context - just render content
            Box::new(floem::views::Stack::vertical_from_iter(children))
        }
    }
}

// ============================================================================
// ComboboxInput
// ============================================================================

/// Search input for filtering items
pub struct ComboboxInput {
    id: ViewId,
    placeholder: String,
}

impl ComboboxInput {
    /// Create a new search input
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            placeholder: "Search...".to_string(),
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl Default for ComboboxInput {
    fn default() -> Self {
        Self::new()
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
        let ctx = Context::get::<ComboboxContext>();
        let placeholder = self.placeholder;

        if let Some(ctx) = ctx {
            let search = ctx.search;

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
                                .h_8()
                                .px_3()
                                .text_sm()
                                .border(0.0)
                                .border_bottom(1.0)
                                .border_color(t.border)
                                .background(floem::peniko::Color::TRANSPARENT)
                                .color(t.foreground)
                        })
                    }),
            )
        } else {
            Box::new(TextInput::new().placeholder(placeholder).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width_full()
                        .h_8()
                        .px_3()
                        .text_sm()
                        .border(0.0)
                        .border_bottom(1.0)
                        .border_color(t.border)
                        .background(floem::peniko::Color::TRANSPARENT)
                        .color(t.foreground)
                })
            }))
        }
    }
}

// ============================================================================
// ComboboxList
// ============================================================================

/// Scrollable list container for combobox items
///
/// Implements ParentView so items can be added with `.child()`.
pub struct ComboboxList {
    id: ViewId,
    max_height: f64,
}

impl ComboboxList {
    /// Create a new list container
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            max_height: 300.0,
        }
    }

    /// Set maximum height before scrolling
    pub fn max_height(mut self, height: f64) -> Self {
        self.max_height = height;
        self
    }
}

impl Default for ComboboxList {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for ComboboxList {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ComboboxList {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let max_height = self.max_height;

        floem::views::Stem::with_id(self.id).style(move |s| s.flex_col().width_full().p_1())
    }
}

impl ParentView for ComboboxList {}

// ============================================================================
// ComboboxItem
// ============================================================================

/// Individual combobox item that reads selection from context
pub struct ComboboxItem {
    id: ViewId,
    value: String,
    label: String,
    disabled: bool,
}

impl ComboboxItem {
    /// Create a new item
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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
        let ctx = Context::get::<ComboboxContext>();
        let value = self.value;
        let label = self.label;
        let disabled = self.disabled;

        let value_for_check = value.clone();
        let value_for_style = value.clone();
        let value_for_click = value.clone();
        let label_for_filter = label.clone();

        if let Some(ctx) = ctx {
            let selected = ctx.selected;
            let search = ctx.search;
            let is_open = ctx.is_open;

            Box::new(
                floem::views::Container::new(
                    floem::views::Stack::horizontal((
                        // Label text
                        floem::views::Label::new(label).style(|s| s.text_sm().flex_grow(1.0)),
                        // Check icon (visible when selected)
                        floem::views::Label::new("✓").style(move |s| {
                            let val = value_for_check.clone();
                            s.with_shadcn_theme(move |s, t| {
                                let is_selected = selected.get() == Some(val.clone());
                                s.size_4()
                                    .text_sm()
                                    .color(t.foreground)
                                    .items_center()
                                    .justify_center()
                                    .flex_shrink(0.0)
                                    .apply_if(!is_selected, |s| {
                                        s.display(floem::style::Display::None)
                                    })
                            })
                        }),
                    ))
                    .style(|s| s.width_full().items_center().gap_2()),
                )
                .style(move |s| {
                    let val = value_for_style.clone();
                    let lbl = label_for_filter.clone();
                    s.with_shadcn_theme(move |s, t| {
                        let search_val = search.get();
                        let matches_search = search_val.is_empty()
                            || lbl.to_lowercase().contains(&search_val.to_lowercase());
                        let is_selected = selected.get() == Some(val.clone());

                        let base = s
                            .width_full()
                            .padding_top(6.0)
                            .padding_bottom(6.0)
                            .padding_left(8.0)
                            .padding_right(8.0)
                            .items_center()
                            .rounded_sm()
                            .cursor(if disabled {
                                CursorStyle::Default
                            } else {
                                CursorStyle::Pointer
                            });

                        if !matches_search {
                            base.display(floem::style::Display::None)
                        } else if is_selected {
                            base.background(t.accent).color(t.accent_foreground)
                        } else if disabled {
                            base.color(t.muted_foreground).opacity_50()
                        } else {
                            base.color(t.foreground)
                                .hover(|s| s.background(t.accent).color(t.accent_foreground))
                        }
                    })
                })
                .on_click_stop(move |_| {
                    if !disabled {
                        selected.set(Some(value_for_click.clone()));
                        is_open.set(false);
                        search.set(String::new());
                    }
                }),
            )
        } else {
            // No context - render static item
            Box::new(floem::views::Label::new(label).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width_full().padding(6.0).text_sm().color(t.foreground)
                })
            }))
        }
    }
}

// ============================================================================
// ComboboxEmpty
// ============================================================================

/// Empty state shown when no items match the search
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
        let text = self.text;

        // Note: visibility based on whether items are filtered is handled
        // by the parent - this just provides the empty state view
        Box::new(floem::views::Label::new(text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .text_sm()
                    .color(t.muted_foreground)
                    .justify_center()
            })
        }))
    }
}

// ============================================================================
// ComboboxGroup
// ============================================================================

/// Group of related combobox items with a label
///
/// Note: Since Stem doesn't support prepending a label, use ComboboxLabel
/// before the group's items instead.
pub struct ComboboxGroup {
    id: ViewId,
}

impl ComboboxGroup {
    /// Create a new group container
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for ComboboxGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for ComboboxGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ComboboxGroup {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| s.flex_col().width_full())
    }
}

impl ParentView for ComboboxGroup {}

// ============================================================================
// ComboboxLabel
// ============================================================================

/// Label for a combobox group
pub struct ComboboxLabel {
    id: ViewId,
    text: String,
}

impl ComboboxLabel {
    /// Create a new group label
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for ComboboxLabel {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ComboboxLabel {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Label::new(self.text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.px_2()
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .text_xs()
                    .font_medium()
                    .color(t.muted_foreground)
            })
        }))
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
                    .margin_left(-4.0)
                    .margin_right(-4.0)
                    .margin_top(4.0)
                    .margin_bottom(4.0)
            })
        }))
    }
}
