//! Breadcrumb component with builder-style API
//!
//! Based on shadcn/ui Breadcrumb - navigation trail showing path hierarchy.
//!
//! # Example
//!
//! ```
//! use floem_shadcn::components::breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbSeparator};
//!
//! Breadcrumb::new((
//!     BreadcrumbItem::new("Home").href("/"),
//!     BreadcrumbSeparator::new(),
//!     BreadcrumbItem::new("Products").href("/products"),
//!     BreadcrumbSeparator::new(),
//!     BreadcrumbItem::new("Widget").current(),
//! ));
//! ```

use floem::prelude::*;
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Breadcrumb
// ============================================================================

pub struct Breadcrumb<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> Breadcrumb<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for Breadcrumb<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Breadcrumb<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.flex_row()
                    .items_center()
                    .gap_1()
                    .flex_wrap(floem::style::FlexWrap::Wrap)
            }),
        )
    }
}

// ============================================================================
// BreadcrumbList
// ============================================================================

pub struct BreadcrumbList<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> BreadcrumbList<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for BreadcrumbList<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for BreadcrumbList<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.flex_row()
                    .items_center()
                    .gap_1()
                    .flex_wrap(floem::style::FlexWrap::Wrap)
            }),
        )
    }
}

// ============================================================================
// BreadcrumbItem
// ============================================================================

pub struct BreadcrumbItem {
    id: ViewId,
    text: String,
    href: Option<String>,
    is_current: bool,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl BreadcrumbItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            href: None,
            is_current: false,
            on_click: None,
        }
    }

    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
    pub fn current(mut self) -> Self {
        self.is_current = true;
        self
    }
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl HasViewId for BreadcrumbItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for BreadcrumbItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        let is_current = self.is_current;
        let has_href = self.href.is_some();
        let on_click = self.on_click;

        let label = floem::views::Label::new(text).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.text_sm();
                if is_current {
                    base.color(t.foreground).font_medium()
                } else if has_href {
                    base.color(t.muted_foreground)
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.color(t.foreground))
                } else {
                    base.color(t.muted_foreground)
                }
            })
        });

        if let Some(handler) = on_click {
            Box::new(label.on_event_stop(floem::event::listener::Click, move |_, _| handler()))
        } else {
            Box::new(label)
        }
    }
}

// ============================================================================
// BreadcrumbLink
// ============================================================================

pub struct BreadcrumbLink<V> {
    id: ViewId,
    child: V,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl<V: IntoView + 'static> BreadcrumbLink<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            on_click: None,
        }
    }
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl<V: IntoView + 'static> HasViewId for BreadcrumbLink<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for BreadcrumbLink<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let on_click = self.on_click;
        let container = floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.color(t.muted_foreground)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.color(t.foreground))
            })
        });

        if let Some(handler) = on_click {
            Box::new(container.on_event_stop(floem::event::listener::Click, move |_, _| handler()))
        } else {
            Box::new(container)
        }
    }
}

// ============================================================================
// BreadcrumbPage
// ============================================================================

pub struct BreadcrumbPage {
    id: ViewId,
    text: String,
}

impl BreadcrumbPage {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for BreadcrumbPage {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for BreadcrumbPage {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_sm().color(t.foreground).font_medium())
        }))
    }
}

// ============================================================================
// BreadcrumbSeparator
// ============================================================================

pub struct BreadcrumbSeparator {
    id: ViewId,
    separator: String,
}

impl BreadcrumbSeparator {
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            separator: "/".to_string(),
        }
    }
    pub fn chevron() -> Self {
        Self {
            id: ViewId::new(),
            separator: ">".to_string(),
        }
    }
    pub fn custom(separator: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            separator: separator.into(),
        }
    }
}

impl Default for BreadcrumbSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for BreadcrumbSeparator {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for BreadcrumbSeparator {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let separator = self.separator;
        Box::new(floem::views::Label::with_id(self.id, separator).style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_sm().color(t.muted_foreground).px_1())
        }))
    }
}

// ============================================================================
// BreadcrumbEllipsis
// ============================================================================

pub struct BreadcrumbEllipsis {
    id: ViewId,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl BreadcrumbEllipsis {
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            on_click: None,
        }
    }
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Default for BreadcrumbEllipsis {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for BreadcrumbEllipsis {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for BreadcrumbEllipsis {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let on_click = self.on_click;
        let has_click = on_click.is_some();
        let label = floem::views::Label::new("...").style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.text_sm().color(t.muted_foreground).px_1();
                if has_click {
                    base.cursor(CursorStyle::Pointer)
                        .hover(|s| s.color(t.foreground))
                } else {
                    base
                }
            })
        });

        Box::new(label)
    }
}
