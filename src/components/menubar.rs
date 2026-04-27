//! Menubar component with builder-style API
//! (tailwind-enhanced – complete file)

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

// Menubar
pub struct Menubar<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> Menubar<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for Menubar<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for Menubar<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.flex_row()
                        .items_center()
                        .p_1()
                        .background(t.background)
                        .border_bottom(1.0)
                        .border_color(t.border)
                })
            }),
        )
    }
}

// MenubarMenu
pub struct MenubarMenu<C> {
    id: ViewId,
    label: String,
    content: Option<C>,
}
impl MenubarMenu<()> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
            content: None,
        }
    }
}
impl<C> MenubarMenu<C> {
    pub fn content<C2: IntoView + 'static>(self, content: C2) -> MenubarMenu<C2> {
        MenubarMenu {
            id: self.id,
            label: self.label,
            content: Some(content),
        }
    }
}
impl<C: IntoView + 'static> HasViewId for MenubarMenu<C> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<C: IntoView + 'static> IntoView for MenubarMenu<C> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let label = self.label;
        let is_open = RwSignal::new(false);
        let trigger = floem::views::Label::new(label)
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let open = is_open.get();
                    let base = s
                        .px_3()
                        .py_1p5()
                        .text_sm()
                        .font_medium()
                        .color(t.foreground)
                        .rounded_md()
                        .cursor(CursorStyle::Pointer);
                    if open {
                        base.background(t.accent).color(t.accent_foreground)
                    } else {
                        base.hover(|s| s.background(t.accent).color(t.accent_foreground))
                    }
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                is_open.update(|v| *v = !*v);
            });
        let dropdown = if let Some(content) = self.content {
            floem::views::Container::new(content)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let open = is_open.get();
                        let base = s
                            .absolute()
                            .inset_top_pct(100.0)
                            .inset_left(0.0)
                            .mt_1()
                            .min_width(180.0)
                            .py_1()
                            .background(t.popover)
                            .border_1()
                            .border_color(t.border)
                            .rounded_md()
                            .box_shadow_blur(8.0)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                            .z_index(100)
                            .flex_col();
                        if open {
                            base
                        } else {
                            base.display(floem::style::Display::None)
                        }
                    })
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };
        let backdrop = floem::views::Empty::new()
            .style(move |s| {
                let open = is_open.get();
                if open {
                    s.absolute()
                        .inset(-1000.0)
                        .width(3000.0)
                        .height(3000.0)
                        .z_index(99)
                } else {
                    s.display(floem::style::Display::None)
                }
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                is_open.set(false);
            });
        Box::new(
            floem::views::Container::new(floem::views::Stack::new((trigger, backdrop, dropdown)))
                .style(|s| s.relative()),
        )
    }
}

// MenubarTrigger
pub struct MenubarTrigger {
    id: ViewId,
    label: String,
}
impl MenubarTrigger {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
        }
    }
}
impl HasViewId for MenubarTrigger {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for MenubarTrigger {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Label::with_id(self.id, self.label).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.px_3()
                        .py_1p5()
                        .text_sm()
                        .font_medium()
                        .color(t.foreground)
                        .rounded_md()
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.accent).color(t.accent_foreground))
                })
            }),
        )
    }
}

// MenubarContent
pub struct MenubarContent<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> MenubarContent<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for MenubarContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for MenubarContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| s.flex_col()))
    }
}

// MenubarItem
pub struct MenubarItem {
    id: ViewId,
    label: String,
    disabled: bool,
    on_select: Option<Box<dyn Fn() + 'static>>,
}
impl MenubarItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
            disabled: false,
            on_select: None,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_select(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}
impl HasViewId for MenubarItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for MenubarItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let label = self.label;
        let disabled = self.disabled;
        let on_select = self.on_select;
        let label_view = floem::views::Label::new(label).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.text_sm().flex_grow(1.0);
                if disabled {
                    base.color(t.muted_foreground)
                } else {
                    base.color(t.foreground)
                }
            })
        });
        let row = floem::views::Stack::horizontal((label_view,)).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.w_full().px_3().py_1p5().cursor(if disabled {
                    CursorStyle::Default
                } else {
                    CursorStyle::Pointer
                });
                if disabled {
                    base
                } else {
                    base.hover(|s| s.background(t.accent))
                }
            })
        });
        if let Some(handler) = on_select {
            if !disabled {
                Box::new(row.on_event_stop(floem::event::listener::Click, move |_, _| handler()))
            } else {
                Box::new(row)
            }
        } else {
            Box::new(row)
        }
    }
}

// MenubarCheckboxItem
pub struct MenubarCheckboxItem {
    id: ViewId,
    label: String,
    checked: RwSignal<bool>,
    disabled: bool,
}
impl MenubarCheckboxItem {
    pub fn new(label: impl Into<String>, checked: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
            checked,
            disabled: false,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
impl HasViewId for MenubarCheckboxItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for MenubarCheckboxItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let label = self.label;
        let checked = self.checked;
        let disabled = self.disabled;
        let check_indicator =
            floem::views::Label::derived(move || if checked.get() { "✓" } else { " " }.to_string())
                .style(|s| s.w_4().text_xs().color(peniko::Color::BLACK));
        let label_view = floem::views::Label::new(label).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.text_sm().flex_grow(1.0);
                if disabled {
                    base.color(t.muted_foreground)
                } else {
                    base.color(t.foreground)
                }
            })
        });
        let row = floem::views::Stack::horizontal((check_indicator, label_view)).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.w_full().px_3().py_1p5().gap_2().cursor(if disabled {
                    CursorStyle::Default
                } else {
                    CursorStyle::Pointer
                });
                if disabled {
                    base
                } else {
                    base.hover(|s| s.background(t.accent))
                }
            })
        });
        if disabled {
            Box::new(row)
        } else {
            Box::new(
                row.on_event_stop(floem::event::listener::Click, move |_, _| {
                    checked.update(|v| *v = !*v);
                }),
            )
        }
    }
}

// MenubarSeparator
pub struct MenubarSeparator;
impl MenubarSeparator {
    pub fn new() -> Self {
        Self
    }
}
impl Default for MenubarSeparator {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for MenubarSeparator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}
impl IntoView for MenubarSeparator {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.with_shadcn_theme(move |s, t| s.w_full().h_px().background(t.border).my_1())
        }))
    }
}

// MenubarShortcut
pub struct MenubarShortcut {
    id: ViewId,
    keys: String,
}
impl MenubarShortcut {
    pub fn new(keys: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            keys: keys.into(),
        }
    }
}
impl HasViewId for MenubarShortcut {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for MenubarShortcut {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Label::with_id(self.id, self.keys)
                .style(|s| s.text_xs().color(peniko::Color::BLACK)),
        )
    }
}
