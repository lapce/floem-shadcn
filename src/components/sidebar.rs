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
//! use floem::view::ParentView;
//! use floem_shadcn::components::sidebar::*;
//!
//! let active = RwSignal::new("buttons");
//!
//! let sidebar = Sidebar::new()
//!     .header(
//!         SidebarHeader::new()
//!             .child(label(|| "My App"))
//!     )
//!     .content(
//!         SidebarContent::new().child(
//!             SidebarGroup::new()
//!                 .child(SidebarGroupLabel::new("Components"))
//!                 .child(
//!                     SidebarGroupContent::new().child(
//!                         SidebarMenu::new()
//!                             .child(
//!                                 SidebarMenuItem::new().child(
//!                                     SidebarMenuButton::new("Buttons")
//!                                         .is_active(move || active.get() == "buttons")
//!                                         .on_click_stop(move |_| active.set("buttons"))
//!                                 )
//!                             )
//!                             .child(
//!                                 SidebarMenuItem::new().child(
//!                                     SidebarMenuButton::new("Cards")
//!                                         .is_active(move || active.get() == "cards")
//!                                         .on_click_stop(move |_| active.set("cards"))
//!                                 )
//!                             )
//!                     )
//!                 )
//!         )
//!     )
//!     .footer(
//!         SidebarFooter::new()
//!             .child(label(|| "v1.0.0"))
//!     );
//! ```

use std::rc::Rc;

use floem::prelude::*;
use floem::style::CursorStyle;
use floem::text::Weight;
use floem::view::ParentView;
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
///     .header(SidebarHeader::new().child(label(|| "My App")))
///     .content(SidebarContent::new().child(/* menu items */))
///     .footer(SidebarFooter::new().child(label(|| "v1.0")))
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
pub struct SidebarHeader {
    id: ViewId,
}

impl SidebarHeader {
    /// Create a new sidebar header
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for SidebarHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarHeader {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarHeader {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.width_full()
                    .padding(16.0)
                    .border_bottom(1.0)
                    .border_color(t.border)
            })
        })
    }
}

impl ParentView for SidebarHeader {}

// ============================================================================
// SidebarContent - Main scrollable content area
// ============================================================================

/// Sidebar content area (scrollable).
pub struct SidebarContent {
    id: ViewId,
}

impl SidebarContent {
    /// Create a new sidebar content area
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for SidebarContent {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarContent {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| {
            s.flex_grow(1.0)
                .flex_basis(0.0)
                .min_height(0.0)
                .width_full()
                .flex_direction(floem::style::FlexDirection::Column)
                .padding(8.0)
                .gap(8.0)
        })
    }
}

impl ParentView for SidebarContent {}

// ============================================================================
// SidebarFooter - Footer section at bottom
// ============================================================================

/// Sidebar footer component
pub struct SidebarFooter {
    id: ViewId,
}

impl SidebarFooter {
    /// Create a new sidebar footer
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for SidebarFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarFooter {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarFooter {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.width_full()
                    .padding(16.0)
                    .border_top(1.0)
                    .border_color(t.border)
            })
        })
    }
}

impl ParentView for SidebarFooter {}

// ============================================================================
// SidebarGroup - Groups related menu items
// ============================================================================

/// Sidebar group container.
pub struct SidebarGroup {
    id: ViewId,
}

impl SidebarGroup {
    /// Create a new sidebar group
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for SidebarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarGroup {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| {
            s.flex_direction(floem::style::FlexDirection::Column)
                .width_full()
                .gap(4.0)
        })
    }
}

impl ParentView for SidebarGroup {}

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
pub struct SidebarGroupContent {
    id: ViewId,
}

impl SidebarGroupContent {
    /// Create a new group content container
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for SidebarGroupContent {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarGroupContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarGroupContent {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| {
            s.flex_direction(floem::style::FlexDirection::Column)
                .width_full()
        })
    }
}

impl ParentView for SidebarGroupContent {}

// ============================================================================
// SidebarGroupAction - Optional action button in group header
// ============================================================================

/// Sidebar group action button (appears next to group label)
pub struct SidebarGroupAction {
    id: ViewId,
}

impl SidebarGroupAction {
    /// Create a new group action
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for SidebarGroupAction {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarGroupAction {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarGroupAction {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.padding(4.0)
                    .border_radius(t.radius_sm)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.background(t.accent))
            })
        })
    }
}

impl ParentView for SidebarGroupAction {}

// ============================================================================
// SidebarMenu - Menu container
// ============================================================================

/// Sidebar menu container.
pub struct SidebarMenu {
    id: ViewId,
}

impl SidebarMenu {
    /// Create a new sidebar menu
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for SidebarMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarMenu {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarMenu {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| {
            s.flex_direction(floem::style::FlexDirection::Column)
                .width_full()
                .gap(2.0)
        })
    }
}

impl ParentView for SidebarMenu {}

// ============================================================================
// SidebarMenuItem - Menu item wrapper
// ============================================================================

/// Sidebar menu item wrapper
pub struct SidebarMenuItem {
    id: ViewId,
}

impl SidebarMenuItem {
    /// Create a new menu item
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for SidebarMenuItem {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarMenuItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarMenuItem {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| s.width_full())
    }
}

impl ParentView for SidebarMenuItem {}

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
