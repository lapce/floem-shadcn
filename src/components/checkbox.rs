//! Checkbox component with builder-style API
//!
//! Based on shadcn/ui Checkbox component.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::checkbox::Checkbox;
//!
//! let checked = RwSignal::new(false);
//!
//! // Basic checkbox
//! let checkbox = Checkbox::new(checked);
//!
//! // With label
//! let checkbox = Checkbox::new(checked).label("Accept terms");
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

/// A styled checkbox builder
pub struct Checkbox {
    id: ViewId,
    checked: RwSignal<bool>,
    label_text: Option<String>,
    disabled: bool,
}

impl Checkbox {
    /// Create a new checkbox with the given checked signal
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

    /// Set the checkbox as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Build the checkbox view
    pub fn build(self) -> impl IntoView {
        let checked = self.checked;
        let disabled = self.disabled;

        // The checkbox box
        let checkbox_box = floem::views::Container::new(
            // Checkmark (only visible when checked)
            floem::views::svg(|| CHECKMARK_SVG.to_string()).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let is_checked = checked.get();
                    s.width(12.0)
                        .height(12.0)
                        .color(t.primary_foreground)
                        .apply_if(!is_checked, |s| s.display(floem::style::Display::None))
                })
            }),
        )
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let is_checked = checked.get();
                s.width(16.0)
                    .height(16.0)
                    .border_radius(4.0)
                    .border(1.0)
                    .display(floem::style::Display::Flex)
                    .items_center()
                    .justify_center()
                    .cursor(if disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    })
                    .transition(
                        floem::style::Background,
                        floem::style::Transition::linear(millis(100)),
                    )
                    .apply_if(is_checked, |s| {
                        s.background(t.primary).border_color(t.primary)
                    })
                    .apply_if(!is_checked, |s| {
                        s.background(t.background).border_color(t.input)
                    })
                    .hover(move |s| {
                        if !disabled && !is_checked {
                            s.border_color(t.primary)
                        } else {
                            s
                        }
                    })
            })
        });

        let checkbox_box = if !disabled {
            checkbox_box
                .on_click_stop(move |_| {
                    checked.update(|c| *c = !*c);
                })
                .into_any()
        } else {
            checkbox_box.into_any()
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

            floem::views::h_stack((checkbox_box, label_view))
                .style(|s| s.gap(8.0).items_center())
                .into_any()
        } else {
            checkbox_box
        }
    }
}

impl HasViewId for Checkbox {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Checkbox {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// Simple checkmark SVG path
const CHECKMARK_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>"#;

fn millis(ms: u64) -> std::time::Duration {
    std::time::Duration::from_millis(ms)
}
