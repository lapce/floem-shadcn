//! Slider component with builder-style API
//!
//! Based on shadcn/ui Slider - a range input for selecting values.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::slider::Slider;
//!
//! let value = RwSignal::new(50.0);
//!
//! // Basic slider (0-100)
//! let slider = Slider::new(value);
//!
//! // Custom range
//! let slider = Slider::new(value).min(0.0).max(200.0);
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

/// A styled slider builder
pub struct Slider {
    id: ViewId,
    value: RwSignal<f64>,
    min: f64,
    max: f64,
    step: f64,
    disabled: bool,
}

impl Slider {
    /// Create a new slider with the given value signal
    pub fn new(value: RwSignal<f64>) -> Self {
        Self {
            id: ViewId::new(),
            value,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
        }
    }

    /// Set the minimum value (default: 0)
    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    /// Set the maximum value (default: 100)
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// Set the step value (default: 1)
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    /// Set the slider as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Build the slider view
    pub fn build(self) -> impl IntoView {
        let value = self.value;
        let min = self.min;
        let max = self.max;
        let disabled = self.disabled;

        // Track (background)
        let track = floem::views::Container::new(
            // Filled portion
            floem::views::Empty::new()
                .style(move |s| s.with_shadcn_theme(move |s, t| {
                    let percent = ((value.get() - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                    s.height_full()
                        .width_pct(percent)
                        .background(t.primary)
                        .border_radius(4.0)
                }))
        )
        .style(move |s| s.with_shadcn_theme(move |s, t| {
            s.width_full()
                .height(6.0)
                .background(t.muted)
                .border_radius(4.0)
                .position(floem::style::Position::Relative)
        }));

        // Thumb
        let thumb = floem::views::Empty::new()
            .style(move |s| s.with_shadcn_theme(move |s, t| {
                let percent = ((value.get() - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                s.width(16.0)
                    .height(16.0)
                    .border_radius(8.0)
                    .background(t.background)
                    .border(2.0)
                    .border_color(t.primary)
                    .position(floem::style::Position::Absolute)
                    .inset_top(-5.0)
                    .inset_left_pct(percent)
                    .margin_left(-8.0) // Center the thumb
                    .cursor(if disabled { CursorStyle::Default } else { CursorStyle::Pointer })
            }));

        // Container with interaction
        floem::views::Container::new(
            floem::views::stack((track, thumb))
                .style(|s| s.width_full().position(floem::style::Position::Relative))
        )
        .style(move |s| {
            s.width_full()
                .height(16.0)
                .cursor(if disabled { CursorStyle::Default } else { CursorStyle::Pointer })
                .padding_left(8.0)
                .padding_right(8.0)
        })
        .on_click_stop(move |_| {
            // For now, clicking toggles between min and max as a simple interaction
            // A full slider implementation would track drag events
            if !disabled {
                value.update(|v| {
                    let mid = (min + max) / 2.0;
                    if *v < mid {
                        *v = max;
                    } else {
                        *v = min;
                    }
                });
            }
        })
    }
}

impl HasViewId for Slider {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Slider {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
