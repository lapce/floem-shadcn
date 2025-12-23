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
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

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

        // shadcn/ui Slider (v4 new-york):
        // Root: relative flex w-full touch-none items-center select-none
        // Track: bg-muted relative grow overflow-hidden rounded-full h-1.5 (6px)
        // Range: bg-primary absolute h-full
        // Thumb: size-4 shrink-0 rounded-full border border-primary bg-white shadow-sm

        // Track (background)
        let track = floem::views::Container::new(
            // Range (filled portion) - bg-primary absolute h-full
            floem::views::Empty::new().style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let percent = ((value.get() - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                    s.rounded_full()
                        .height_full()
                        .width_pct(percent)
                        .background(t.primary) // bg-primary
                })
            }),
        )
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                // Track: bg-muted relative grow rounded-full h-1.5
                s.rounded_full()
                    .width_full()
                    .height(6.0) // h-1.5 = 6px
                    .background(t.muted) // bg-muted
                    .position(floem::style::Position::Relative)
            })
        });

        // Thumb: size-4 shrink-0 rounded-full border border-primary bg-white shadow-sm
        let thumb = floem::views::Empty::new().style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let percent = ((value.get() - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                s.size_4() // size-4 = 16px
                    .flex_shrink(0.0) // shrink-0
                    .rounded_full() // rounded-full
                    .background(peniko::Color::WHITE) // bg-white (always white, not theme background)
                    .border_1() // border (1px)
                    .border_color(t.primary) // border-primary
                    .shadow_sm() // shadow-sm
                    .position(floem::style::Position::Absolute)
                    .inset_top(-5.0) // center vertically: (16 - 6) / 2 = 5
                    .inset_left_pct(percent)
                    .margin_left(-8.0) // Center the thumb horizontally
                    .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                    .apply_if(!disabled, |s| s.cursor(CursorStyle::Pointer))
            })
        });

        // Container with interaction
        floem::views::Container::new(
            floem::views::stack((track, thumb))
                .style(|s| s.width_full().position(floem::style::Position::Relative)),
        )
        .style(move |s| {
            // Root: relative flex w-full touch-none items-center select-none
            s.width_full()
                .h_4() // height matches thumb for proper alignment
                .items_center()
                .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                .apply_if(!disabled, |s| s.cursor(CursorStyle::Pointer))
                .padding_left(8.0) // account for thumb overflow
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
