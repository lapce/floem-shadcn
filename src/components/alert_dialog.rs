//! AlertDialog component with builder-style API
//!
//! Based on shadcn/ui AlertDialog - a modal dialog for important confirmations.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::alert_dialog::*;
//!
//! let is_open = RwSignal::new(false);
//!
//! AlertDialog::new(is_open)
//!     .trigger("Delete Account")
//!     .title("Are you absolutely sure?")
//!     .description("This action cannot be undone.")
//!     .cancel("Cancel")
//!     .action("Yes, delete account", || {
//!         // Handle deletion
//!     });
//! ```

use floem::prelude::*;
use floem::views::{Decorators, Overlay};
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// AlertDialog
// ============================================================================

/// Modal dialog for important confirmations
pub struct AlertDialog {
    id: ViewId,
    is_open: RwSignal<bool>,
    trigger_text: String,
    title: String,
    description: String,
    cancel_text: String,
    action_text: String,
    on_action: Option<Box<dyn Fn() + 'static>>,
    destructive: bool,
}

impl AlertDialog {
    /// Create a new alert dialog
    pub fn new(is_open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            is_open,
            trigger_text: "Open".to_string(),
            title: "Are you sure?".to_string(),
            description: String::new(),
            cancel_text: "Cancel".to_string(),
            action_text: "Continue".to_string(),
            on_action: None,
            destructive: false,
        }
    }

    /// Set the trigger button text
    pub fn trigger(mut self, text: impl Into<String>) -> Self {
        self.trigger_text = text.into();
        self
    }

    /// Set the dialog title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the dialog description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set the cancel button text
    pub fn cancel(mut self, text: impl Into<String>) -> Self {
        self.cancel_text = text.into();
        self
    }

    /// Set the action button text and handler
    pub fn action(mut self, text: impl Into<String>, handler: impl Fn() + 'static) -> Self {
        self.action_text = text.into();
        self.on_action = Some(Box::new(handler));
        self
    }

    /// Make the action button destructive (red)
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

impl HasViewId for AlertDialog {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for AlertDialog {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;
        let trigger_text = self.trigger_text;
        let title = self.title;
        let description = self.description;
        let cancel_text = self.cancel_text;
        let action_text = self.action_text;
        let on_action = self.on_action;
        let destructive = self.destructive;

        // Trigger button
        let trigger = floem::views::Label::new(trigger_text)
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.padding_left(16.0)
                        .padding_right(16.0)
                        .padding_top(8.0)
                        .padding_bottom(8.0)
                        .font_size(14.0)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .background(t.primary)
                        .color(t.primary_foreground)
                        .border_radius(t.radius)
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.primary.with_alpha(0.9)))
                })
            })
            .on_click_stop(move |_| {
                is_open.set(true);
            });

        // Title
        let title_view = floem::views::Label::new(title).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.text_lg()
                    .font_semibold()
                    .color(t.foreground)
            })
        });

        // Description
        let desc_view = if !description.is_empty() {
            floem::views::Label::new(description)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.text_sm().color(t.muted_foreground).margin_top(8.0)
                    })
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        // Cancel button
        let cancel_btn = floem::views::Label::new(cancel_text)
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.padding_left(16.0)
                        .padding_right(16.0)
                        .padding_top(8.0)
                        .padding_bottom(8.0)
                        .font_size(14.0)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .background(t.secondary)
                        .color(t.secondary_foreground)
                        .border_radius(t.radius)
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.secondary.with_alpha(0.8)))
                })
            })
            .on_click_stop(move |_| {
                is_open.set(false);
            });

        // Action button
        let action_btn = floem::views::Label::new(action_text)
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let bg = if destructive {
                        t.destructive
                    } else {
                        t.primary
                    };
                    let fg = if destructive {
                        t.destructive_foreground
                    } else {
                        t.primary_foreground
                    };

                    s.padding_left(16.0)
                        .padding_right(16.0)
                        .padding_top(8.0)
                        .padding_bottom(8.0)
                        .font_size(14.0)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .background(bg)
                        .color(fg)
                        .border_radius(t.radius)
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(bg.with_alpha(0.9)))
                })
            })
            .on_click_stop(move |_| {
                if let Some(ref handler) = on_action {
                    handler();
                }
                is_open.set(false);
            });

        // Footer with buttons
        let footer =
            floem::views::h_stack((cancel_btn, action_btn)).style(|s| s.gap(8.0).justify_end());

        // Dialog content in Overlay - escapes parent clipping
        let dialog_overlay = Overlay::new(
            floem::views::stack((
                // Backdrop - semi-transparent, doesn't close on click for alert dialogs
                floem::views::Empty::new()
                    .style(move |s| {
                        s.absolute()
                            .inset_0()
                            .background(peniko::Color::from_rgba8(0, 0, 0, 128))
                    })
                    .on_click_stop(move |_| {
                        // Don't close on backdrop click for alert dialogs
                    }),
                // Content wrapper - centered modal
                floem::views::v_stack((title_view, desc_view, footer))
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
                let open = is_open.get();
                s.fixed()
                    .inset_0()
                    .width_full()
                    .height_full()
                    .apply_if(!open, |s| s.hide())
            }),
        );

        Box::new(floem::views::stack((trigger, dialog_overlay)))
    }
}

// ============================================================================
// AlertDialogTrigger
// ============================================================================

/// Standalone trigger for alert dialog
pub struct AlertDialogTrigger<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}

impl<V: IntoView + 'static> AlertDialogTrigger<V> {
    /// Create a new trigger
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self { id: ViewId::new(), child, is_open }
    }
}

impl<V: IntoView + 'static> HasViewId for AlertDialogTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for AlertDialogTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;

        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_click_stop(move |_| {
                    is_open.set(true);
                }),
        )
    }
}

// ============================================================================
// AlertDialogContent
// ============================================================================

/// Standalone content for alert dialog
pub struct AlertDialogContent<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}

impl<V: IntoView + 'static> AlertDialogContent<V> {
    /// Create new content
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self { id: ViewId::new(), child, is_open }
    }
}

impl<V: IntoView + 'static> HasViewId for AlertDialogContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for AlertDialogContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let is_open = self.is_open;
        let child = self.child;

        // Alert dialog content in Overlay - escapes parent clipping
        Box::new(Overlay::new(
            floem::views::stack((
                // Backdrop - semi-transparent, doesn't close on click for alert dialogs
                floem::views::Empty::new()
                    .style(move |s| {
                        s.absolute()
                            .inset_0()
                            .background(peniko::Color::from_rgba8(0, 0, 0, 128))
                    })
                    .on_click_stop(move |_| {
                        // Don't close on backdrop click for alert dialogs
                    }),
                // Content wrapper - centered modal
                floem::views::Container::new(child)
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
                let open = is_open.get();
                s.fixed()
                    .inset_0()
                    .width_full()
                    .height_full()
                    .apply_if(!open, |s| s.hide())
            }),
        ))
    }
}

// ============================================================================
// AlertDialogHeader
// ============================================================================

/// Header section of alert dialog
pub struct AlertDialogHeader<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> AlertDialogHeader<V> {
    /// Create a new header
    pub fn new(child: V) -> Self {
        Self { id: ViewId::new(), child }
    }
}

impl<V: IntoView + 'static> HasViewId for AlertDialogHeader<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for AlertDialogHeader<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Column)
                .margin_bottom(16.0)
        }))
    }
}

// ============================================================================
// AlertDialogFooter
// ============================================================================

/// Footer section with action buttons
pub struct AlertDialogFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> AlertDialogFooter<V> {
    /// Create a new footer
    pub fn new(child: V) -> Self {
        Self { id: ViewId::new(), child }
    }
}

impl<V: IntoView + 'static> HasViewId for AlertDialogFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for AlertDialogFooter<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Row)
                .justify_end()
                .gap(8.0)
        }))
    }
}

// ============================================================================
// AlertDialogTitle
// ============================================================================

/// Title for alert dialog
pub struct AlertDialogTitle {
    id: ViewId,
    text: String,
}

impl AlertDialogTitle {
    /// Create a new title
    pub fn new(text: impl Into<String>) -> Self {
        Self { id: ViewId::new(), text: text.into() }
    }
}

impl HasViewId for AlertDialogTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for AlertDialogTitle {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;

        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(18.0)
                    .font_weight(floem::text::Weight::SEMIBOLD)
                    .color(t.foreground)
            })
        }))
    }
}

// ============================================================================
// AlertDialogDescription
// ============================================================================

/// Description for alert dialog
pub struct AlertDialogDescription {
    id: ViewId,
    text: String,
}

impl AlertDialogDescription {
    /// Create a new description
    pub fn new(text: impl Into<String>) -> Self {
        Self { id: ViewId::new(), text: text.into() }
    }
}

impl HasViewId for AlertDialogDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for AlertDialogDescription {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;

        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(14.0).color(t.muted_foreground).margin_top(8.0)
            })
        }))
    }
}

// ============================================================================
// AlertDialogAction
// ============================================================================

/// Action button for alert dialog
pub struct AlertDialogAction {
    id: ViewId,
    text: String,
    destructive: bool,
    on_click: Option<Box<dyn Fn() + 'static>>,
    is_open: Option<RwSignal<bool>>,
}

impl AlertDialogAction {
    /// Create a new action button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            destructive: false,
            on_click: None,
            is_open: None,
        }
    }

    /// Make destructive (red)
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    /// Set click handler
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Connect to dialog state (to close on click)
    pub fn dialog(mut self, is_open: RwSignal<bool>) -> Self {
        self.is_open = Some(is_open);
        self
    }
}

impl HasViewId for AlertDialogAction {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for AlertDialogAction {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        let destructive = self.destructive;
        let on_click = self.on_click;
        let is_open = self.is_open;

        Box::new(
            floem::views::Label::new(text)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let bg = if destructive {
                            t.destructive
                        } else {
                            t.primary
                        };
                        let fg = if destructive {
                            t.destructive_foreground
                        } else {
                            t.primary_foreground
                        };

                        s.padding_left(16.0)
                            .padding_right(16.0)
                            .padding_top(8.0)
                            .padding_bottom(8.0)
                            .font_size(14.0)
                            .font_weight(floem::text::Weight::MEDIUM)
                            .background(bg)
                            .color(fg)
                            .border_radius(t.radius)
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.background(bg.with_alpha(0.9)))
                    })
                })
                .on_click_stop(move |_| {
                    if let Some(ref handler) = on_click {
                        handler();
                    }
                    if let Some(signal) = is_open {
                        signal.set(false);
                    }
                }),
        )
    }
}

// ============================================================================
// AlertDialogCancel
// ============================================================================

/// Cancel button for alert dialog
pub struct AlertDialogCancel {
    id: ViewId,
    text: String,
    is_open: Option<RwSignal<bool>>,
}

impl AlertDialogCancel {
    /// Create a new cancel button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            is_open: None,
        }
    }

    /// Connect to dialog state
    pub fn dialog(mut self, is_open: RwSignal<bool>) -> Self {
        self.is_open = Some(is_open);
        self
    }
}

impl HasViewId for AlertDialogCancel {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for AlertDialogCancel {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        let is_open = self.is_open;

        Box::new(
            floem::views::Label::new(text)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.padding_left(16.0)
                            .padding_right(16.0)
                            .padding_top(8.0)
                            .padding_bottom(8.0)
                            .font_size(14.0)
                            .font_weight(floem::text::Weight::MEDIUM)
                            .background(t.secondary)
                            .color(t.secondary_foreground)
                            .border_radius(t.radius)
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.background(t.secondary.with_alpha(0.8)))
                    })
                })
                .on_click_stop(move |_| {
                    if let Some(signal) = is_open {
                        signal.set(false);
                    }
                }),
        )
    }
}
