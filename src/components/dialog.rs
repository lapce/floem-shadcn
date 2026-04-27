//! Dialog component with builder-style API
//!
//! Based on shadcn/ui Dialog component for modal dialogs.
//!
//! # Example (recommended)
//!
//! ```rust
//! use floem_shadcn::components::dialog::{Dialog, DialogTrigger, DialogContent, DialogHeader, DialogFooter, DialogClose};
//! use floem_shadcn::components::button::Button;
//!
//! Dialog::new((
//!     DialogTrigger::new(Button::new("Open Dialog")),
//!     DialogContent::new((
//!         DialogHeader::new()
//!             .title("Are you sure?")
//!             .description("This action cannot be undone."),
//!         DialogFooter::new((
//!             DialogClose::new(Button::new("Cancel").outline()),
//!             Button::new("Continue").on_event_stop(floem::event::listener::Click, move |_, _| {
//!                 // do something
//!             }),
//!         )),
//!     )),
//! ));
//! ```
//!
//! # Components
//!
//! - `Dialog` - Root component that provides context
//! - `DialogTrigger` - Opens the dialog when clicked
//! - `DialogContent` - The modal content (includes overlay/backdrop automatically)
//! - `DialogHeader` - Container for title and description
//! - `DialogFooter` - Container for action buttons
//! - `DialogClose` - Closes the dialog when clicked
//!
//! # External state control
//!
//! Use `dialog.open_signal()` to get the signal for programmatic control:
//!
//! ```rust
//! let dialog = Dialog::new((trigger, content));
//! let open = dialog.open_signal();
//! // Later: open.set(true) to open programmatically
//! ```

use floem::prelude::*;
use floem::reactive::{Context, RwSignal, Scope, SignalGet, SignalUpdate};
use floem::views::Decorators;
use floem::views::Overlay;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// Dialog Context - passes the open signal to children via the reactive context.
#[derive(Clone, Copy)]
pub struct DialogContext {
    pub open: RwSignal<bool>,
}

/// Root dialog component that provides the open/close context.
pub struct Dialog<V> {
    id: ViewId,
    open: RwSignal<bool>,
    child: V,
    scope: Scope,
}

impl<V: IntoView + 'static> Dialog<V> {
    /// Create a new dialog. Children receive the `DialogContext`.
    pub fn new(child: V) -> Self {
        let open = RwSignal::new(false);
        let scope = Scope::current().create_child();
        scope.provide_context(DialogContext { open });
        Self {
            id: ViewId::new(),
            open,
            child,
            scope,
        }
    }

    /// Returns the signal controlling the dialog's open state.
    pub fn open_signal(&self) -> RwSignal<bool> {
        self.open
    }
}

impl<V: IntoView + 'static> HasViewId for Dialog<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Dialog<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let scope = self.scope;
        let child = self.child;
        let id = self.id;
        Box::new(scope.enter(move || floem::views::Container::with_id(id, child)))
    }
}

/// When clicked, opens the nearest parent dialog.
pub struct DialogTrigger<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogTrigger<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for DialogTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let ctx = Context::get::<DialogContext>();
        Box::new(
            floem::views::Container::with_id(self.id, self.child).on_event_stop(
                floem::event::listener::Click,
                move |_, _| {
                    if let Some(ctx) = ctx {
                        ctx.open.set(true);
                    }
                },
            ),
        )
    }
}

/// When clicked, closes the nearest parent dialog.
pub struct DialogClose<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogClose<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for DialogClose<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogClose<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let ctx = Context::get::<DialogContext>();
        Box::new(
            floem::views::Container::with_id(self.id, self.child).on_event_stop(
                floem::event::listener::Click,
                move |_, _| {
                    if let Some(ctx) = ctx {
                        ctx.open.set(false);
                    }
                },
            ),
        )
    }
}

/// The modal portal that contains the overlay backdrop and the content panel.
///
/// Children are automatically placed inside the centered modal card.
pub struct DialogContent {
    id: ViewId,
    children: Vec<Box<dyn View>>,
}

impl DialogContent {
    /// Create a new dialog content from any collection of views.
    pub fn new(children: impl floem::view::IntoViewIter) -> Self {
        Self {
            id: ViewId::new(),
            children: children.into_view_iter().collect(),
        }
    }
}

impl HasViewId for DialogContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for DialogContent {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let id = self.id;
        let children = self.children;
        let ctx = Context::get::<DialogContext>();
        if let Some(ctx) = ctx {
            let open = ctx.open;
            Box::new(
                Overlay::with_id(id).child(
                    floem::views::Stack::new((
                        floem::views::Container::new(())
                            .style(move |s| {
                                s.absolute()
                                    .inset_0()
                                    .background(peniko::Color::from_rgba8(0, 0, 0, 128))
                            })
                            .on_event_stop(floem::event::listener::Click, move |_, _| {
                                open.set(false)
                            }),
                        floem::views::Stack::vertical_from_iter(children)
                            .style(move |s| {
                                s.absolute()
                                    .inset_left_pct(50.0)
                                    .inset_top_pct(50.0)
                                    .z_index(10)
                                    .max_width(512.0)
                                    .rounded_lg()
                                    .p_6()
                                    .gap_4()
                                    .box_shadow_blur(8.0)
                                    .box_shadow_color(peniko::Color::from_rgba8(0, 0, 0, 60))
                            })
                            .style(move |s| {
                                s.with_shadcn_theme(move |s, t| {
                                    s.background(t.background).border_1().border_color(t.border)
                                })
                            }),
                    ))
                    .style(move |s| {
                        let is_open = open.get();
                        s.fixed()
                            .inset_0()
                            .w_full()
                            .h_full()
                            .apply_if(!is_open, |s| s.hide())
                    }),
                ),
            )
        } else {
            Box::new(floem::views::Stack::vertical_from_iter(children).style(|s| s.w_full()))
        }
    }
}

/// Header container for the dialog title and description.
pub struct DialogHeader {
    id: ViewId,
    title: Option<String>,
    description: Option<String>,
}

impl DialogHeader {
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            title: None,
            description: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl Default for DialogHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for DialogHeader {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for DialogHeader {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let mut children: Vec<Box<dyn View>> = Vec::new();
        if let Some(title) = self.title {
            children.push(Box::new(floem::views::Label::new(title).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.text_lg()
                        .leading_none()
                        .font_semibold()
                        .color(t.foreground)
                })
            })));
        }
        if let Some(description) = self.description {
            children.push(Box::new(floem::views::Label::new(description).style(|s| {
                s.with_shadcn_theme(|s, t| s.text_sm().color(t.muted_foreground))
            })));
        }
        Box::new(floem::views::Stack::vertical_from_iter(children).style(|s| s.gap_2()))
    }
}

/// Footer container, usually holding action buttons.
pub struct DialogFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogFooter<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for DialogFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogFooter<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.flex().flex_row().justify_end().gap_2()),
        )
    }
}

/// Standalone dialog title.
pub struct DialogTitle<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogTitle<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for DialogTitle<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogTitle<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.with_shadcn_theme(|s, t| s.text_lg().color(t.foreground))),
        )
    }
}

/// Standalone dialog description.
pub struct DialogDescription<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogDescription<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for DialogDescription<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogDescription<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.with_shadcn_theme(|s, t| s.text_sm().color(t.muted_foreground))),
        )
    }
}
