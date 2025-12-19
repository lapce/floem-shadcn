//! Menubar component with builder-style API
//!
//! Based on shadcn/ui Menubar - a horizontal menu bar for application menus.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::menubar::*;
//!
//! Menubar::new((
//!     MenubarMenu::new("File").content((
//!         MenubarItem::new("New Tab").shortcut("⌘T"),
//!         MenubarItem::new("New Window").shortcut("⌘N"),
//!         MenubarSeparator::new(),
//!         MenubarItem::new("Share"),
//!         MenubarSeparator::new(),
//!         MenubarItem::new("Print").shortcut("⌘P"),
//!     )),
//!     MenubarMenu::new("Edit").content((
//!         MenubarItem::new("Undo").shortcut("⌘Z"),
//!         MenubarItem::new("Redo").shortcut("⇧⌘Z"),
//!     )),
//! ));
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Menubar
// ============================================================================

/// Horizontal menu bar container
pub struct Menubar<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> Menubar<V> {
    /// Create a new menubar
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for Menubar<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Menubar<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Row)
                    .items_center()
                    .padding(4.0)
                    .background(t.background)
                    .border_bottom(1.0)
                    .border_color(t.border)
            })
        }))
    }
}

// ============================================================================
// MenubarMenu
// ============================================================================

/// Individual menu in the menubar
pub struct MenubarMenu<C> {
    id: ViewId,
    label: String,
    content: Option<C>,
}

impl MenubarMenu<()> {
    /// Create a new menu
    pub fn new(label: impl Into<String>) -> Self { Self { id: ViewId::new(),
            label: label.into(),
            content: None,
        }
    }
}

impl<C> MenubarMenu<C> {
    /// Set the dropdown content
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let label = self.label;
        let is_open = RwSignal::new(false);

        // Menu trigger
        let trigger = floem::views::Label::new(label)
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let open = is_open.get();
                    let base = s
                        .padding_left(12.0)
                        .padding_right(12.0)
                        .padding_top(6.0)
                        .padding_bottom(6.0)
                        .font_size(14.0)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .color(t.foreground)
                        .border_radius(t.radius)
                        .cursor(CursorStyle::Pointer);
                    if open {
                        base.background(t.accent).color(t.accent_foreground)
                    } else {
                        base.hover(|s| s.background(t.accent).color(t.accent_foreground))
                    }
                })
            })
            .on_click_stop(move |_| {
                is_open.update(|v| *v = !*v);
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
                            .min_width(180.0)
                            .padding_top(4.0)
                            .padding_bottom(4.0)
                            .background(t.popover)
                            .border(1.0)
                            .border_color(t.border)
                            .border_radius(t.radius)
                            .box_shadow_blur(8.0)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                            .z_index(100)
                            .display(floem::style::Display::Flex)
                            .flex_direction(floem::style::FlexDirection::Column);
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

        // Backdrop to close - using absolute positioning with large area
        let backdrop = floem::views::Empty::new()
            .style(move |s| {
                let open = is_open.get();
                let base = s
                    .position(floem::style::Position::Absolute)
                    .inset_top(-1000.0)
                    .inset_left(-1000.0)
                    .width(3000.0)
                    .height(3000.0)
                    .z_index(99);

                if open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
            .on_click_stop(move |_| {
                is_open.set(false);
            });

        Box::new(
            floem::views::Container::new(floem::views::stack((trigger, backdrop, dropdown)))
                .style(|s| s.position(floem::style::Position::Relative)),
        )
    }
}

// ============================================================================
// MenubarTrigger
// ============================================================================

/// Standalone menu trigger
pub struct MenubarTrigger {
    id: ViewId,
    label: String,
}

impl MenubarTrigger {
    /// Create a new trigger
    pub fn new(label: impl Into<String>) -> Self { Self { id: ViewId::new(),
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let label = self.label;

        Box::new(floem::views::Label::with_id(self.id, label).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.padding_left(12.0)
                    .padding_right(12.0)
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .font_size(14.0)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .color(t.foreground)
                    .border_radius(t.radius)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.background(t.accent).color(t.accent_foreground))
            })
        }))
    }
}

// ============================================================================
// MenubarContent
// ============================================================================

/// Content container for menu dropdown
pub struct MenubarContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> MenubarContent<V> {
    /// Create new menu content
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for MenubarContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for MenubarContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Column)
        }))
    }
}

// ============================================================================
// MenubarItem
// ============================================================================

/// Individual menu item
pub struct MenubarItem {
    id: ViewId,
    label: String,
    shortcut: Option<String>,
    disabled: bool,
    on_select: Option<Box<dyn Fn() + 'static>>,
}

impl MenubarItem {
    /// Create a new menu item
    pub fn new(label: impl Into<String>) -> Self { Self { id: ViewId::new(),
            label: label.into(),
            shortcut: None,
            disabled: false,
            on_select: None,
        }
    }

    /// Add keyboard shortcut hint
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self { self.shortcut = Some(shortcut.into());
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled;
        self
    }

    /// Set selection handler
    pub fn on_select(mut self, handler: impl Fn() + 'static) -> Self { self.on_select = Some(Box::new(handler));
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let label = self.label;
        let shortcut = self.shortcut;
        let disabled = self.disabled;
        let on_select = self.on_select;

        // Label
        let label_view = floem::views::Label::new(label).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.font_size(14.0).flex_grow(1.0);
                if disabled {
                    base.color(t.muted_foreground)
                } else {
                    base.color(t.foreground)
                }
            })
        });

        // Shortcut hint
        let shortcut_view = if let Some(sc) = shortcut {
            floem::views::Label::new(sc)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.font_size(12.0)
                            .color(t.muted_foreground)
                            .margin_left(24.0)
                    })
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        let row = floem::views::h_stack((label_view, shortcut_view)).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .width_full()
                    .padding_left(8.0)
                    .padding_right(8.0)
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .border_radius(4.0)
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

        if let Some(handler) = on_select {
            if !disabled {
                Box::new(row.on_click_stop(move |_| handler()))
            } else {
                Box::new(row)
            }
        } else {
            Box::new(row)
        }
    }
}

// ============================================================================
// MenubarSeparator
// ============================================================================

/// Separator between menu items
pub struct MenubarSeparator;

impl MenubarSeparator {
    /// Create a new separator
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
// MenubarCheckboxItem
// ============================================================================

/// Checkbox menu item
pub struct MenubarCheckboxItem {
    id: ViewId,
    label: String,
    checked: RwSignal<bool>,
    disabled: bool,
}

impl MenubarCheckboxItem {
    /// Create a new checkbox item
    pub fn new(label: impl Into<String>, checked: RwSignal<bool>) -> Self { Self { id: ViewId::new(),
            label: label.into(),
            checked,
            disabled: false,
        }
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled;
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let label = self.label;
        let checked = self.checked;
        let disabled = self.disabled;

        // Checkbox indicator
        let check_indicator =
            floem::views::Label::derived(move || if checked.get() { "✓" } else { " " }.to_string())
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.width(16.0).font_size(12.0).color(t.foreground)
                    })
                });

        // Label
        let label_view = floem::views::Label::new(label).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s.font_size(14.0).flex_grow(1.0);
                if disabled {
                    base.color(t.muted_foreground)
                } else {
                    base.color(t.foreground)
                }
            })
        });

        let row = floem::views::h_stack((check_indicator, label_view)).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .width_full()
                    .padding_left(8.0)
                    .padding_right(8.0)
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .border_radius(4.0)
                    .gap(8.0)
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

        if disabled {
            Box::new(row)
        } else {
            Box::new(row.on_click_stop(move |_| {
                checked.update(|v| *v = !*v);
            }))
        }
    }
}

// ============================================================================
// MenubarShortcut
// ============================================================================

/// Keyboard shortcut display
pub struct MenubarShortcut {
    id: ViewId,
    keys: String,
}

impl MenubarShortcut {
    /// Create a new shortcut display
    pub fn new(keys: impl Into<String>) -> Self { Self { id: ViewId::new(), keys: keys.into() }
    }
}


impl HasViewId for MenubarShortcut {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for MenubarShortcut {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let keys = self.keys;

        Box::new(floem::views::Label::new(keys).style(|s| {
            s.with_shadcn_theme(move |s, t| s.font_size(12.0).color(t.muted_foreground))
        }))
    }
}
