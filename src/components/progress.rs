//! Progress component with builder-style API
//!
//! Based on shadcn/ui Progress - a progress bar indicator.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::progress::Progress;
//!
//! let progress = RwSignal::new(60.0);
//!
//! // Basic progress bar
//! let bar = Progress::new(progress);
//!
//! // Indeterminate progress (animated)
//! let bar = Progress::indeterminate();
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet};
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

/// A styled progress bar builder
pub struct Progress {
    id: ViewId,
    value: Option<RwSignal<f64>>,
    max: f64,
}

impl Progress {
    /// Create a new progress bar with the given value signal (0-100)
    pub fn new(value: RwSignal<f64>) -> Self {
        Self {
            id: ViewId::new(),
            value: Some(value),
            max: 100.0,
        }
    }

    /// Create an indeterminate progress bar (no specific value)
    pub fn indeterminate() -> Self {
        Self {
            id: ViewId::new(),
            value: None,
            max: 100.0,
        }
    }

    /// Set the maximum value (default: 100)
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// Build the progress bar view
    pub fn build(self) -> impl IntoView {
        let value = self.value;
        let max = self.max;

        // The track (background)
        

        floem::views::Container::new(
            // The indicator (foreground)
            floem::views::Empty::new()
                .style(move |s| {
                    let percent = if let Some(v) = value {
                        ((v.get() / max) * 100.0).clamp(0.0, 100.0)
                    } else {
                        // Indeterminate - show 30% width
                        30.0
                    };

                    s.height_full()
                        .width_pct(percent)
                        .border_radius(4.0)
                        .transition(floem::style::Width, floem::style::Transition::linear(millis(200)))
                        .with_shadcn_theme(|s, t| s.background(t.primary))
                })
        )
        .style(|s| {
            s.width_full()
                .height(8.0)
                .border_radius(4.0)
                .with_shadcn_theme(|s, t| s.background(t.muted))
        })
    }
}

impl HasViewId for Progress {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Progress {
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
