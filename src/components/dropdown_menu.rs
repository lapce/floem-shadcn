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

use crate::theme::ShadcnThemeExt;

// ============================================================================
// DropdownMenu
// ============================================================================

/// Dropdown menu container
pub struct DropdownMenu<T, C> {
    open: RwSignal<bool>,
    trigger: Option<T>,
    content: Option<C>,
}

impl DropdownMenu<(), ()> {
    /// Create a new dropdown menu
    pub fn new(open: RwSignal<bool>) -> Self {
        Self {
            open,
            trigger: None,
            content: None,
        }
    }
}

impl<T, C> DropdownMenu<T, C> {
    /// Set the trigger element
    pub fn trigger<T2: Fn() -> V, V: IntoView + 'static>(self, trigger: T2) -> DropdownMenu<T2, C> {
        DropdownMenu {
            open: self.open,
            trigger: Some(trigger),
            content: self.content,
        }
    }

    /// Set the menu content
    pub fn content<C2: IntoView + 'static>(self, content: C2) -> DropdownMenu<T, C2> {
        DropdownMenu {
            open: self.open,
            trigger: self.trigger,
            content: Some(content),
        }
    }
}

impl<T, C, TV> DropdownMenu<T, C>
where
    T: Fn() -> TV + 'static,
    C: IntoView + 'static,
    TV: IntoView + 'static,
{
    /// Build the dropdown menu view
    pub fn build(self) -> impl IntoView {
        let open = self.open;
        let trigger = self.trigger;
        let content = self.content;

        // Trigger wrapper
        let trigger_view = if let Some(trigger_fn) = trigger {
            floem::views::Container::new(trigger_fn())
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_click_stop(move |_| {
                    open.update(|v| *v = !*v);
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        // Menu content
        let content_view = if let Some(menu_content) = content {
            floem::views::Container::new(menu_content)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let is_open = open.get();
                        let base = s
                            .min_width(180.0)
                            .padding_top(4.0)
                            .padding_bottom(4.0)
                            .background(t.popover)
                            .border(1.0)
                            .border_color(t.border)
                            .border_radius(t.radius)
                            .box_shadow_blur(8.0)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                            .position(floem::style::Position::Absolute)
                            .inset_top_pct(100.0)
                            .margin_top(4.0)
                            .inset_left(0.0)
                            .z_index(50)
                            .display(floem::style::Display::Flex)
                            .flex_direction(floem::style::FlexDirection::Column);
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
            .style(|s| s.position(floem::style::Position::Relative))
    }
}

impl<T, C, TV> HasViewId for DropdownMenu<T, C>
where
    T: Fn() -> TV + 'static,
    C: IntoView + 'static,
    TV: IntoView + 'static,
{
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}

impl<T, C, TV> IntoView for DropdownMenu<T, C>
where
    T: Fn() -> TV + 'static,
    C: IntoView + 'static,
    TV: IntoView + 'static,
{
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// ============================================================================
// DropdownMenuContent
// ============================================================================

/// Styled container for dropdown menu items
pub struct DropdownMenuContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DropdownMenuContent<V> {
    /// Create new dropdown menu content
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Column)
            }),
        )
    }
}

// ============================================================================
// DropdownMenuItem
// ============================================================================

/// Individual menu item
pub struct DropdownMenuItem {
    id: ViewId,
    text: String,
    disabled: bool,
    destructive: bool,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl DropdownMenuItem {
    /// Create a new menu item
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            disabled: false,
            destructive: false,
            on_click: None,
        }
    }

    /// Set click handler
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Mark as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Mark as destructive (red text)
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        let disabled = self.disabled;
        let destructive = self.destructive;
        let on_click = self.on_click;

        let label = floem::views::Label::new(text).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .width_full()
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .font_size(14.0)
                    .cursor(if disabled {
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
                Box::new(label.on_click_stop(move |_| handler()))
            } else {
                Box::new(label)
            }
        } else {
            Box::new(label)
        }
    }
}

// ============================================================================
// DropdownMenuItemCustom
// ============================================================================

/// Menu item with custom content
pub struct DropdownMenuItemCustom<V> {
    id: ViewId,
    child: V,
    disabled: bool,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl<V: IntoView + 'static> DropdownMenuItemCustom<V> {
    /// Create a new custom menu item
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            disabled: false,
            on_click: None,
        }
    }

    /// Set click handler
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Mark as disabled
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let disabled = self.disabled;
        let on_click = self.on_click;

        let container = floem::views::Container::new(self.child).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .width_full()
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .cursor(if disabled {
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
                Box::new(container.on_click_stop(move |_| handler()))
            } else {
                Box::new(container)
            }
        } else {
            Box::new(container)
        }
    }
}

// ============================================================================
// DropdownMenuSeparator
// ============================================================================

/// Separator line between menu items
pub struct DropdownMenuSeparator;

impl DropdownMenuSeparator {
    /// Create a new separator
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .height(1.0)
                    .background(t.border)
                    .margin_top(4.0)
                    .margin_bottom(4.0)
            })
        }))
    }
}

// ============================================================================
// DropdownMenuLabel
// ============================================================================

/// Label/header for a group of menu items
pub struct DropdownMenuLabel {
    id: ViewId,
    text: String,
}

impl DropdownMenuLabel {
    /// Create a new menu label
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;

        Box::new(floem::views::Label::new(text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .padding_top(8.0)
                    .padding_bottom(4.0)
                    .font_size(12.0)
                    .font_weight(floem::text::Weight::SEMIBOLD)
                    .color(t.foreground)
            })
        }))
    }
}

// ============================================================================
// DropdownMenuGroup
// ============================================================================

/// Group of related menu items
pub struct DropdownMenuGroup<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DropdownMenuGroup<V> {
    /// Create a new menu group
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Column)
            }),
        )
    }
}

// ============================================================================
// DropdownMenuShortcut
// ============================================================================

/// Keyboard shortcut hint displayed in menu item
pub struct DropdownMenuShortcut {
    id: ViewId,
    text: String,
}

impl DropdownMenuShortcut {
    /// Create a new shortcut hint
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;

        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(12.0)
                    .color(t.muted_foreground)
                    .margin_left(16.0)
            })
        }))
    }
}
