//! Navigation Menu component with builder-style API
//!
//! Based on shadcn/ui Navigation Menu - a collection of links for site navigation.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::navigation_menu::*;
//!
//! NavigationMenu::new((
//!     NavigationMenuItem::new("Getting Started")
//!         .content((
//!             NavigationMenuLink::new("Introduction", "intro"),
//!             NavigationMenuLink::new("Installation", "install"),
//!         )),
//!     NavigationMenuItem::new("Components")
//!         .content((
//!             NavigationMenuLink::new("Button", "button"),
//!             NavigationMenuLink::new("Card", "card"),
//!         )),
//!     NavigationMenuLink::simple("Documentation"),
//! ));
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

// ============================================================================
// NavigationMenu
// ============================================================================

/// Navigation menu container
pub struct NavigationMenu<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> NavigationMenu<V> {
    /// Create a new navigation menu
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Row)
                        .items_center()
                        .gap(4.0)
                        .padding(4.0)
                        .background(t.background)
                })
            }),
        )
    }
}

// ============================================================================
// NavigationMenuList
// ============================================================================

/// List of navigation items
pub struct NavigationMenuList<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> NavigationMenuList<V> {
    /// Create a navigation menu list
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
            }),
        )
    }
}

// ============================================================================
// NavigationMenuItem
// ============================================================================

/// Navigation menu item with dropdown
pub struct NavigationMenuItem<C> {
    id: ViewId,
    label: String,
    content: Option<C>,
}

impl NavigationMenuItem<()> {
    /// Create a new menu item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
            content: None,
        }
    }
}

impl<C> NavigationMenuItem<C> {
    /// Set dropdown content
    pub fn content<C2: IntoView + 'static>(self, content: C2) -> NavigationMenuItem<C2> {
        NavigationMenuItem {
            id: self.id,
            label: self.label,
            content: Some(content),
        }
    }
}

impl<C: IntoView + 'static> HasViewId for NavigationMenuItem<C> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<C: IntoView + 'static> IntoView for NavigationMenuItem<C> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let label = self.label;
        let is_open = RwSignal::new(false);

        // Trigger button
        let trigger = floem::views::Label::new(format!("{} ▾", label))
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.padding_left(12.0)
                        .padding_right(12.0)
                        .padding_top(8.0)
                        .padding_bottom(8.0)
                        .font_size(14.0)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .color(t.foreground)
                        .border_radius(t.radius)
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.accent).color(t.accent_foreground))
                })
            })
            .on_event_stop(floem::event::EventListener::PointerEnter, move |_| {
                is_open.set(true);
            });

        // Dropdown content
        let dropdown = if let Some(content) = self.content {
            floem::views::Container::new(content)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let open = is_open.get();
                        let base = s
                            .position(floem::style::Position::Absolute)
                            .inset_top_pct(100.0)
                            .inset_left(0.0)
                            .margin_top(4.0)
                            .min_width(200.0)
                            .padding(12.0)
                            .background(t.popover)
                            .border(1.0)
                            .border_color(t.border)
                            .border_radius(t.radius)
                            .box_shadow_blur(8.0)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                            .z_index(50)
                            .display(floem::style::Display::Flex)
                            .flex_direction(floem::style::FlexDirection::Column)
                            .gap(4.0);
                        if open {
                            base
                        } else {
                            base.display(floem::style::Display::None)
                        }
                    })
                })
                .on_event_stop(floem::event::EventListener::PointerEnter, move |_| {
                    is_open.set(true);
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        Box::new(
            floem::views::Container::new(floem::views::Stack::new((trigger, dropdown)))
                .style(|s| s.position(floem::style::Position::Relative))
                .on_event_stop(floem::event::EventListener::PointerLeave, move |_| {
                    is_open.set(false);
                }),
        )
    }
}

// ============================================================================
// NavigationMenuTrigger
// ============================================================================

/// Trigger for navigation menu item
pub struct NavigationMenuTrigger {
    id: ViewId,
    label: String,
}

impl NavigationMenuTrigger {
    /// Create a new trigger
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let label = self.label;

        Box::new(
            floem::views::Label::with_id(self.id, format!("{} ▾", label)).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.padding_left(12.0)
                        .padding_right(12.0)
                        .padding_top(8.0)
                        .padding_bottom(8.0)
                        .font_size(14.0)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .color(t.foreground)
                        .border_radius(t.radius)
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.accent).color(t.accent_foreground))
                })
            }),
        )
    }
}

// ============================================================================
// NavigationMenuContent
// ============================================================================

/// Content container for navigation dropdown
pub struct NavigationMenuContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> NavigationMenuContent<V> {
    /// Create new navigation content
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Column)
                    .gap(4.0)
            }),
        )
    }
}

// ============================================================================
// NavigationMenuLink
// ============================================================================

/// Navigation link item
pub struct NavigationMenuLink {
    id: ViewId,
    label: String,
    href: String,
    description: Option<String>,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl NavigationMenuLink {
    /// Create a new navigation link
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
            href: href.into(),
            description: None,
            on_click: None,
        }
    }

    /// Create a simple link without href
    pub fn simple(label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
            href: String::new(),
            description: None,
            on_click: None,
        }
    }

    /// Add description text
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set click handler
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let label = self.label;
        let description = self.description;
        let on_click = self.on_click;

        // Title
        let title = floem::views::Label::new(label).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(14.0)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .color(t.foreground)
            })
        });

        // Description (if any)
        let desc_view = if let Some(desc) = description {
            floem::views::Label::new(desc)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.font_size(12.0).color(t.muted_foreground).margin_top(2.0)
                    })
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        let container = floem::views::Stack::vertical((title, desc_view)).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.padding(8.0)
                    .border_radius(t.radius)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.background(t.accent))
            })
        });

        if let Some(handler) = on_click {
            Box::new(container.on_click_stop(move |_| handler()))
        } else {
            Box::new(container)
        }
    }
}

// ============================================================================
// NavigationMenuIndicator
// ============================================================================

/// Visual indicator for active menu item
pub struct NavigationMenuIndicator;

impl NavigationMenuIndicator {
    /// Create a new indicator
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.height(2.0)
                    .background(t.primary)
                    .position(floem::style::Position::Absolute)
                    .inset_bottom(0.0)
            })
        }))
    }
}

// ============================================================================
// NavigationMenuViewport
// ============================================================================

/// Viewport for navigation content (for animations)
pub struct NavigationMenuViewport<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> NavigationMenuViewport<V> {
    /// Create a new viewport
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::new(self.child).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.position(floem::style::Position::Absolute)
                    .inset_top_pct(100.0)
                    .inset_left(0.0)
                    .background(t.popover)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius)
                    .box_shadow_blur(8.0)
                    .box_shadow_color(t.foreground.with_alpha(0.1))
            })
        }))
    }
}
