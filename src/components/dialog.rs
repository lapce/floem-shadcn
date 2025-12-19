//! Dialog component with builder-style API
//!
//! Based on shadcn/ui Dialog component for modal dialogs.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::dialog::{Dialog, DialogContent, DialogHeader, DialogFooter};
//! use floem_shadcn::components::button::Button;
//!
//! let open = RwSignal::new(false);
//!
//! // Trigger button
//! Button::new("Open Dialog").on_click_stop(move |_| open.set(true));
//!
//! // Dialog - uses a closure to build content
//! Dialog::new(open, move || DialogContent::new((
//!     DialogHeader::new()
//!         .title("Are you sure?")
//!         .description("This action cannot be undone."),
//!     DialogFooter::new((
//!         Button::new("Cancel").outline().on_click_stop(move |_| open.set(false)),
//!         Button::new("Continue").on_click_stop(move |_| {
//!             // do something
//!             open.set(false);
//!         }),
//!     )),
//! )));
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::action::{add_overlay, remove_overlay};
use floem::reactive::{Effect, RwSignal, SignalGet, SignalUpdate};
use floem::text::Weight;
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Dialog
// ============================================================================

/// Dialog (modal) builder
pub struct Dialog<F> {
    id: ViewId,
    open: RwSignal<bool>,
    content_fn: F,
}

impl<F, V> Dialog<F>
where
    F: Fn() -> V + 'static,
    V: IntoView + 'static,
{
    /// Create a new dialog with the given open signal and content builder function
    pub fn new(open: RwSignal<bool>, content_fn: F) -> Self {
        Self { id: ViewId::new(), open, content_fn }
    }
}


impl<F, V> HasViewId for Dialog<F> where
    F: Fn() -> V + 'static,
    V: IntoView + 'static,
 {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<F, V> IntoView for Dialog<F>
where
    F: Fn() -> V + 'static,
    V: IntoView + 'static,
 {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let open = self.open;
        let content_fn = self.content_fn;
        let overlay_id: RwSignal<Option<ViewId>> = RwSignal::new(None);

        // Effect to show/hide the overlay based on open signal
        Effect::new(move |_| {
            let is_open = open.get();
            let current_overlay = overlay_id.get();

            if is_open && current_overlay.is_none() {
                // Create and show the overlay
                let content = content_fn();
                let id = add_overlay(
                    dialog_overlay(open, content)
                );
                overlay_id.set(Some(id));
            } else if !is_open && current_overlay.is_some() {
                // Remove the overlay
                if let Some(id) = current_overlay {
                    remove_overlay(id);
                }
                overlay_id.set(None);
            }
        });

        // Return an empty view - the dialog is managed via overlay
        Box::new(floem::views::Empty::with_id(self.id))
    }
}

fn dialog_overlay<V: IntoView + 'static>(open: RwSignal<bool>, content: V) -> impl View {
    // Backdrop
    let backdrop = floem::views::Empty::new()
        .style(|s| {
            s.position(floem::style::Position::Absolute)
                .inset(0.0)
                .background(peniko::Color::from_rgba8(0, 0, 0, 128))
        })
        .on_click_stop(move |_| {
            open.set(false);
        });

    // Dialog container
    let dialog = floem::views::Container::new(content)
        .style(|s| s.with_shadcn_theme(|s, t| {            s.position(floem::style::Position::Absolute)
                .inset_left_pct(50.0)
                .inset_top_pct(50.0)
                // CSS transform: translate(-50%, -50%) equivalent
                .margin_left(-200.0) // Half of max-width
                .margin_top(-100.0) // Approximate
                .max_width(400.0)
                .width_full()
                .background(t.background)
                .border(1.0)
                .border_color(t.border)
                .border_radius(8.0)
                .padding(24.0)
                .flex_direction(floem::style::FlexDirection::Column)
                .gap(16.0)

        }));

    floem::views::stack((backdrop, dialog))
        .style(|s| {
            s.position(floem::style::Position::Absolute)
                .inset(0.0)
                .display(floem::style::Display::Flex)
                .items_center()
                .justify_center()
        })
}

// ============================================================================
// DialogContent
// ============================================================================

/// Dialog content container
pub struct DialogContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogContent<V> {
    /// Create new dialog content
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for DialogContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| {
                    s.width_full()
                        .flex_direction(floem::style::FlexDirection::Column)
                        .gap(16.0)
                })
        )
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
    pub fn new() -> Self { Self { id: ViewId::new(),
            title: None,
            description: None,
        }
    }

    /// Set the dialog title
    pub fn title(mut self, title: impl Into<String>) -> Self { self.title = Some(title.into());
        self
    }

    /// Set the dialog description
    pub fn description(mut self, description: impl Into<String>) -> Self { self.description = Some(description.into());
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
        let mut children: Vec<Box<dyn View>> = Vec::new();

        if let Some(title) = self.title {
            children.push(Box::new(
                floem::views::Label::new(title)
                    .style(|s| s.with_shadcn_theme(|s, t| {                        s.font_size(18.0)
                            .font_weight(Weight::SEMIBOLD)
                            .color(t.foreground)

                    }))
            ));
        }

        if let Some(description) = self.description {
            children.push(Box::new(
                floem::views::Label::new(description)
                    .style(|s| s.with_shadcn_theme(|s, t| {                        s.font_size(14.0)
                            .color(t.muted_foreground)

                    }))
            ));
        }

        Box::new(
            floem::views::v_stack_from_iter(children)
                .style(|s| s.gap(8.0))
        )
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
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
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
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| {
                    s.display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Row)
                        .justify_end()
                        .gap(8.0)
                        .margin_top(8.0)
                })
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
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
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
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.with_shadcn_theme(|s, t| {                    s.font_size(18.0)
                        .font_weight(Weight::SEMIBOLD)
                        .color(t.foreground)

                }))
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
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
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
                .style(|s| s.with_shadcn_theme(|s, t| {                    s.font_size(14.0)
                        .color(t.muted_foreground)

                }))
        )
    }
}
