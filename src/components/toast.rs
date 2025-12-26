//! Toast component with builder-style API
//!
//! Based on shadcn/ui Toast - a notification popup system.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::toast::*;
//!
//! // Create a toast state
//! let toasts = RwSignal::new(Vec::<ToastData>::new());
//!
//! // Add a toast
//! toasts.update(|t| t.push(ToastData::new("Success!", "Your changes have been saved.")));
//!
//! // Render toast container
//! ToastContainer::new(toasts);
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::{Decorators, Overlay};
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// Toast variant for styling
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Destructive,
}

/// Data for a single toast notification
#[derive(Clone)]
pub struct ToastData {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
}

impl ToastData {
    /// Create a new toast with title
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: rand_id(),
            title: title.into(),
            description: Some(description.into()),
            variant: ToastVariant::Default,
        }
    }

    /// Create a toast with just a title
    pub fn title_only(title: impl Into<String>) -> Self {
        Self {
            id: rand_id(),
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
        }
    }

    /// Set the variant
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Make it a success toast
    pub fn success(mut self) -> Self {
        self.variant = ToastVariant::Success;
        self
    }

    /// Make it a destructive/error toast
    pub fn destructive(mut self) -> Self {
        self.variant = ToastVariant::Destructive;
        self
    }
}

/// Simple random ID generator
fn rand_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

// ============================================================================
// ToastContainer
// ============================================================================

/// Container that renders all active toasts
pub struct ToastContainer {
    id: ViewId,
    toasts: RwSignal<Vec<ToastData>>,
}

impl ToastContainer {
    /// Create a new toast container
    pub fn new(toasts: RwSignal<Vec<ToastData>>) -> Self {
        Self {
            id: ViewId::new(),
            toasts,
        }
    }
}

impl HasViewId for Toast {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl HasViewId for ToastContainer {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ToastContainer {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let toasts = self.toasts;

        // Toast list positioned at bottom-right
        let toast_list = floem::views::dyn_container(
            move || toasts.get(),
            move |toast_list| {
                let views: Vec<Box<dyn View>> = toast_list
                    .iter()
                    .map(|toast| {
                        let toast_id = toast.id;
                        let toasts_signal = toasts;
                        Toast::new(toast.clone())
                            .on_close(move || {
                                toasts_signal.update(|t| t.retain(|x| x.id != toast_id));
                            })
                            .into_view()
                    })
                    .collect();

                floem::views::Stack::vertical_from_iter(views)
                    .style(|s| s.gap(8.0))
                    .into_any()
            },
        )
        .style(|s| {
            s.absolute()
                .inset_bottom(16.0)
                .inset_right(16.0)
                .flex_col()
                .gap(8.0)
        });

        // Use Overlay with fixed positioning
        Box::new(Overlay::new(toast_list).style(move |s| {
            let has_toasts = !toasts.get().is_empty();
            s.fixed()
                .inset_0()
                .width_full()
                .height_full()
                .pointer_events_none()
                .apply_if(!has_toasts, |s| s.hide())
        }))
    }
}

// ============================================================================
// Toast
// ============================================================================

/// Individual toast notification
pub struct Toast {
    id: ViewId,
    data: ToastData,
    on_close: Option<Box<dyn Fn() + 'static>>,
}

impl Toast {
    /// Create a new toast from data
    pub fn new(data: ToastData) -> Self {
        Self {
            id: ViewId::new(),
            data,
            on_close: None,
        }
    }

    /// Set close handler
    pub fn on_close(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }
}

impl IntoView for Toast {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let title = self.data.title.clone();
        let description = self.data.description.clone();
        let variant = self.data.variant;
        let on_close = self.on_close;

        // Title
        let title_view = floem::views::Label::new(title).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(14.0)
                    .font_weight(floem::text::Weight::SEMIBOLD)
                    .color(match variant {
                        ToastVariant::Default => t.foreground,
                        ToastVariant::Success => t.foreground,
                        ToastVariant::Destructive => t.destructive_foreground,
                    })
            })
        });

        // Description (optional)
        let desc_view = if let Some(desc) = description {
            floem::views::Label::new(desc)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.font_size(13.0)
                            .color(match variant {
                                ToastVariant::Default => t.muted_foreground,
                                ToastVariant::Success => t.muted_foreground,
                                ToastVariant::Destructive => {
                                    t.destructive_foreground.with_alpha(0.9)
                                }
                            })
                            .margin_top(2.0)
                    })
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        // Close button
        let close_btn = floem::views::Label::new("×").style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(18.0)
                    .color(t.muted_foreground)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.color(t.foreground))
            })
        });

        let close_btn = if let Some(handler) = on_close {
            close_btn.on_click_stop(move |_| handler()).into_any()
        } else {
            close_btn.into_any()
        };

        // Content stack
        let content =
            floem::views::Stack::vertical((title_view, desc_view)).style(|s| s.flex_grow(1.0));

        Box::new(
            floem::views::Stack::horizontal((content, close_btn)).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let base = s
                        .min_width(300.0)
                        .max_width(420.0)
                        .padding(16.0)
                        .border(1.0)
                        .border_radius(t.radius)
                        .box_shadow_blur(8.0)
                        .box_shadow_color(t.foreground.with_alpha(0.1))
                        .items_start()
                        .gap(8.0)
                        .pointer_events_auto(); // Enable clicks on toast (parent overlay has pointer-events: none)
                    match variant {
                        ToastVariant::Default | ToastVariant::Success => {
                            base.background(t.background).border_color(t.border)
                        }
                        ToastVariant::Destructive => {
                            base.background(t.destructive).border_color(t.destructive)
                        }
                    }
                })
            }),
        )
    }
}

// ============================================================================
// ToastAction
// ============================================================================

/// Action button for toasts
pub struct ToastAction {
    id: ViewId,
    text: String,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl ToastAction {
    /// Create a new toast action button
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            on_click: None,
        }
    }

    /// Set click handler
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl HasViewId for ToastAction {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ToastAction {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        let on_click = self.on_click;

        let btn = floem::views::Label::new(text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(13.0)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .color(t.foreground)
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.background(t.accent))
            })
        });

        if let Some(handler) = on_click {
            Box::new(btn.on_click_stop(move |_| handler()))
        } else {
            Box::new(btn)
        }
    }
}

// ============================================================================
// Helper function to add toasts
// ============================================================================

/// Helper to push a toast to the toast state
pub fn push_toast(toasts: RwSignal<Vec<ToastData>>, toast: ToastData) {
    toasts.update(|t| t.push(toast));
}

/// Helper to remove a toast by ID
pub fn remove_toast(toasts: RwSignal<Vec<ToastData>>, id: u64) {
    toasts.update(|t| t.retain(|x| x.id != id));
}

/// Helper to clear all toasts
pub fn clear_toasts(toasts: RwSignal<Vec<ToastData>>) {
    toasts.update(|t| t.clear());
}
