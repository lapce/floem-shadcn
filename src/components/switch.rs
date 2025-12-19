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
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

/// A styled switch (toggle) builder
pub struct Switch {
    id: ViewId,
    checked: RwSignal<bool>,
    label_text: Option<String>,
    disabled: bool,
}

impl Switch {
    /// Create a new switch with the given checked signal
    pub fn new(checked: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            checked,
            label_text: None,
            disabled: false,
        }
    }

    /// Set the label text
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label_text = Some(text.into());
        self
    }

    /// Set the switch as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Build the switch view
    pub fn build(self) -> impl IntoView {
        let checked = self.checked;
        let disabled = self.disabled;

        // The thumb (circle that moves)
        let thumb = floem::views::Empty::new().style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let is_checked = checked.get();
                let translate_x = if is_checked { 18.0 } else { 2.0 };
                s.width(16.0)
                    .height(16.0)
                    .border_radius(8.0)
                    .background(t.background)
                    .position(floem::style::Position::Absolute)
                    .inset_top(2.0)
                    .inset_left(translate_x)
                    .transition(
                        floem::style::InsetLeft,
                        floem::style::Transition::linear(millis(150)),
                    )
            })
        });

        // The track (background)
        let track = floem::views::Container::new(thumb).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let is_checked = checked.get();
                s.width(36.0)
                    .height(20.0)
                    .border_radius(10.0)
                    .position(floem::style::Position::Relative)
                    .cursor(if disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    })
                    .transition(
                        floem::style::Background,
                        floem::style::Transition::linear(millis(150)),
                    )
                    .apply_if(is_checked, |s| s.background(t.primary))
                    .apply_if(!is_checked, |s| s.background(t.input))
            })
        });

        let track = if !disabled {
            track
                .on_click_stop(move |_| {
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
                    s.font_size(14.0)
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
                    .on_click_stop(move |_| {
                        checked.update(|c| *c = !*c);
                    })
                    .into_any()
            } else {
                label_view.into_any()
            };

            floem::views::h_stack((track, label_view))
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

fn millis(ms: u64) -> std::time::Duration {
    std::time::Duration::from_millis(ms)
}
