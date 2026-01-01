//! Command component with builder-style API
//!
//! Based on shadcn/ui Command - a command palette for searching and executing commands.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem::view::ParentView;
//! use floem_shadcn::components::command::*;
//!
//! let search = RwSignal::new(String::new());
//!
//! // Command with input and content using ParentView
//! Command::new(search)
//!     .placeholder("Type a command or search...")
//!     .child(
//!         CommandList::new()
//!             .child(
//!                 CommandGroup::new("Suggestions")
//!                     .child(CommandItem::new("calendar", "Calendar"))
//!                     .child(CommandItem::new("search", "Search Emoji"))
//!             )
//!             .child(CommandSeparator::new())
//!             .child(
//!                 CommandGroup::new("Settings")
//!                     .child(CommandItem::new("profile", "Profile"))
//!                     .child(CommandItem::new("settings", "Settings"))
//!             )
//!     );
//! ```

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::style::CursorStyle;
use floem::view::ParentView;
use floem::views::{Decorators, Stem};
use floem::{HasViewId, ViewId};

use crate::text::TextInput;
use crate::theme::ShadcnThemeExt;

// ============================================================================
// Command
// ============================================================================

/// Command palette container with search input
///
/// The Command component includes a search input at the top.
/// Add content using `.child()` - typically a `CommandList` containing groups and items.
pub struct Command {
    /// ID for the content area (where children go via ParentView)
    content_id: ViewId,
    search: RwSignal<String>,
    placeholder: String,
}

impl Command {
    /// Create a new command palette
    pub fn new(search: RwSignal<String>) -> Self {
        Self {
            content_id: ViewId::new(),
            search,
            placeholder: "Type a command...".to_string(),
        }
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl HasViewId for Command {
    fn view_id(&self) -> ViewId {
        // Return the content_id so ParentView adds children to the content area
        self.content_id
    }
}

impl IntoView for Command {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let search = self.search;
        let placeholder = self.placeholder;
        let content_id = self.content_id;

        // Search input at the top using floem-shadcn's TextInput
        let input = TextInput::new()
            .placeholder(placeholder)
            .value(move || search.get())
            .on_update(move |text| search.set(text.to_string()))
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width_full()
                        .min_height(36.0)
                        .padding_left(12.0)
                        .padding_right(12.0)
                        .padding_top(8.0)
                        .padding_bottom(8.0)
                        .font_size(14.0)
                        .border(0.0)
                        .border_bottom(1.0)
                        .border_color(t.border)
                        .background(floem::peniko::Color::TRANSPARENT)
                        .color(t.foreground)
                })
            });

        // Content area - children are added here via ParentView
        let content = Stem::with_id(content_id).style(|s| {
            s.padding(4.0)
                .flex_direction(floem::style::FlexDirection::Column)
        });

        // Stack: input on top, content below
        Box::new(floem::views::Stack::vertical((input, content)).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full()
                    .flex_direction(floem::style::FlexDirection::Column)
                    .background(t.popover)
                    .color(t.popover_foreground)
                    .border_radius(t.radius)
            })
        }))
    }
}

impl ParentView for Command {}

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

        Box::new(
            TextInput::new()
                .placeholder(placeholder)
                .value(move || search.get())
                .on_update(move |text| search.set(text.to_string()))
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.width_full()
                            .min_height(36.0)
                            .padding_left(12.0)
                            .padding_right(12.0)
                            .padding_top(8.0)
                            .padding_bottom(8.0)
                            .font_size(14.0)
                            .border(0.0)
                            .border_bottom(1.0)
                            .border_color(t.border)
                            .background(floem::peniko::Color::TRANSPARENT)
                            .color(t.foreground)
                    })
                }),
        )
    }
}

// ============================================================================
// CommandList
// ============================================================================

/// Container for command items (scrollable area)
pub struct CommandList {
    id: ViewId,
}

impl CommandList {
    /// Create a new command list
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for CommandList {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for CommandList {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CommandList {
    type V = Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Stem::with_id(self.id).style(|s| {
            s.padding(4.0)
                .max_height(300.0)
                .flex_direction(floem::style::FlexDirection::Column)
        })
    }
}

impl ParentView for CommandList {}

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
///
/// Uses ParentView pattern for ergonomic child composition.
pub struct CommandGroup {
    /// ID for the items area (where children go via ParentView)
    items_id: ViewId,
    heading: String,
}

impl CommandGroup {
    /// Create a new command group
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            items_id: ViewId::new(),
            heading: heading.into(),
        }
    }
}

impl HasViewId for CommandGroup {
    fn view_id(&self) -> ViewId {
        // Return items_id so ParentView adds children to the items area
        self.items_id
    }
}

impl IntoView for CommandGroup {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let heading = self.heading;
        let items_id = self.items_id;

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

        // Items area - children are added here via ParentView
        let items = Stem::with_id(items_id)
            .style(|s| s.flex_direction(floem::style::FlexDirection::Column));

        // Stack: heading on top, items below
        Box::new(floem::views::Stack::vertical((heading_view, items)))
    }
}

impl ParentView for CommandGroup {}

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
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .font_size(14.0)
                    .border_radius(2.0)
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
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .border_radius(2.0)
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
                    .margin_left(-4.0)
                    .margin_right(-4.0)
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
                    .margin_left(floem::style::PxPctAuto::Auto)
            })
        }))
    }
}
