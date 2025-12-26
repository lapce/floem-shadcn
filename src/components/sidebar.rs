//! Sidebar component with builder-style API
//!
//! Based on shadcn/ui Sidebar component for navigation.
//! Active state is controlled via the `is_active` prop on `SidebarMenuButton`,
//! following the same pattern as shadcn/ui.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::sidebar::*;
//!
//! let active = RwSignal::new("buttons");
//!
//! let sidebar = Sidebar::new()
//!     .header(SidebarHeader::new(label(|| "My App")))
//!     .content(SidebarContent::new(
//!         SidebarGroup::new((
//!             SidebarGroupLabel::new("Components"),
//!             SidebarGroupContent::new(
//!                 SidebarMenu::new((
//!                     SidebarMenuItem::new(
//!                         SidebarMenuButton::new("Buttons")
//!                             .is_active(move || active.get() == "buttons")
//!                             .on_click_stop(move |_| active.set("buttons"))
//!                     ),
//!                     SidebarMenuItem::new(
//!                         SidebarMenuButton::new("Cards")
//!                             .is_active(move || active.get() == "cards")
//!                             .on_click_stop(move |_| active.set("cards"))
//!                     ),
//!                 )),
//!             ),
//!         )),
//!     ))
//!     .footer(SidebarFooter::new(label(|| "v1.0.0")));
//! ```

use std::rc::Rc;

use floem::style::CursorStyle;
use floem::text::Weight;
use floem::{AnyView, prelude::*};
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Sidebar - Main container
// ============================================================================

/// Sidebar container builder with ergonomic API.
///
/// # Example
/// ```rust
/// Sidebar::new()
///     .header(SidebarHeader::new(label(|| "My App")))
///     .content(SidebarContent::new(/* menu items */))
///     .footer(SidebarFooter::new(label(|| "v1.0")))
/// ```
pub struct Sidebar {
    id: ViewId,
    header: Option<Box<dyn View>>,
    content: Option<Box<dyn View>>,
    footer: Option<Box<dyn View>>,
    width: f32,
}

impl Sidebar {
    /// Create a new empty sidebar builder
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            header: None,
            content: None,
            footer: None,
            width: 240.0,
        }
    }

    /// Set the sidebar header
    pub fn header(mut self, header: impl IntoView + 'static) -> Self {
        self.header = Some(Box::new(header.into_view()));
        self
    }

    /// Set the sidebar content (scrollable area)
    pub fn content(mut self, content: impl IntoView + 'static) -> Self {
        self.content = Some(Box::new(content.into_view()));
        self
    }

    /// Set the sidebar footer
    pub fn footer(mut self, footer: impl IntoView + 'static) -> Self {
        self.footer = Some(Box::new(footer.into_view()));
        self
    }

    /// Set the sidebar width (default: 240px)
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for Sidebar {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Sidebar {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let width = self.width;
        let header = self.header;
        let content = self.content;
        let footer = self.footer;

        // Build view with optional header, content, footer
        let header_view: Box<dyn View> = header.unwrap_or_else(|| ().into_any());
        let content_view: Box<dyn View> = content.unwrap_or_else(|| ().into_any());
        let footer_view: Box<dyn View> = footer.unwrap_or_else(|| ().into_any());

        Box::new(
            Stack::vertical((header_view, content_view, footer_view)).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width(width)
                        .height_full()
                        .border_right(1.0)
                        .border_color(t.border)
                        .background(t.background)
                })
            }),
        )
    }
}

// ============================================================================
// SidebarHeader - Header section at top
// ============================================================================

/// Sidebar header component (for logo/title at top)
pub struct SidebarHeader<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SidebarHeader<V> {
    /// Create a new sidebar header
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SidebarHeader<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SidebarHeader<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.width_full()
                        .padding(16.0)
                        .border_bottom(1.0)
                        .border_color(t.border)
                })
            }),
        )
    }
}

// ============================================================================
// SidebarContent - Main scrollable content area
// ============================================================================

/// Sidebar content area (scrollable).
/// The child should be a vertically laid out view (e.g., using v_stack).
pub struct SidebarContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SidebarContent<V> {
    /// Create a new sidebar content area
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SidebarContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SidebarContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Scroll::new(floem::views::Container::new(self.child).style(|s| {
                s.flex_direction(floem::style::FlexDirection::Column)
                    .padding(8.0)
                    .gap(8.0)
                    .width_full()
            }))
            .style(|s| {
                s.flex_grow(1.0)
                    .flex_basis(0.0)
                    .min_height(0.0) // Required for scroll in flex column
                    .width_full()
            }),
        )
    }
}

// ============================================================================
// SidebarFooter - Footer section at bottom
// ============================================================================

/// Sidebar footer component
pub struct SidebarFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SidebarFooter<V> {
    /// Create a new sidebar footer
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SidebarFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SidebarFooter<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.width_full()
                        .padding(16.0)
                        .border_top(1.0)
                        .border_color(t.border)
                })
            }),
        )
    }
}

// ============================================================================
// SidebarGroup - Groups related menu items
// ============================================================================

/// Sidebar group container.
/// The child should be a vertically laid out view.
pub struct SidebarGroup<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SidebarGroup<V> {
    /// Create a new sidebar group
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SidebarGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SidebarGroup<V> {
    type V = AnyView;
    type Intermediate = AnyView;

    fn into_intermediate(self) -> Self::Intermediate {
        Container::with_id(self.id, self.child)
            .style(|s| {
                s.flex_direction(floem::style::FlexDirection::Column)
                    .width_full()
                    .gap(4.0)
            })
            .into_any()
    }
}

// ============================================================================
// SidebarGroupLabel - Label for a group
// ============================================================================

/// Sidebar group label
pub struct SidebarGroupLabel {
    id: ViewId,
    text: String,
}

impl SidebarGroupLabel {
    /// Create a new group label
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for SidebarGroupLabel {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarGroupLabel {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text.to_uppercase();
        Box::new(
            floem::views::Label::derived(move || text.clone()).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.font_size(11.0)
                        .font_weight(Weight::SEMIBOLD)
                        .color(t.muted_foreground)
                        .padding_left(8.0)
                        .padding_right(8.0)
                        .padding_top(8.0)
                        .padding_bottom(4.0)
                })
            }),
        )
    }
}

// ============================================================================
// SidebarGroupContent - Content container within a group
// ============================================================================

/// Sidebar group content container
pub struct SidebarGroupContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SidebarGroupContent<V> {
    /// Create a new group content container
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SidebarGroupContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SidebarGroupContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.flex_direction(floem::style::FlexDirection::Column)
                    .width_full()
            }),
        )
    }
}

// ============================================================================
// SidebarGroupAction - Optional action button in group header
// ============================================================================

/// Sidebar group action button (appears next to group label)
pub struct SidebarGroupAction<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SidebarGroupAction<V> {
    /// Create a new group action
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SidebarGroupAction<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SidebarGroupAction<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.padding(4.0)
                        .border_radius(t.radius_sm)
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.accent))
                })
            }),
        )
    }
}

// ============================================================================
// SidebarMenu - Menu container
// ============================================================================

/// Sidebar menu container.
/// The child should be a vertically laid out view of SidebarMenuItem elements.
pub struct SidebarMenu<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SidebarMenu<V> {
    /// Create a new sidebar menu
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SidebarMenu<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SidebarMenu<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.flex_direction(floem::style::FlexDirection::Column)
                    .width_full()
                    .gap(2.0)
            }),
        )
    }
}

// ============================================================================
// SidebarMenuItem - Menu item wrapper
// ============================================================================

/// Sidebar menu item wrapper
pub struct SidebarMenuItem<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> SidebarMenuItem<V> {
    /// Create a new menu item
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for SidebarMenuItem<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for SidebarMenuItem<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| s.width_full()))
    }
}

// ============================================================================
// SidebarMenuButton - Clickable menu button
// ============================================================================

/// Sidebar menu button (the actual clickable item)
///
/// Active state is controlled via the `is_active` prop, following the same pattern
/// as shadcn/ui. This gives full control to the consumer.
///
/// # Example
/// ```rust
/// let active = RwSignal::new("buttons");
///
/// SidebarMenuButton::new("Buttons")
///     .is_active(move || active.get() == "buttons")
///     .on_click_stop(move |_| active.set("buttons"))
/// ```
pub struct SidebarMenuButton {
    id: ViewId,
    label: String,
    is_active: Option<Box<dyn Fn() -> bool>>,
}

impl SidebarMenuButton {
    /// Create a new menu button with a label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            label: label.into(),
            is_active: None,
        }
    }

    /// Set whether this button is active (reactive closure).
    pub fn is_active(mut self, active: impl Fn() -> bool + 'static) -> Self {
        self.is_active = Some(Box::new(active));
        self
    }
}

impl HasViewId for SidebarMenuButton {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarMenuButton {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let label = self.label.clone();
        let is_active: Rc<Option<Box<dyn Fn() -> bool>>> = Rc::new(self.is_active);

        Box::new(
            floem::views::Container::with_id(
                self.id,
                floem::views::Label::derived(move || label.clone()),
            )
            .style(move |s| {
                let is_active = is_active.clone();
                s.with_shadcn_theme(move |s, t| {
                    let active = is_active.as_ref().as_ref().map(|f| f()).unwrap_or(false);
                    let base = s
                        .width_full()
                        .padding(8.0)
                        .padding_left(12.0)
                        .padding_right(12.0)
                        .border_radius(t.radius_sm)
                        .font_size(14.0)
                        .cursor(CursorStyle::Pointer)
                        .transition(
                            floem::style::Background,
                            floem::style::Transition::linear(millis(100)),
                        )
                        .hover(move |s| s.background(t.accent));
                    if active {
                        base.background(t.accent)
                            .color(t.accent_foreground)
                            .font_weight(Weight::MEDIUM)
                    } else {
                        base.background(peniko::Color::TRANSPARENT)
                            .color(t.foreground)
                    }
                })
            }),
        )
    }
}

// ============================================================================
// SidebarSeparator - Visual divider
// ============================================================================

/// Sidebar separator (horizontal line)
pub struct SidebarSeparator;

impl SidebarSeparator {
    /// Create a new separator
    pub fn new() -> Self {
        Self
    }
}

impl Default for SidebarSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarSeparator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}

impl IntoView for SidebarSeparator {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.width_full()
                    .height(1.0)
                    .margin_top(8.0)
                    .margin_bottom(8.0)
                    .background(t.border)
            })
        }))
    }
}

// ============================================================================
// Helper
// ============================================================================

fn millis(ms: u64) -> std::time::Duration {
    std::time::Duration::from_millis(ms)
}
