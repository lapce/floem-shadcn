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
use floem::reactive::{RwSignal, SignalGet};
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::styled::ShadcnStyleExt;

/// A styled progress bar builder.
///
/// Accepts a signal for 0‑100 percent value or can be indeterminate.
pub struct Progress {
    id: ViewId,
    value: Option<RwSignal<f64>>,
    max: f64,
}

impl Progress {
    /// Create a new progress bar with the given value signal (0‑100).
    pub fn new(value: RwSignal<f64>) -> Self {
        Self {
            id: ViewId::new(),
            value: Some(value),
            max: 100.0,
        }
    }

    /// Create an indeterminate progress bar (no specific value).
    pub fn indeterminate() -> Self {
        Self {
            id: ViewId::new(),
            value: None,
            max: 100.0,
        }
    }

    /// Set the maximum value (default 100).
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// Build the progress bar view.
    pub fn build(self) -> impl IntoView {
        let value = self.value;
        let max = self.max;

        floem::views::Container::new(floem::views::Empty::new().style(move |s| {
            let percent = if let Some(v) = value {
                ((v.get() / max) * 100.0).clamp(0.0, 100.0)
            } else {
                30.0
            };

            s.h_full()
                .width_pct(percent)
                .rounded()
                .transition(
                    floem::style::Width,
                    floem::style::Transition::linear(millis(200)),
                )
                .bg_primary()
        }))
        .style(|s| s.w_full().h_2().rounded().bg_muted())
    }
}

impl HasViewId for Progress {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Progress {
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
