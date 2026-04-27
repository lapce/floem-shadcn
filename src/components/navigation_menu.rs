//! Navigation Menu component with builder-style API
//! (tailwind-enhanced – complete file)

use crate::components::dropdown_menu::DropdownMenu;
use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

// NavigationMenu
pub struct NavigationMenu<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> NavigationMenu<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for NavigationMenu<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for NavigationMenu<V> {
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
                        .gap_1()
                        .p_1()
                        .background(t.background)
                })
            }),
        )
    }
}

// NavigationMenuList
pub struct NavigationMenuList<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> NavigationMenuList<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for NavigationMenuList<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for NavigationMenuList<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.flex_row().items_center().gap_1()),
        )
    }
}

// NavigationMenuItem - reuses DropdownMenu for consistent dropdown behavior
#[allow(dead_code)]
pub struct NavigationMenuItem {
    id: ViewId,
    label: String,
}

impl NavigationMenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
        }
    }
}

impl HasViewId for NavigationMenuItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for NavigationMenuItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let label = self.label;

        DropdownMenu::new(RwSignal::new(false))
            .trigger(move || {
                floem::views::Label::new(label.clone()).style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.px_3()
                            .py_2()
                            .text_sm()
                            .font_medium()
                            .color(t.foreground)
                            .rounded_md()
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.background(t.accent).color(t.accent_foreground))
                    })
                })
            })
            .content(floem::views::Empty::new())
            .into_view()
    }
}

// NavigationMenuTrigger
pub struct NavigationMenuTrigger {
    id: ViewId,
    label: String,
}
impl NavigationMenuTrigger {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
        }
    }
}
impl HasViewId for NavigationMenuTrigger {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for NavigationMenuTrigger {
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
                        .py_2()
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

// NavigationMenuContent
pub struct NavigationMenuContent<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> NavigationMenuContent<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for NavigationMenuContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for NavigationMenuContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| s.flex_col().gap_1()),
        )
    }
}

// NavigationMenuLink
pub struct NavigationMenuLink {
    id: ViewId,
    label: String,
    description: Option<String>,
    on_click: Option<Box<dyn Fn() + 'static>>,
}
impl NavigationMenuLink {
    pub fn new(label: impl Into<String>, _href: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
            description: None,
            on_click: None,
        }
    }
    pub fn simple(label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
            description: None,
            on_click: None,
        }
    }
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}
impl HasViewId for NavigationMenuLink {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for NavigationMenuLink {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let label = self.label;
        let description = self.description;
        let on_click = self.on_click;
        let title = floem::views::Label::new(label).style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_sm().font_medium().color(t.foreground))
        });
        let desc_view = if let Some(desc) = description {
            floem::views::Label::new(desc)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.text_xs().color(t.muted_foreground).margin_top(2.0)
                    })
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };
        let container = floem::views::Stack::vertical((title, desc_view)).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.p_2()
                    .rounded_md()
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.background(t.accent))
            })
        });
        if let Some(handler) = on_click {
            Box::new(container.on_event_stop(floem::event::listener::Click, move |_, _| handler()))
        } else {
            Box::new(container)
        }
    }
}

// NavigationMenuIndicator
pub struct NavigationMenuIndicator;
impl NavigationMenuIndicator {
    pub fn new() -> Self {
        Self
    }
}
impl Default for NavigationMenuIndicator {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for NavigationMenuIndicator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}
impl IntoView for NavigationMenuIndicator {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new())
    }
}

// NavigationMenuViewport
pub struct NavigationMenuViewport<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> NavigationMenuViewport<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for NavigationMenuViewport<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for NavigationMenuViewport<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.absolute()
                    .inset_top_pct(100.0)
                    .inset_left(0.0)
                    .background(peniko::Color::WHITE)
                    .border_1()
                    .border_color(peniko::Color::BLACK)
                    .rounded_md()
                    .box_shadow_blur(8.0)
                    .box_shadow_color(peniko::Color::BLACK.with_alpha(0.1))
            }),
        )
    }
}
