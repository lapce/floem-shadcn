//! Command component with builder-style API
//!
//! Based on shadcn/ui Command - a command palette for searching and executing commands.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::command::*;
//!
//! let search = RwSignal::new(String::new());
//!
//! Command::new(search)
//!     .placeholder("Type a command or search...")
//!     .content((
//!         CommandGroup::new("Suggestions").items((
//!             CommandItem::new("calendar", "Calendar"),
//!             CommandItem::new("search", "Search Emoji"),
//!             CommandItem::new("calculator", "Calculator"),
//!         )),
//!         CommandSeparator::new(),
//!         CommandGroup::new("Settings").items((
//!             CommandItem::new("profile", "Profile"),
//!             CommandItem::new("settings", "Settings"),
//!         )),
//!     ));
//! ```

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::style::CursorStyle;
use floem::views::{Decorators, text_input};
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Command
// ============================================================================

/// Command palette container
pub struct Command<C> {
    id: ViewId,
    search: RwSignal<String>,
    placeholder: String,
    content: Option<C>,
}

impl Command<()> {
    /// Create a new command palette
    pub fn new(search: RwSignal<String>) -> Self {
        Self {
            id: ViewId::new(),
            search,
            placeholder: "Type a command...".to_string(),
            content: None,
        }
    }
}

impl<C> Command<C> {
    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the command content
    pub fn content<C2: IntoView + 'static>(self, content: C2) -> Command<C2> {
        Command {
            id: self.id,
            search: self.search,
            placeholder: self.placeholder,
            content: Some(content),
        }
    }
}

impl<C: IntoView + 'static> HasViewId for Command<C> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<C: IntoView + 'static> IntoView for Command<C> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let search = self.search;
        let placeholder = self.placeholder;

        // Search input
        let input = text_input(search).placeholder(placeholder).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .padding(12.0)
                    .font_size(14.0)
                    .border(0.0)
                    .border_bottom(1.0)
                    .border_color(t.border)
                    .background(floem::peniko::Color::TRANSPARENT)
                    .color(t.foreground)
            })
        });

        // Content area
        let content_view = if let Some(content) = self.content {
            floem::views::Container::new(content)
                .style(|s| {
                    s.padding(4.0)
                        .max_height(300.0)
                        .display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Column)
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        Box::new(
            floem::views::Stack::vertical((input, content_view)).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width_full()
                        .background(t.popover)
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(t.radius)
                })
            }),
        )
    }
}

// ============================================================================
// CommandInput
// ============================================================================

/// Standalone command input (for use outside Command)
pub struct CommandInput {
    id: ViewId,
    search: RwSignal<String>,
    placeholder: String,
}

impl CommandInput {
    /// Create a new command input
    pub fn new(search: RwSignal<String>) -> Self {
        Self {
            id: ViewId::new(),
            search,
            placeholder: "Search...".to_string(),
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl HasViewId for CommandInput {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CommandInput {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let search = self.search;
        let placeholder = self.placeholder;

        Box::new(text_input(search).placeholder(placeholder).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .padding(12.0)
                    .font_size(14.0)
                    .border(0.0)
                    .background(floem::peniko::Color::TRANSPARENT)
                    .color(t.foreground)
            })
        }))
    }
}

// ============================================================================
// CommandList
// ============================================================================

/// Container for command items
pub struct CommandList<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CommandList<V> {
    /// Create a new command list
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CommandList<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CommandList<V> {
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
// CommandEmpty
// ============================================================================

/// Displayed when no results are found
pub struct CommandEmpty {
    id: ViewId,
    text: String,
}

impl CommandEmpty {
    /// Create a new empty state
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl Default for CommandEmpty {
    fn default() -> Self {
        Self::new("No results found.")
    }
}

impl HasViewId for CommandEmpty {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CommandEmpty {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;

        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .padding(24.0)
                    .font_size(14.0)
                    .color(t.muted_foreground)
                    .justify_center()
            })
        }))
    }
}

// ============================================================================
// CommandGroup
// ============================================================================

/// Group of related commands
pub struct CommandGroup<V> {
    id: ViewId,
    heading: String,
    items: Option<V>,
}

impl CommandGroup<()> {
    /// Create a new command group
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            heading: heading.into(),
            items: None,
        }
    }
}

impl<V> CommandGroup<V> {
    /// Set the group items
    pub fn items<V2: IntoView + 'static>(self, items: V2) -> CommandGroup<V2> {
        CommandGroup {
            id: self.id,
            heading: self.heading,
            items: Some(items),
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CommandGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CommandGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let heading = self.heading;

        // Group heading
        let heading_view = floem::views::Label::new(heading).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.padding_left(8.0)
                    .padding_right(8.0)
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .font_size(12.0)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .color(t.muted_foreground)
            })
        });

        // Items
        let items_view = if let Some(items) = self.items {
            floem::views::Container::new(items)
                .style(|s| {
                    s.display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Column)
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        Box::new(floem::views::Stack::vertical((heading_view, items_view)))
    }
}

// ============================================================================
// CommandItem
// ============================================================================

/// Individual command item
pub struct CommandItem {
    id: ViewId,
    #[allow(dead_code)]
    value: String,
    text: String,
    disabled: bool,
    on_select: Option<Box<dyn Fn() + 'static>>,
}

impl CommandItem {
    /// Create a new command item
    pub fn new(value: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            value: value.into(),
            text: text.into(),
            disabled: false,
            on_select: None,
        }
    }

    /// Set selection handler
    pub fn on_select(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl HasViewId for CommandItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CommandItem {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        let disabled = self.disabled;
        let on_select = self.on_select;

        let label = floem::views::Label::new(text).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .width_full()
                    .padding_left(8.0)
                    .padding_right(8.0)
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .font_size(14.0)
                    .border_radius(4.0)
                    .cursor(if disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    });
                if disabled {
                    base.color(t.muted_foreground)
                } else {
                    base.color(t.foreground)
                        .hover(|s| s.background(t.accent).color(t.accent_foreground))
                }
            })
        });

        if let Some(handler) = on_select {
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
// CommandItemCustom
// ============================================================================

/// Command item with custom content
pub struct CommandItemCustom<V> {
    id: ViewId,
    child: V,
    disabled: bool,
    on_select: Option<Box<dyn Fn() + 'static>>,
}

impl<V: IntoView + 'static> CommandItemCustom<V> {
    /// Create a new command item with custom content
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            disabled: false,
            on_select: None,
        }
    }

    /// Set selection handler
    pub fn on_select(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<V: IntoView + 'static> HasViewId for CommandItemCustom<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CommandItemCustom<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let disabled = self.disabled;
        let on_select = self.on_select;

        let container = floem::views::Container::new(self.child).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .width_full()
                    .padding_left(8.0)
                    .padding_right(8.0)
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .border_radius(4.0)
                    .display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Row)
                    .items_center()
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

        if let Some(handler) = on_select {
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
// CommandSeparator
// ============================================================================

/// Separator between command groups
pub struct CommandSeparator;

impl CommandSeparator {
    /// Create a new separator
    pub fn new() -> Self {
        Self
    }
}

impl Default for CommandSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for CommandSeparator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}

impl IntoView for CommandSeparator {
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
// CommandShortcut
// ============================================================================

/// Keyboard shortcut hint for command item
pub struct CommandShortcut {
    id: ViewId,
    keys: String,
}

impl CommandShortcut {
    /// Create a new shortcut hint
    pub fn new(keys: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            keys: keys.into(),
        }
    }
}

impl HasViewId for CommandShortcut {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CommandShortcut {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let keys = self.keys;

        Box::new(floem::views::Label::new(keys).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(12.0)
                    .color(t.muted_foreground)
                    .margin_left(16.0)
            })
        }))
    }
}
