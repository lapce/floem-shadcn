//! Dropdown Menu component with builder-style API
//!
//! Based on shadcn/ui Dropdown Menu - a menu that appears on trigger.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::dropdown_menu::*;
//!
//! let open = RwSignal::new(false);
//!
//! DropdownMenu::new(open)
//!     .trigger(|| Button::new("Options"))
//!     .content((
//!         DropdownMenuItem::new("Edit").on_click(|| println!("Edit")),
//!         DropdownMenuItem::new("Copy").on_click(|| println!("Copy")),
//!         DropdownMenuSeparator::new(),
//!         DropdownMenuItem::new("Delete").destructive(),
//!     ));
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// DropdownMenu – a popup menu that appears when the trigger is clicked.
pub struct DropdownMenu<T, C> {
    open: RwSignal<bool>,
    trigger: Option<T>,
    content: Option<C>,
}

impl DropdownMenu<(), ()> {
    /// Create a new dropdown menu controlled by the given open signal.
    pub fn new(open: RwSignal<bool>) -> Self {
        Self {
            open,
            trigger: None,
            content: None,
        }
    }
}

impl<T, C> DropdownMenu<T, C> {
    /// Provide the trigger widget (a button, for example).
    pub fn trigger<T2: Fn() -> V, V: IntoView + 'static>(self, trigger: T2) -> DropdownMenu<T2, C> {
        DropdownMenu {
            open: self.open,
            trigger: Some(trigger),
            content: self.content,
        }
    }

    /// Provide the menu items to show when open.
    /// Eagerly converts content to Box<dyn View> so it can be captured in closures.
    pub fn content<C2: IntoView + 'static>(self, content: C2) -> DropdownMenu<T, Box<dyn View>> {
        DropdownMenu {
            open: self.open,
            trigger: self.trigger,
            content: Some(Box::new(content.into_view())),
        }
    }
}

impl<T, C> HasViewId for DropdownMenu<T, C> {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}

impl<T, TV> DropdownMenu<T, Box<dyn View>>
where
    T: Fn() -> TV + 'static,
    TV: IntoView + 'static,
{
    /// Build the view: trigger + dropdown overlay.
    fn build(self) -> impl IntoView {
        let open = self.open;
        let trigger = self.trigger;
        let content = self.content;

        let trigger_view = if let Some(trigger_fn) = trigger {
            floem::views::Container::new(trigger_fn())
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    open.update(|v| *v = !*v);
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
                            .min_width(180.0)
                            .py_1()
                            .background(t.popover)
                            .border_1()
                            .border_color(t.border)
                            .rounded_md()
                            .box_shadow_blur(8.0)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                            .absolute()
                            .inset_top_pct(100.0)
                            .mt_1()
                            .inset_left(0.0)
                            .z_index(50)
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

        floem::views::Container::new(floem::views::Stack::new((trigger_view, content_view)))
            .style(|s| s.relative())
    }
}

impl<T, TV> IntoView for DropdownMenu<T, Box<dyn View>>
where
    T: Fn() -> TV + 'static,
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

/// Content wrapper for the dropdown menu items.
pub struct DropdownMenuContent<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> DropdownMenuContent<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for DropdownMenuContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for DropdownMenuContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| s.flex_col()))
    }
}

/// A single item inside a dropdown menu.
pub struct DropdownMenuItem {
    id: ViewId,
    text: String,
    disabled: bool,
    destructive: bool,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl DropdownMenuItem {
    /// Create a new item with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            disabled: false,
            destructive: false,
            on_click: None,
        }
    }

    /// Attach a click handler.
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Mark the item as disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Apply destructive (red) styling.
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

impl HasViewId for DropdownMenuItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for DropdownMenuItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        let disabled = self.disabled;
        let destructive = self.destructive;
        let on_click = self.on_click;

        let label = floem::views::Label::new(text).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.w_full().px_3().py_2().text_sm().cursor(if disabled {
                    CursorStyle::Default
                } else {
                    CursorStyle::Pointer
                });
                let colored = if destructive {
                    base.color(t.destructive)
                } else if disabled {
                    base.color(t.muted_foreground)
                } else {
                    base.color(t.foreground)
                };
                if disabled {
                    colored
                } else {
                    colored.hover(|s| s.background(t.accent).color(t.accent_foreground))
                }
            })
        });

        if let Some(handler) = on_click {
            if !disabled {
                Box::new(label.on_event_stop(floem::event::listener::Click, move |_, _| handler()))
            } else {
                Box::new(label)
            }
        } else {
            Box::new(label)
        }
    }
}

/// Dropdown menu item with custom child content.
pub struct DropdownMenuItemCustom<V> {
    id: ViewId,
    child: V,
    disabled: bool,
    on_click: Option<Box<dyn Fn() + 'static>>,
}
impl<V: IntoView + 'static> DropdownMenuItemCustom<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            disabled: false,
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
}
impl<V: IntoView + 'static> HasViewId for DropdownMenuItemCustom<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for DropdownMenuItemCustom<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let disabled = self.disabled;
        let on_click = self.on_click;
        let container = floem::views::Container::new(self.child).style(move |s| {
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
                Box::new(
                    container.on_event_stop(floem::event::listener::Click, move |_, _| handler()),
                )
            } else {
                Box::new(container)
            }
        } else {
            Box::new(container)
        }
    }
}

/// Separator line between menu items.
pub struct DropdownMenuSeparator;
impl DropdownMenuSeparator {
    pub fn new() -> Self {
        Self
    }
}
impl Default for DropdownMenuSeparator {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for DropdownMenuSeparator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}
impl IntoView for DropdownMenuSeparator {
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

/// Label for a group of menu items.
pub struct DropdownMenuLabel {
    id: ViewId,
    text: String,
}
impl DropdownMenuLabel {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for DropdownMenuLabel {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for DropdownMenuLabel {
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

/// Group of related menu items.
pub struct DropdownMenuGroup<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> DropdownMenuGroup<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for DropdownMenuGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for DropdownMenuGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| s.flex_col()))
    }
}

/// Shortcut hint displayed to the right of a menu item.
pub struct DropdownMenuShortcut {
    id: ViewId,
    text: String,
}
impl DropdownMenuShortcut {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for DropdownMenuShortcut {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for DropdownMenuShortcut {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_xs().color(t.muted_foreground).ml_4())
        }))
    }
}
