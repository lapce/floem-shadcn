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
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::{Decorators, Overlay};
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// A modal confirmation dialog with a trigger, title, description, and action buttons.
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
    /// Create a new AlertDialog controlled by the given open signal.
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

    pub fn trigger(mut self, text: impl Into<String>) -> Self {
        self.trigger_text = text.into();
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
    pub fn cancel(mut self, text: impl Into<String>) -> Self {
        self.cancel_text = text.into();
        self
    }
    pub fn action(mut self, text: impl Into<String>, handler: impl Fn() + 'static) -> Self {
        self.action_text = text.into();
        self.on_action = Some(Box::new(handler));
        self
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
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

        let trigger = floem::views::Label::new(trigger_text)
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.px_4()
                        .py_2()
                        .text_sm()
                        .font_medium()
                        .background(t.primary)
                        .color(t.primary_foreground)
                        .rounded_md()
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.primary.with_alpha(0.9)))
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                is_open.set(true);
            });

        let title_view = floem::views::Label::new(title).style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_lg().font_semibold().color(t.foreground))
        });

        let desc_view = if !description.is_empty() {
            floem::views::Label::new(description)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| s.text_sm().color(t.muted_foreground).mt_2())
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        let cancel_btn = floem::views::Label::new(cancel_text)
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.px_4()
                        .py_2()
                        .text_sm()
                        .font_medium()
                        .background(t.secondary)
                        .color(t.secondary_foreground)
                        .rounded_md()
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.secondary.with_alpha(0.8)))
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                is_open.set(false);
            });

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
                    s.px_4()
                        .py_2()
                        .text_sm()
                        .font_medium()
                        .background(bg)
                        .color(fg)
                        .rounded_md()
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(bg.with_alpha(0.9)))
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                if let Some(ref handler) = on_action {
                    handler();
                }
                is_open.set(false);
            });

        let footer = floem::views::Stack::horizontal((cancel_btn, action_btn))
            .style(|s| s.gap_2().justify_end());

        let dialog_overlay = Overlay::new(
            floem::views::Stack::new((
                floem::views::Empty::new()
                    .style(move |s| {
                        s.absolute()
                            .inset_0()
                            .background(peniko::Color::from_rgba8(0, 0, 0, 128))
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {}),
                floem::views::Stack::vertical((title_view, desc_view, footer))
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
                let open = is_open.get();
                s.fixed()
                    .inset_0()
                    .w_full()
                    .h_full()
                    .apply_if(!open, |s| s.hide())
            }),
        );

        Box::new(floem::views::Stack::new((trigger, dialog_overlay)))
    }
}

/// When clicked, opens the nearest parent AlertDialog.
pub struct AlertDialogTrigger<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}
impl<V: IntoView + 'static> AlertDialogTrigger<V> {
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child,
            is_open,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for AlertDialogTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for AlertDialogTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let is_open = self.is_open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.cursor(CursorStyle::Pointer))
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    is_open.set(true);
                }),
        )
    }
}

/// The content (overlay) of an AlertDialog.
pub struct AlertDialogContent<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}
impl<V: IntoView + 'static> AlertDialogContent<V> {
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child,
            is_open,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for AlertDialogContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for AlertDialogContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let is_open = self.is_open;
        let child = self.child;
        Box::new(Overlay::new(
            floem::views::Stack::new((
                floem::views::Empty::new()
                    .style(move |s| {
                        s.absolute()
                            .inset_0()
                            .background(peniko::Color::from_rgba8(0, 0, 0, 128))
                    })
                    .on_event_stop(floem::event::listener::Click, move |_, _| {}),
                floem::views::Container::new(child)
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
                let open = is_open.get();
                s.fixed()
                    .inset_0()
                    .w_full()
                    .h_full()
                    .apply_if(!open, |s| s.hide())
            }),
        ))
    }
}

/// Header for an AlertDialog.
pub struct AlertDialogHeader<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> AlertDialogHeader<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for AlertDialogHeader<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for AlertDialogHeader<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| s.flex_col().mb_4()),
        )
    }
}

/// Footer for an AlertDialog.
pub struct AlertDialogFooter<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> AlertDialogFooter<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for AlertDialogFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for AlertDialogFooter<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.flex_row().justify_end().gap_2()),
        )
    }
}

/// Title text for an AlertDialog.
pub struct AlertDialogTitle {
    id: ViewId,
    text: String,
}
impl AlertDialogTitle {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for AlertDialogTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for AlertDialogTitle {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_lg().font_semibold().color(t.foreground))
        }))
    }
}

/// Description text for an AlertDialog.
pub struct AlertDialogDescription {
    id: ViewId,
    text: String,
}
impl AlertDialogDescription {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for AlertDialogDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for AlertDialogDescription {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| s.text_sm().color(t.muted_foreground).mt_2())
        }))
    }
}

/// Action button of an AlertDialog.
pub struct AlertDialogAction {
    id: ViewId,
    text: String,
    destructive: bool,
    on_click: Option<Box<dyn Fn() + 'static>>,
    is_open: Option<RwSignal<bool>>,
}
impl AlertDialogAction {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            destructive: false,
            on_click: None,
            is_open: None,
        }
    }
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
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
                        s.px_4()
                            .py_2()
                            .text_sm()
                            .font_medium()
                            .background(bg)
                            .color(fg)
                            .rounded_md()
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.background(bg.with_alpha(0.9)))
                    })
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
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

/// Cancel button of an AlertDialog.
pub struct AlertDialogCancel {
    id: ViewId,
    text: String,
    is_open: Option<RwSignal<bool>>,
}
impl AlertDialogCancel {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            is_open: None,
        }
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        let is_open = self.is_open;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.px_4()
                            .py_2()
                            .text_sm()
                            .font_medium()
                            .background(t.secondary)
                            .color(t.secondary_foreground)
                            .rounded_md()
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.background(t.secondary.with_alpha(0.8)))
                    })
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    if let Some(signal) = is_open {
                        signal.set(false);
                    }
                }),
        )
    }
}
