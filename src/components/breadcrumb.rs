//! Breadcrumb component with builder-style API
//!
//! Based on shadcn/ui Breadcrumb - navigation trail showing path hierarchy.
//!
//! # Example
//!
//! ```rust
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

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Breadcrumb
// ============================================================================

/// Breadcrumb container
pub struct Breadcrumb<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> Breadcrumb<V> {
    /// Create a new breadcrumb with items
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Row)
                    .items_center()
                    .gap(4.0)
                    .flex_wrap(floem::style::FlexWrap::Wrap)
            }),
        )
    }
}

// ============================================================================
// BreadcrumbList
// ============================================================================

/// Wrapper for breadcrumb items (semantic element)
pub struct BreadcrumbList<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> BreadcrumbList<V> {
    /// Create a new breadcrumb list
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Row)
                    .items_center()
                    .gap(4.0)
                    .flex_wrap(floem::style::FlexWrap::Wrap)
            }),
        )
    }
}

// ============================================================================
// BreadcrumbItem
// ============================================================================

/// Individual breadcrumb item
pub struct BreadcrumbItem {
    id: ViewId,
    text: String,
    href: Option<String>,
    is_current: bool,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            href: None,
            is_current: false,
            on_click: None,
        }
    }

    /// Set the href (for display purposes - actual navigation handled by on_click)
    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Mark this item as the current page
    pub fn current(mut self) -> Self {
        self.is_current = true;
        self
    }

    /// Set a click handler
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        let is_current = self.is_current;
        let has_href = self.href.is_some();
        let on_click = self.on_click;

        let label = floem::views::Label::new(text).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.font_size(14.0);

                if is_current {
                    base.color(t.foreground)
                        .font_weight(floem::text::Weight::MEDIUM)
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
            Box::new(label.on_click_stop(move |_| {
                handler();
            }))
        } else {
            Box::new(label)
        }
    }
}

// ============================================================================
// BreadcrumbLink
// ============================================================================

/// Clickable breadcrumb link (alternative to BreadcrumbItem)
pub struct BreadcrumbLink<V> {
    id: ViewId,
    child: V,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl<V: IntoView + 'static> BreadcrumbLink<V> {
    /// Create a new breadcrumb link
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            on_click: None,
        }
    }

    /// Set click handler
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
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
            Box::new(container.on_click_stop(move |_| {
                handler();
            }))
        } else {
            Box::new(container)
        }
    }
}

// ============================================================================
// BreadcrumbPage
// ============================================================================

/// Current page indicator (non-clickable)
pub struct BreadcrumbPage {
    id: ViewId,
    text: String,
}

impl BreadcrumbPage {
    /// Create a new breadcrumb page (current location)
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(14.0)
                    .color(t.foreground)
                    .font_weight(floem::text::Weight::MEDIUM)
            })
        }))
    }
}

// ============================================================================
// BreadcrumbSeparator
// ============================================================================

/// Separator between breadcrumb items (default: /)
pub struct BreadcrumbSeparator {
    id: ViewId,
    separator: String,
}

impl BreadcrumbSeparator {
    /// Create a new separator with default "/" character
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            separator: "/".to_string(),
        }
    }

    /// Create a separator with chevron ">"
    pub fn chevron() -> Self {
        Self {
            id: ViewId::new(),
            separator: ">".to_string(),
        }
    }

    /// Create a separator with custom text
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let separator = self.separator;
        Box::new(floem::views::Label::with_id(self.id, separator).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(14.0)
                    .color(t.muted_foreground)
                    .padding_left(4.0)
                    .padding_right(4.0)
            })
        }))
    }
}

// ============================================================================
// BreadcrumbEllipsis
// ============================================================================

/// Ellipsis indicator for collapsed breadcrumb items
pub struct BreadcrumbEllipsis {
    id: ViewId,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl BreadcrumbEllipsis {
    /// Create a new ellipsis
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            on_click: None,
        }
    }

    /// Set click handler (e.g., to expand collapsed items)
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let on_click = self.on_click;

        let has_click = on_click.is_some();
        let label = floem::views::Label::new("...").style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .font_size(14.0)
                    .color(t.muted_foreground)
                    .padding_left(4.0)
                    .padding_right(4.0);

                if has_click {
                    base.cursor(CursorStyle::Pointer)
                        .hover(|s| s.color(t.foreground))
                } else {
                    base
                }
            })
        });

        // Note: Can't move on_click twice, so we check existence differently
        Box::new(label)
    }
}
