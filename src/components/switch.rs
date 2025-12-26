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

        // shadcn/ui Switch (v4 new-york):
        // Root: h-[1.15rem] w-8 shrink-0 rounded-full border border-transparent shadow-xs
        //       data-[state=checked]:bg-primary data-[state=unchecked]:bg-input
        // Thumb: size-4 rounded-full bg-background
        //        data-[state=checked]:translate-x-[calc(100%-2px)] data-[state=unchecked]:translate-x-0

        // The thumb (circle that moves)
        // size-4 = 16px
        let thumb = floem::views::Empty::new().style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let is_checked = checked.get();
                // Track is 32px (w-8), thumb is 16px (size-4)
                // Checked: translate-x-[calc(100%-2px)] = 32 - 16 - 2 = 14px
                // Unchecked: translate-x-0 = 0px
                let translate_x = if is_checked { 14.0 } else { 0.0 };
                s.size_4() // size-4 = 16px
                    .rounded_full() // rounded-full
                    .background(t.background) // bg-background
                    .position(floem::style::Position::Absolute)
                    .inset_left(translate_x)
                    .transition(
                        floem::style::InsetLeft,
                        floem::style::Transition::linear(millis(150)),
                    )
            })
        });

        // The track (background)
        // h-[1.15rem] ≈ 18px, w-8 = 32px
        let track = floem::views::Container::new(thumb).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let is_checked = checked.get();
                s.height(18.0) // h-[1.15rem] ≈ 18px
                    .w_8() // w-8 = 32px
                    .flex_shrink(0.0) // shrink-0
                    .rounded_full() // rounded-full
                    .border_1() // border
                    .border_color(peniko::Color::TRANSPARENT) // border-transparent
                    .shadow_sm() // shadow-xs
                    .position(floem::style::Position::Relative)
                    .transition(
                        floem::style::Background,
                        floem::style::Transition::linear(millis(150)),
                    )
                    // Checked: bg-primary, Unchecked: bg-input
                    .apply_if(is_checked, |s| s.background(t.primary))
                    .apply_if(!is_checked, |s| s.background(t.input))
                    // Disabled state
                    .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                    .apply_if(!disabled, |s| s.cursor(CursorStyle::Pointer))
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
                    // Label: text-sm font-medium leading-none
                    s.text_sm()
                        .font_medium()
                        .leading_none()
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

            floem::views::Stack::horizontal((track, label_view))
                .style(|s| s.gap_2().items_center()) // gap-2 = 8px
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
