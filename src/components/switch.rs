//! Switch component with builder-style API
//!
//! Based on shadcn/ui Switch component - a toggle switch like iOS.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::switch::Switch;
//!
//! let enabled = RwSignal::new(false);
//!
//! // Basic switch
//! let switch = Switch::new(enabled);
//!
//! // With label
//! let switch = Switch::new(enabled).label("Enable notifications");
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// A styled switch (toggle) builder.
///
/// Supports a label and disabled state.
pub struct Switch {
    id: ViewId,
    checked: RwSignal<bool>,
    label_text: Option<String>,
    disabled: bool,
}

impl Switch {
    /// Create a new switch with the given checked signal.
    pub fn new(checked: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            checked,
            label_text: None,
            disabled: false,
        }
    }

    /// Set the label text.
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label_text = Some(text.into());
        self
    }

    /// Set the switch as disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Build the switch view.
    pub fn build(self) -> impl IntoView {
        let checked = self.checked;
        let disabled = self.disabled;

        // Thumb (circle that moves)
        let thumb = floem::views::Empty::new().style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let is_checked = checked.get();
                let translate_x = if is_checked { 14.0 } else { 0.0 };
                s.size_4()
                    .rounded_full()
                    .background(t.background)
                    .position(floem::style::Position::Absolute)
                    .inset_left(translate_x)
                    .transition(
                        floem::style::InsetLeft,
                        floem::style::Transition::linear(millis(150)),
                    )
            })
        });

        // Track (background)
        let track = floem::views::Container::new(thumb).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let is_checked = checked.get();
                s.height(18.0)
                    .w_8()
                    .flex_shrink(0.0)
                    .rounded_full()
                    .border_1()
                    .border_color(peniko::Color::TRANSPARENT)
                    .box_shadow_blur(2.0)
                    .box_shadow_color(peniko::Color::from_rgba8(0, 0, 0, 25))
                    .position(floem::style::Position::Relative)
                    .transition(
                        floem::style::Background,
                        floem::style::Transition::linear(millis(150)),
                    )
                    .apply_if(is_checked, |s| s.background(t.primary))
                    .apply_if(!is_checked, |s| s.background(t.input))
                    .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                    .apply_if(!disabled, |s| s.cursor(CursorStyle::Pointer))
            })
        });

        let track = if !disabled {
            track
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    checked.update(|c| *c = !*c);
                })
                .into_any()
        } else {
            track.into_any()
        };

        // With or without label
        if let Some(label_text) = self.label_text {
            let label_view = floem::views::Label::new(label_text).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    s.text_sm()
                        .font_medium()
                        .line_height(1.0)
                        .color(if disabled {
                            t.muted_foreground
                        } else {
                            t.foreground
                        })
                        .cursor(if disabled {
                            CursorStyle::Default
                        } else {
                            CursorStyle::Pointer
                        })
                })
            });

            let label_view = if !disabled {
                label_view
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        checked.update(|c| *c = !*c);
                    })
                    .into_any()
            } else {
                label_view.into_any()
            };

            floem::views::Stack::horizontal((track, label_view))
                .style(|s| s.gap(8.0).items_center())
                .into_any()
        } else {
            track
        }
    }
}

impl HasViewId for Switch {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Switch {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

fn millis(ms: u64) -> std::time::Duration {
    std::time::Duration::from_millis(ms)
}
