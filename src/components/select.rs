//! Select component with builder-style API
//!
//! Based on shadcn/ui Select component - a dropdown for selecting from a list.
//!
//! # Example
//!
//! ``````rust
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

use crate::theme::ShadcnThemeExt;
use floem::context::LayoutChanged;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

/// Data for a select item.
#[derive(Clone)]
pub struct SelectItemData {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}
impl SelectItemData {
    pub fn new(v: impl Into<String>, l: impl Into<String>) -> Self {
        Self {
            value: v.into(),
            label: l.into(),
            disabled: false,
        }
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// A styled select dropdown.
pub struct Select {
    id: ViewId,
    selected: RwSignal<Option<String>>,
    placeholder: String,
    items: Vec<SelectItemData>,
    disabled: bool,
    trigger_origin: RwSignal<floem::kurbo::Point>,
    trigger_size: RwSignal<floem::kurbo::Size>,
}
impl Select {
    pub fn new(s: RwSignal<Option<String>>) -> Self {
        Self {
            id: ViewId::new(),
            selected: s,
            placeholder: "Select...".into(),
            items: vec![],
            disabled: false,
            trigger_origin: RwSignal::new(floem::kurbo::Point::ZERO),
            trigger_size: RwSignal::new(floem::kurbo::Size::ZERO),
        }
    }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }
    pub fn items(mut self, i: Vec<SelectItemData>) -> Self {
        self.items = i;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let sel = self.selected;
        let ph = self.placeholder;
        let items = self.items;
        let disabled = self.disabled;
        let trigger_origin = self.trigger_origin;
        let trigger_size = self.trigger_size;
        let is_open = RwSignal::new(false);
        let items_for_trigger = items.clone();
        let trigger = floem::views::Stack::horizontal((
            floem::views::Label::derived(move || {
                if let Some(v) = sel.get() {
                    items_for_trigger
                        .iter()
                        .find(|i| i.value == v)
                        .map(|i| i.label.clone())
                        .unwrap_or(v)
                } else {
                    ph.clone()
                }
            })
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let hv = sel.get().is_some();
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
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                s.min_width(120.0)
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
                    .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                    .apply_if(!disabled, |s| {
                        s.cursor(CursorStyle::Pointer)
                            .hover(|s| s.border_color(t.ring))
                    })
            })
        })
        .on_event_stop(
            LayoutChanged::listener(),
            move |_cx, event: &LayoutChanged| {
                trigger_origin.set(event.new_window_origin);
                trigger_size.set(event.new_box.size());
            },
        );
        let trigger = if !disabled {
            trigger
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    is_open.update(|v| *v = !*v);
                })
                .into_any()
        } else {
            trigger.into_any()
        };
        let to = trigger_origin;
        let ts = trigger_size;
        let item_views: Vec<Box<dyn View>> = items
            .iter()
            .map(|item| {
                let v = item.value.clone();
                let l = item.label.clone();
                let d = item.disabled;
                let sel2 = self.selected;
                let io2 = is_open;
                let vc = v.clone();
                let vs = v.clone();
                let vk = v.clone();
                Box::new(
                    floem::views::Container::new(
                        floem::views::Stack::horizontal((
                            floem::views::Label::new(l).style(|s| s.font_size(14.0).flex_grow(1.0)),
                            floem::views::Label::new("✓").style(move |s| {
                                let v = vc.clone();
                                s.with_shadcn_theme(move |s, t| {
                                    let is_sel = sel2.get() == Some(v.clone());
                                    s.size(16.0, 16.0)
                                        .font_size(14.0)
                                        .color(t.foreground)
                                        .items_center()
                                        .justify_center()
                                        .flex_shrink(0.0)
                                        .apply_if(!is_sel, |s| {
                                            s.display(floem::style::Display::None)
                                        })
                                })
                            }),
                        ))
                        .style(|s| s.width_full().items_center().gap(8.0)),
                    )
                    .style(move |s| {
                        let v = vs.clone();
                        s.with_shadcn_theme(move |s, t| {
                            let is_sel = sel2.get() == Some(v.clone());
                            let base = s
                                .width_full()
                                .padding_top(6.0)
                                .padding_bottom(6.0)
                                .padding_left(8.0)
                                .padding_right(8.0)
                                .items_center()
                                .border_radius(3.0)
                                .cursor(if d {
                                    CursorStyle::Default
                                } else {
                                    CursorStyle::Pointer
                                });
                            if is_sel {
                                base.background(t.accent).color(t.accent_foreground)
                            } else if d {
                                base.color(t.muted_foreground).opacity(0.5)
                            } else {
                                base.color(t.foreground)
                                    .hover(|s| s.background(t.accent).color(t.accent_foreground))
                            }
                        })
                    })
                    .on_event_stop(
                        floem::event::listener::Click,
                        move |_, _| {
                            if !d {
                                sel2.set(Some(vk.clone()));
                                io2.set(false);
                            }
                        },
                    ),
                ) as _
            })
            .collect();
        let items_container = floem::views::Stack::vertical_from_iter(item_views)
            .style(|s| s.width_full().max_height(300.0));
        let dropdown = floem::views::Container::new(items_container).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let origin = to.get();
                let tsize = ts.get();
                s.position(floem::style::Position::Absolute)
                    .inset_left(origin.x)
                    .inset_top(origin.y + tsize.height + 6.0)
                    .min_width(tsize.width.max(120.0))
                    .margin_top(6.0)
                    .padding(4.0)
                    .background(t.popover)
                    .color(t.popover_foreground)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(6.0)
                    .box_shadow_blur(4.0)
                    .box_shadow_color(peniko::Color::from_rgba8(0, 0, 0, 40))
                    .z_index(100)
                    .flex_direction(floem::style::FlexDirection::Column)
                    .apply_if(!is_open.get(), |s| s.display(floem::style::Display::None))
            })
        });
        Box::new(floem::views::Stack::new((trigger, dropdown)))
    }
}

/// Standalone trigger for select.
pub struct SelectTrigger<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}
impl<V: IntoView + 'static> SelectTrigger<V> {
    pub fn new(c: V, io: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child: c,
            is_open: io,
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let io = self.is_open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.height(36.0)
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
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    io.update(|v| *v = !*v);
                }),
        )
    }
}

/// Content container for select dropdown.
pub struct SelectContent<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}
impl<V: IntoView + 'static> SelectContent<V> {
    pub fn new(c: V, io: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child: c,
            is_open: io,
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let io = self.is_open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let base = s
                        .position(floem::style::Position::Absolute)
                        .inset_top_pct(100.0)
                        .inset_left(0.0)
                        .inset_right(0.0)
                        .margin_top(6.0)
                        .padding(4.0)
                        .background(t.popover)
                        .color(t.popover_foreground)
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(6.0)
                        .box_shadow_blur(4.0)
                        .box_shadow_color(peniko::Color::from_rgba8(0, 0, 0, 40))
                        .z_index(100)
                        .flex_direction(floem::style::FlexDirection::Column);
                    if io.get() {
                        base
                    } else {
                        base.display(floem::style::Display::None)
                    }
                })
            }),
        )
    }
}

// Standalone stubs for compatibility with prelude
pub struct SelectItem;
impl SelectItem {
    #[allow(dead_code)]
    pub fn new(_: impl Into<String>, _: impl Into<String>) -> Self {
        Self
    }
    #[allow(dead_code)]
    pub fn disabled(self, _: bool) -> Self {
        self
    }
    #[allow(dead_code)]
    pub fn bind(self, _: RwSignal<Option<String>>) -> Self {
        self
    }
    #[allow(dead_code)]
    pub fn auto_close(self, _: RwSignal<bool>) -> Self {
        self
    }
}
impl HasViewId for SelectItem {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}
impl IntoView for SelectItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new())
    }
}

pub struct SelectLabel;
impl SelectLabel {
    #[allow(dead_code)]
    pub fn new(_: impl Into<String>) -> Self {
        Self
    }
}
impl HasViewId for SelectLabel {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}
impl IntoView for SelectLabel {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new())
    }
}

pub struct SelectSeparator;
impl SelectSeparator {
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new())
    }
}

pub struct SelectGroup<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> SelectGroup<V> {
    pub fn new(_: impl Into<String>, child: V) -> Self {
        Self {
            id: ViewId::new(),
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.child.into_view())
    }
}
