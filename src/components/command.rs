//! Command palette component with builder-style API
//!
//! Based on shadcn/ui Command - a command palette for quick actions and search.
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
//!     .child(
//!         CommandList::new()
//!             .child(
//!                 CommandGroup::new("Suggestions")
//!                     .child(CommandItem::new("calendar", "Calendar"))
//!                     .child(CommandItem::new("search", "Search Emoji"))
//!             )
//!     );
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::style::CursorStyle;
use floem::view::ParentView;
use floem::views::{Decorators, Stem};
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

pub struct Command {
    content_id: ViewId,
    #[allow(dead_code)]
    search: RwSignal<String>,
    placeholder: String,
}
impl Command {
    pub fn new(search: RwSignal<String>) -> Self {
        Self {
            content_id: ViewId::new(),
            search,
            placeholder: "Type a command...".to_string(),
        }
    }
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}
impl HasViewId for Command {
    fn view_id(&self) -> ViewId {
        self.content_id
    }
}
impl IntoView for Command {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let placeholder = self.placeholder;
        let content_id = self.content_id;
        let input = TextInput::new(RwSignal::new(String::new()))
            .placeholder(placeholder)
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.w_full()
                        .h_9()
                        .px_3()
                        .py_2()
                        .text_sm()
                        .border_0()
                        .border_bottom(1.0)
                        .border_color(t.border)
                        .background(peniko::Color::TRANSPARENT)
                        .color(t.foreground)
                })
            });
        let content = Stem::with_id(content_id).style(|s| s.p_1().flex_col());
        Box::new(floem::views::Stack::vertical((input, content)).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.w_full()
                    .flex_col()
                    .background(t.popover)
                    .color(t.popover_foreground)
                    .rounded_md()
            })
        }))
    }
}
impl ParentView for Command {}

pub struct CommandInput {
    id: ViewId,
    #[allow(dead_code)]
    search: RwSignal<String>,
    placeholder: String,
}
impl CommandInput {
    pub fn new(search: RwSignal<String>) -> Self {
        Self {
            id: ViewId::new(),
            search,
            placeholder: "Search...".to_string(),
        }
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let placeholder = self.placeholder;
        Box::new(
            TextInput::new(RwSignal::new(String::new()))
                .placeholder(placeholder)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.w_full()
                            .h_9()
                            .px_3()
                            .py_2()
                            .text_sm()
                            .border_0()
                            .border_bottom(1.0)
                            .border_color(t.border)
                            .background(peniko::Color::TRANSPARENT)
                            .color(t.foreground)
                    })
                }),
        )
    }
}

pub struct CommandList {
    id: ViewId,
}
impl CommandList {
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
    type Intermediate = Stem;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Stem::with_id(self.id).style(|s| s.p_1().max_height(300.0).flex_col())
    }
}
impl ParentView for CommandList {}

pub struct CommandEmpty {
    id: ViewId,
    text: String,
}
impl CommandEmpty {
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.w_full()
                    .p_6()
                    .text_sm()
                    .color(t.muted_foreground)
                    .justify_center()
            })
        }))
    }
}

pub struct CommandGroup {
    items_id: ViewId,
    heading: String,
}
impl CommandGroup {
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            items_id: ViewId::new(),
            heading: heading.into(),
        }
    }
}
impl HasViewId for CommandGroup {
    fn view_id(&self) -> ViewId {
        self.items_id
    }
}
impl IntoView for CommandGroup {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let heading = self.heading;
        let items_id = self.items_id;
        let heading_view = floem::views::Label::new(heading).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.px_2()
                    .py_1p5()
                    .text_xs()
                    .font_medium()
                    .color(t.muted_foreground)
            })
        });
        let items = Stem::with_id(items_id).style(|s| s.flex_col());
        Box::new(floem::views::Stack::vertical((heading_view, items)))
    }
}
impl ParentView for CommandGroup {}

pub struct CommandItem {
    id: ViewId,
    #[allow(dead_code)]
    value: String,
    text: String,
    disabled: bool,
    on_select: Option<Box<dyn Fn() + 'static>>,
}
impl CommandItem {
    pub fn new(value: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            value: value.into(),
            text: text.into(),
            disabled: false,
            on_select: None,
        }
    }
    pub fn on_select(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        let disabled = self.disabled;
        let on_select = self.on_select;
        let label = floem::views::Label::new(text).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .w_full()
                    .px_2()
                    .py_1p5()
                    .text_sm()
                    .rounded_sm()
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
                Box::new(label.on_event_stop(floem::event::listener::Click, move |_, _| handler()))
            } else {
                Box::new(label)
            }
        } else {
            Box::new(label)
        }
    }
}

pub struct CommandItemCustom<V> {
    id: ViewId,
    child: V,
    disabled: bool,
    on_select: Option<Box<dyn Fn() + 'static>>,
}
impl<V: IntoView + 'static> CommandItemCustom<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            disabled: false,
            on_select: None,
        }
    }
    pub fn on_select(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let disabled = self.disabled;
        let on_select = self.on_select;
        let container = floem::views::Container::new(self.child).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .w_full()
                    .px_2()
                    .py_1p5()
                    .rounded_sm()
                    .flex_row()
                    .items_center()
                    .gap_2()
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
                Box::new(
                    container.on_event_stop(floem::event::listener::Click, move |_, _| handler()),
                )
            } else {
                Box::new(container)
            }
        } else {
            Box::new(container)
        }
    }
}

pub struct CommandSeparator;
impl CommandSeparator {
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.w_full()
                    .h_px()
                    .background(t.border)
                    .margin_left(-4.0)
                    .margin_right(-4.0)
            })
        }))
    }
}

pub struct CommandShortcut {
    id: ViewId,
    keys: String,
}
impl CommandShortcut {
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let keys = self.keys;
        Box::new(floem::views::Label::new(keys).style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_xs().color(t.muted_foreground).ml_auto())
        }))
    }
}
