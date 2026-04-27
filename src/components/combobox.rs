//! Combobox component with builder-style API
//!
//! Based on shadcn/ui Combobox - autocomplete/searchable select.
//!
//! # Example
//!
//! ``````rust
//! use floem::reactive::RwSignal;
//! use floem::view::ParentView;
//! use floem_shadcn::components::combobox::*;
//!
//! let selected = RwSignal::new(None::<String>);
//! let search = RwSignal::new(String::new());
//!
//! Combobox::new(selected, search)
//!     .child(ComboboxTrigger::new("Select framework..."))
//!     .child(
//!         ComboboxContent::new()
//!             .child(ComboboxInput::new())
//!             .child(
//!                 ComboboxList::new()
//!                     .child(ComboboxItem::new("next", "Next.js"))
//!                     .child(ComboboxItem::new("sveltekit", "SvelteKit"))
//!                     .child(ComboboxItem::new("nuxt", "Nuxt.js")),
//!             )
//!             .child(ComboboxEmpty::new("No results found.")),
//!     );
//! ```

use crate::theme::ShadcnThemeExt;
use floem::context::LayoutChanged;
use floem::prelude::*;
use floem::reactive::{Context, RwSignal, Scope, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

/// Combobox context that holds the shared signals.
///
/// Provided via `Scope::provide_context` and accessed by child components
/// using `Context::get::<ComboboxContext>()`.
#[derive(Clone, Copy)]
pub struct ComboboxContext {
    pub selected: RwSignal<Option<String>>,
    pub search: RwSignal<String>,
    pub is_open: RwSignal<bool>,
    /// Trigger position (window coordinates) – set by ComboboxTrigger
    pub trigger_origin: RwSignal<floem::kurbo::Point>,
    /// Trigger size – set by ComboboxTrigger
    pub trigger_size: RwSignal<floem::kurbo::Size>,
}

/// Combobox root component that provides context to children.
///
/// Contains trigger and content. Implements ParentView so children can be
/// added with `.child()`.
pub struct Combobox {
    id: ViewId,
    selected: RwSignal<Option<String>>,
    search: RwSignal<String>,
    is_open: RwSignal<bool>,
    scope: Scope,
}
impl Combobox {
    pub fn new(selected: RwSignal<Option<String>>, search: RwSignal<String>) -> Self {
        let is_open = RwSignal::new(false);
        let trigger_origin = RwSignal::new(floem::kurbo::Point::ZERO);
        let trigger_size = RwSignal::new(floem::kurbo::Size::ZERO);
        let scope = Scope::current().create_child();
        scope.provide_context(ComboboxContext {
            selected,
            search,
            is_open,
            trigger_origin,
            trigger_size,
        });
        Self {
            id: ViewId::new(),
            selected,
            search,
            is_open,
            scope,
        }
    }
    pub fn is_open_signal(&self) -> RwSignal<bool> {
        self.is_open
    }
    pub fn selected_signal(&self) -> RwSignal<Option<String>> {
        self.selected
    }
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
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let scope = self.scope;
        let id = self.id;
        scope.enter(move || Container::with_id(id, ()))
    }
}
impl ParentView for Combobox {
    fn scope(&self) -> Option<Scope> {
        Some(self.scope)
    }
}

/// Trigger button that opens/closes the combobox dropdown.
///
/// Reads the combobox signals from context and displays the selected value
/// or placeholder text.
pub struct ComboboxTrigger {
    id: ViewId,
    placeholder: String,
    items: Vec<(String, String)>,
}
impl ComboboxTrigger {
    pub fn new(p: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            placeholder: p.into(),
            items: vec![],
        }
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let ctx = Context::get::<ComboboxContext>();
        if let Some(ctx) = ctx {
            let selected = ctx.selected;
            let is_open = ctx.is_open;
            let items = self.items.clone();
            let trigger_origin = ctx.trigger_origin;
            let trigger_size = ctx.trigger_size;
            Box::new(
                floem::views::Stack::horizontal((
                    floem::views::Label::derived(move || {
                        if let Some(v) = selected.get() {
                            items
                                .iter()
                                .find(|(x, _)| x == &v)
                                .map(|(_, l)| l.clone())
                                .unwrap_or(v)
                        } else {
                            self.placeholder.clone()
                        }
                    })
                    .style(move |s| {
                        s.with_shadcn_theme(move |s, t| {
                            let hv = selected.get().is_some();
                            s.flex_grow(1.0).font_size(14.0).color(if hv {
                                t.foreground
                            } else {
                                t.muted_foreground
                            })
                        })
                    }),
                    floem::views::Label::new("▼").style(|s| {
                        s.with_shadcn_theme(move |s, t| {
                            s.font_size(10.0).color(t.muted_foreground).flex_shrink(0.0)
                        })
                    }),
                ))
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.min_width(200.0)
                            .height(36.0)
                            .padding_left(12.0)
                            .padding_right(12.0)
                            .padding_top(8.0)
                            .padding_bottom(8.0)
                            .gap(8.0)
                            .items_center()
                            .border(1.0)
                            .border_color(t.input)
                            .border_radius(6.0)
                            .background(t.background)
                            .box_shadow_blur(2.0)
                            .box_shadow_color(peniko::Color::from_rgba8(0, 0, 0, 25))
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.border_color(t.ring))
                    })
                })
                .on_event_stop(
                    LayoutChanged::listener(),
                    move |_cx, event: &LayoutChanged| {
                        trigger_origin.set(event.new_window_origin);
                        trigger_size.set(event.new_box.size());
                    },
                )
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    is_open.update(|v| *v = !*v);
                }),
            )
        } else {
            Box::new(floem::views::Label::new(self.placeholder).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.min_width(200.0)
                        .height(36.0)
                        .padding_left(12.0)
                        .padding_right(12.0)
                        .padding_top(8.0)
                        .padding_bottom(8.0)
                        .items_center()
                        .border(1.0)
                        .border_color(t.input)
                        .border_radius(6.0)
                        .background(t.background)
                        .color(t.muted_foreground)
                })
            }))
        }
    }
}

/// Dropdown content container with overlay positioning.
///
/// Creates an overlay with backdrop for click-outside-to-close behavior.
/// Use `.child()` to add children. Context is automatically available.
pub struct ComboboxContent {
    id: ViewId,
}
impl ComboboxContent {
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}
impl Default for ComboboxContent {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ComboboxContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ComboboxContent {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let ctx = Context::get::<ComboboxContext>();
        let stem = Container::with_id(self.id, ()).style(|s| s.flex_col().width_full());
        if let Some(ctx) = ctx {
            let trigger_origin = ctx.trigger_origin;
            let trigger_size = ctx.trigger_size;
            Box::new(floem::views::Container::new(stem).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let origin = trigger_origin.get();
                    let size = trigger_size.get();
                    s.position(floem::style::Position::Absolute)
                        .inset_left(origin.x)
                        .inset_top(origin.y + size.height + 6.0)
                        .min_width(size.width.max(200.0))
                        .flex_col()
                        .background(t.popover)
                        .color(t.popover_foreground)
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(6.0)
                        .box_shadow_blur(8.0)
                        .box_shadow_color(peniko::Color::from_rgba8(0, 0, 0, 60))
                        .z_index(100)
                        .apply_if(!ctx.is_open.get(), |s| {
                            s.display(floem::style::Display::None)
                        })
                })
            }))
        } else {
            Box::new(stem)
        }
    }
}
impl ParentView for ComboboxContent {}

/// Search input for filtering items.
pub struct ComboboxInput {
    id: ViewId,
    placeholder: String,
}
impl ComboboxInput {
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            placeholder: "Search...".into(),
        }
    }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::TextInput::new(RwSignal::new(String::new()))
                .placeholder(self.placeholder)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.width_full()
                            .height(32.0)
                            .padding_left(12.0)
                            .padding_right(12.0)
                            .font_size(14.0)
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

/// Scrollable list container for combobox items.
///
/// Use `.child()` to add items. Context is automatically available to children.
pub struct ComboboxList {
    id: ViewId,
    max_height: f64,
}
impl ComboboxList {
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            max_height: 300.0,
        }
    }
    pub fn max_height(mut self, h: f64) -> Self {
        self.max_height = h;
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
    type V = floem::views::Scroll;
    type Intermediate = floem::views::Scroll;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let mh = self.max_height;
        let c = Container::with_id(self.id, ()).style(|s| s.flex_col().width_full().padding(4.0));
        floem::views::Scroll::new(c).style(move |s| s.max_height(mh).width_full())
    }
}
impl ParentView for ComboboxList {}

/// Individual combobox item that reads selection from context.
pub struct ComboboxItem {
    id: ViewId,
    value: String,
    label: String,
    disabled: bool,
}
impl ComboboxItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let ctx = Context::get::<ComboboxContext>();
        if let Some(ctx) = ctx {
            let selected = ctx.selected;
            let is_open = ctx.is_open;
            let value = self.value;
            let label = self.label;
            let disabled = self.disabled;
            let v0 = value.clone();
            let v1 = value.clone();
            let v2 = value.clone();
            Box::new(
                floem::views::Container::new(
                    floem::views::Stack::horizontal((
                        floem::views::Label::new(label).style(|s| s.font_size(14.0).flex_grow(1.0)),
                        floem::views::Label::new("✓").style(move |s| {
                            let v = v0.clone();
                            s.with_shadcn_theme(move |s, t| {
                                let is_sel = selected.get() == Some(v.clone());
                                s.size(16.0, 16.0)
                                    .font_size(14.0)
                                    .color(t.foreground)
                                    .items_center()
                                    .justify_center()
                                    .flex_shrink(0.0)
                                    .apply_if(!is_sel, |s| s.display(floem::style::Display::None))
                            })
                        }),
                    ))
                    .style(|s| s.width_full().items_center().gap(8.0)),
                )
                .style(move |s| {
                    let v = v1.clone();
                    s.with_shadcn_theme(move |s, t| {
                        let is_sel = selected.get() == Some(v.clone());
                        let base = s
                            .width_full()
                            .padding_top(6.0)
                            .padding_bottom(6.0)
                            .padding_left(8.0)
                            .padding_right(8.0)
                            .items_center()
                            .border_radius(3.0)
                            .cursor(if disabled {
                                CursorStyle::Default
                            } else {
                                CursorStyle::Pointer
                            });
                        if is_sel {
                            base.background(t.accent).color(t.accent_foreground)
                        } else if disabled {
                            base.color(t.muted_foreground).opacity(0.5)
                        } else {
                            base.color(t.foreground)
                                .hover(|s| s.background(t.accent).color(t.accent_foreground))
                        }
                    })
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    if !disabled {
                        selected.set(Some(v2.clone()));
                        is_open.set(false);
                    }
                }),
            )
        } else {
            Box::new(floem::views::Label::new(self.label).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width_full()
                        .padding(6.0)
                        .font_size(14.0)
                        .color(t.foreground)
                })
            }))
        }
    }
}

/// Empty state shown when no items match the search.
pub struct ComboboxEmpty {
    id: ViewId,
    text: String,
}
impl ComboboxEmpty {
    pub fn new(t: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: t.into(),
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Label::new(self.text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .font_size(14.0)
                    .color(t.muted_foreground)
                    .justify_center()
            })
        }))
    }
}

/// Group of related combobox items with a label.
pub struct ComboboxGroup {
    id: ViewId,
}
impl ComboboxGroup {
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
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| s.flex_col().width_full())
    }
}
impl ParentView for ComboboxGroup {}

/// Label for a combobox group.
pub struct ComboboxLabel {
    id: ViewId,
    text: String,
}
impl ComboboxLabel {
    pub fn new(t: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: t.into(),
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Label::new(self.text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.padding_left(8.0)
                    .padding_right(8.0)
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .font_size(12.0)
                    .color(t.muted_foreground)
            })
        }))
    }
}

/// Separator between combobox items.
pub struct ComboboxSeparator;
impl ComboboxSeparator {
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
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
