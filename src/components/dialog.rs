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
//!             Button::new("Continue").on_click_stop(move |_| {
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

// ============================================================================
// Dialog Context - passes open signal to children via reactive Context
// ============================================================================

/// Dialog context that holds the open signal
///
/// This is provided via `Context::provide` and can be accessed by child
/// components using `Context::get::<DialogContext>()`.
#[derive(Clone, Copy)]
pub struct DialogContext {
    pub open: RwSignal<bool>,
}

// ============================================================================
// Dialog
// ============================================================================

/// Dialog (modal) root component
///
/// Contains both the trigger and content. Uses internal state management
/// that is shared via context with child components.
pub struct Dialog<V> {
    id: ViewId,
    open: RwSignal<bool>,
    child: V,
    scope: Scope,
}

impl<V: IntoView + 'static> Dialog<V> {
    /// Create a new dialog with internal state management
    ///
    /// The dialog state is managed internally and shared via context.
    /// Use `DialogTrigger` to open the dialog and `DialogClose` to close it.
    ///
    /// # Example
    /// ```rust
    /// Dialog::new((
    ///     DialogTrigger::new(Button::new("Open")),
    ///     DialogContent::new((
    ///         DialogHeader::new().title("Title"),
    ///         DialogFooter::new(DialogClose::new(Button::new("Close"))),
    ///     )),
    /// ))
    /// ```
    pub fn new(child: V) -> Self {
        let open = RwSignal::new(false);
        let scope = Scope::current().create_child();

        // Provide the dialog context in the child scope
        scope.provide_context(DialogContext { open });

        Self {
            id: ViewId::new(),
            open,
            child,
            scope,
        }
    }

    /// Get the open signal for external control
    ///
    /// Use this when you need to open the dialog programmatically from outside.
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let scope = self.scope;
        let child = self.child;
        let id = self.id;

        // Build the child view within the dialog's scope so it has access to context
        Box::new(scope.enter(move || floem::views::Container::with_id(id, child)))
    }
}

// ============================================================================
// DialogTrigger
// ============================================================================

/// Trigger element that opens the dialog when clicked
///
/// Reads the dialog's open signal from context and sets it to true on click.
pub struct DialogTrigger<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogTrigger<V> {
    /// Create a new dialog trigger wrapping the given element
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        // Get the dialog context from the current scope
        let ctx = Context::get::<DialogContext>();

        Box::new(
            floem::views::Container::with_id(self.id, self.child).on_click_stop(move |_| {
                if let Some(ctx) = ctx {
                    ctx.open.set(true);
                }
            }),
        )
    }
}

// ============================================================================
// DialogClose
// ============================================================================

/// Element that closes the dialog when clicked
///
/// Reads the dialog's open signal from context and sets it to false on click.
pub struct DialogClose<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogClose<V> {
    /// Create a new dialog close element wrapping the given element
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        // Get the dialog context from the current scope
        let ctx = Context::get::<DialogContext>();

        Box::new(
            floem::views::Container::with_id(self.id, self.child).on_click_stop(move |_| {
                if let Some(ctx) = ctx {
                    ctx.open.set(false);
                }
            }),
        )
    }
}

// ============================================================================
// DialogContent
// ============================================================================

/// Dialog content container with integrated overlay and backdrop
///
/// Like shadcn/ui, DialogContent automatically includes:
/// - A semi-transparent backdrop that closes the dialog when clicked
/// - Proper portal rendering via Overlay
/// - Centering and styling
///
/// Accepts any type that implements `IntoViewIter`, including:
/// - Tuples: `(DialogHeader::new(), DialogFooter::new(...))`
/// - Arrays: `[view1, view2]`
/// - Vectors: `vec![view1, view2]`
pub struct DialogContent {
    id: ViewId,
    children: Vec<Box<dyn View>>,
}

impl DialogContent {
    /// Create new dialog content with children
    ///
    /// # Example
    /// ```rust
    /// DialogContent::new((
    ///     DialogHeader::new().title("Title"),
    ///     DialogFooter::new(Button::new("Close")),
    /// ))
    /// ```
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let id = self.id;
        let children = self.children;

        // Get the dialog context from the current scope
        let ctx = Context::get::<DialogContext>();

        if let Some(ctx) = ctx {
            let open = ctx.open;

            // Like shadcn/ui, DialogContent includes the portal and overlay
            Box::new(Overlay::with_id(id)
                .child(floem::views::Stack::new((
                    // Backdrop - semi-transparent overlay that closes dialog when clicked
                    floem::views::Empty::new()
                        .style(move |s| {
                            s.absolute()
                                .inset_0()
                                .background(peniko::Color::from_rgba8(0, 0, 0, 128))
                        })
                        .on_click_stop(move |_| {
                            open.set(false);
                        }),
                    // Content wrapper - centered modal with vertical stack for children
                    floem::views::Stack::vertical_from_iter(children)
                        .style(move |s| {
                            s.absolute()
                                .left_1_2()
                                .top_1_2()
                                .translate_x_neg_1_2()
                                .translate_y_neg_1_2()
                                .z_index(10)
                                .max_w_lg()
                                .rounded_lg()
                                .p_6()
                                .gap_4()
                                .shadow_lg()
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
                        .width_full()
                        .height_full()
                        .apply_if(!is_open, |s| s.hide())
                })))
        } else {
            // No dialog context - just render the content (for use outside Dialog)
            Box::new(floem::views::Stack::vertical_from_iter(children).style(|s| s.w_full()))
        }
    }
}

// ============================================================================
// DialogHeader
// ============================================================================

/// Dialog header with title and description
pub struct DialogHeader {
    id: ViewId,
    title: Option<String>,
    description: Option<String>,
}

impl DialogHeader {
    /// Create a new dialog header
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            title: None,
            description: None,
        }
    }

    /// Set the dialog title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the dialog description
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        // shadcn/ui DialogHeader: flex flex-col gap-2 text-center sm:text-left
        let mut children: Vec<Box<dyn View>> = Vec::new();

        if let Some(title) = self.title {
            // shadcn/ui DialogTitle: text-lg leading-none font-semibold
            children.push(Box::new(floem::views::Label::new(title).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.text_lg() // text-lg = 18px
                        .leading_none() // leading-none
                        .font_semibold() // font-semibold
                        .color(t.foreground)
                })
            })));
        }

        if let Some(description) = self.description {
            // shadcn/ui DialogDescription: text-muted-foreground text-sm
            children.push(Box::new(floem::views::Label::new(description).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.text_sm() // text-sm = 14px
                        .color(t.muted_foreground) // text-muted-foreground
                })
            })));
        }

        Box::new(floem::views::Stack::vertical_from_iter(children).style(|s| s.gap_2())) // gap-2 = 8px
    }
}

// ============================================================================
// DialogFooter
// ============================================================================

/// Dialog footer for action buttons
pub struct DialogFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogFooter<V> {
    /// Create a new dialog footer
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        // shadcn/ui: flex flex-col-reverse gap-2 sm:flex-row sm:justify-end
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.flex().flex_row().justify_end().gap_2()),
        )
    }
}

// ============================================================================
// DialogTitle (standalone)
// ============================================================================

/// Standalone dialog title
pub struct DialogTitle<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogTitle<V> {
    /// Create a new dialog title
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.with_shadcn_theme(|s, t| s.text_lg().font_semibold().color(t.foreground))
            }),
        )
    }
}

// ============================================================================
// DialogDescription (standalone)
// ============================================================================

/// Standalone dialog description
pub struct DialogDescription<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogDescription<V> {
    /// Create a new dialog description
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.with_shadcn_theme(|s, t| s.text_sm().color(t.muted_foreground))),
        )
    }
}
