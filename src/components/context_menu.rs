//! Context Menu component with builder-style API
//!
//! Based on shadcn/ui Context Menu - a menu triggered by right-click.
//!
//! # Example
//!
//! ```
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::context_menu::*;
//!
//! let menu_open = RwSignal::new(false);
//!
//! ContextMenu::new(menu_open)
//!     .trigger(|| {
//!         Container::new(label(|| "Right click here"))
//!             .style(|s| s.padding(40.0).border(1.0).border_dashed())
//!     })
//!     .content((
//!         ContextMenuItem::new("Cut"),
//!         ContextMenuItem::new("Copy"),
//!         ContextMenuItem::new("Paste"),
//!         ContextMenuSeparator::new(),
//!         ContextMenuItem::new("Delete").destructive(),
//!     ));
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

pub struct ContextMenu<T, C> {
    id: ViewId,
    open: RwSignal<bool>,
    trigger: Option<T>,
    content: Option<C>,
}
impl ContextMenu<(), ()> {
    pub fn new(open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            open,
            trigger: None,
            content: None,
        }
    }
}
impl<T, C> ContextMenu<T, C> {
    pub fn trigger<T2: Fn() -> V, V: IntoView + 'static>(self, trigger: T2) -> ContextMenu<T2, C> {
        ContextMenu {
            id: self.id,
            open: self.open,
            trigger: Some(trigger),
            content: self.content,
        }
    }
    pub fn content<C2: IntoView + 'static>(self, content: C2) -> ContextMenu<T, C2> {
        ContextMenu {
            id: self.id,
            open: self.open,
            trigger: self.trigger,
            content: Some(content),
        }
    }
}
impl<T, C, TV> ContextMenu<T, C>
where
    T: Fn() -> TV + 'static,
    C: IntoView + 'static,
    TV: IntoView + 'static,
{
    pub fn build(self) -> impl IntoView {
        let open = self.open;
        let trigger = self.trigger;
        let content = self.content;
        let trigger_view = if let Some(trigger_fn) = trigger {
            floem::views::Container::new(trigger_fn())
                .on_event_stop(floem::event::listener::SecondaryClick, move |_, _| {
                    open.set(true);
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };
        let content_view = if let Some(menu_content) = content {
            floem::views::Container::new(menu_content)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let is_open = open.get();
                        let base = s
                            .min_width(160.0)
                            .py_1()
                            .background(t.popover)
                            .border_1()
                            .border_color(t.border)
                            .rounded_md()
                            .box_shadow_blur(8.0)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                            .absolute()
                            .inset_top_pct(100.0)
                            .inset_left(0.0)
                            .mt_1()
                            .z_index(100)
                            .flex_col();
                        if is_open {
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
                let is_open = open.get();
                let base = s.absolute().inset_0().z_index(99);
                if is_open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                open.set(false);
            });
        floem::views::Container::new(floem::views::Stack::new((
            trigger_view,
            backdrop,
            content_view,
        )))
        .style(|s| s.relative())
    }
}
impl<T, C, TV> HasViewId for ContextMenu<T, C>
where
    T: Fn() -> TV + 'static,
    C: IntoView + 'static,
    TV: IntoView + 'static,
{
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<T, C, TV> IntoView for ContextMenu<T, C>
where
    T: Fn() -> TV + 'static,
    C: IntoView + 'static,
    TV: IntoView + 'static,
{
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

pub struct ContextMenuContent<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> ContextMenuContent<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for ContextMenuContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for ContextMenuContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| s.flex_col()))
    }
}

pub struct ContextMenuItem {
    id: ViewId,
    text: String,
    disabled: bool,
    destructive: bool,
    shortcut: Option<String>,
    on_click: Option<Box<dyn Fn() + 'static>>,
}
impl ContextMenuItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            disabled: false,
            destructive: false,
            shortcut: None,
            on_click: None,
        }
    }
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
}
impl HasViewId for ContextMenuItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ContextMenuItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        let disabled = self.disabled;
        let destructive = self.destructive;
        let shortcut = self.shortcut;
        let on_click = self.on_click;
        let label = floem::views::Label::new(text.clone()).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.text_sm().flex_grow(1.0);
                if destructive {
                    base.color(t.destructive)
                } else if disabled {
                    base.color(t.muted_foreground)
                } else {
                    base.color(t.foreground)
                }
            })
        });
        let shortcut_view = if let Some(sc) = shortcut {
            floem::views::Label::new(sc)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| s.text_xs().color(t.muted_foreground).ml_4())
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };
        let row = floem::views::Stack::horizontal((label, shortcut_view)).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.w_full().px_3().py_2().cursor(if disabled {
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
        if let Some(handler) = on_click {
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

pub struct ContextMenuSeparator;
impl ContextMenuSeparator {
    pub fn new() -> Self {
        Self
    }
}
impl Default for ContextMenuSeparator {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ContextMenuSeparator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}
impl IntoView for ContextMenuSeparator {
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

pub struct ContextMenuLabel {
    id: ViewId,
    text: String,
}
impl ContextMenuLabel {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for ContextMenuLabel {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ContextMenuLabel {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.w_full()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .text_xs()
                    .font_medium()
                    .color(t.foreground)
            })
        }))
    }
}

pub struct ContextMenuGroup<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> ContextMenuGroup<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for ContextMenuGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for ContextMenuGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| s.flex_col()))
    }
}
