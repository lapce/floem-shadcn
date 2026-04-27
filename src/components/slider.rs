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

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

pub struct Slider {
    id: ViewId,
    value: RwSignal<f64>,
    min: f64,
    max: f64,
    step: f64,
    disabled: bool,
}
impl Slider {
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
    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn build(self) -> impl IntoView {
        let value = self.value;
        let min = self.min;
        let max = self.max;
        let disabled = self.disabled;
        let is_dragging = RwSignal::new(false);

        let track = floem::views::Container::new(floem::views::Empty::new().style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let percent = ((value.get() - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                s.border_radius(9999.0)
                    .height_full()
                    .width_pct(percent)
                    .background(t.primary)
            })
        }))
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                s.border_radius(9999.0)
                    .width_full()
                    .height(6.0)
                    .background(t.muted)
                    .position(floem::style::Position::Relative)
            })
        });

        let thumb = floem::views::Empty::new().style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let percent = ((value.get() - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                s.size(16.0, 16.0)
                    .flex_shrink(0.0)
                    .border_radius(9999.0)
                    .background(peniko::Color::WHITE)
                    .border(1.0)
                    .border_color(t.primary)
                    .box_shadow_blur(2.0)
                    .box_shadow_color(peniko::Color::from_rgba8(0, 0, 0, 25))
                    .position(floem::style::Position::Absolute)
                    .inset_top(-5.0)
                    .inset_left_pct(percent)
                    .margin_left(-8.0)
                    .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                    .apply_if(!disabled, |s| s.cursor(CursorStyle::Pointer))
            })
        });

        let calc_value_window = move |container_id: ViewId, x: f64| {
            let content_rect = container_id.get_content_rect();
            let track_width = content_rect.width();
            let click_x = x - content_rect.x0;
            let percent = (click_x / track_width).clamp(0.0, 1.0);
            min + percent * (max - min)
        };
        let calc_value_relative = move |container_id: ViewId, x: f64| {
            let content_rect = container_id.get_content_rect();
            let layout_rect = container_id.get_layout_rect();
            let track_width = content_rect.width();
            let padding = content_rect.x0 - layout_rect.x0;
            let click_x = x - padding;
            let percent = (click_x / track_width).clamp(0.0, 1.0);
            min + percent * (max - min)
        };

        let container_id = ViewId::new();
        floem::views::Container::with_id(
            container_id,
            floem::views::Stack::new((track, thumb))
                .style(|s| s.width_full().position(floem::style::Position::Relative)),
        )
        .style(move |s| {
            s.width_full()
                .height(16.0)
                .items_center()
                .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                .apply_if(!disabled, |s| s.cursor(CursorStyle::Pointer))
                .padding_left(8.0)
                .padding_right(8.0)
        })
        .on_event_stop(floem::event::listener::PointerDown, move |_cx, event| {
            if disabled {
                return;
            }
            let x = event.state.logical_point().x;
            value.set(calc_value_window(container_id, x));
            is_dragging.set(true);
            if let Some(pid) = event.pointer.pointer_id {
                container_id.set_pointer_capture(pid);
            }
        })
        .on_event_stop(floem::event::listener::PointerMove, move |_cx, event| {
            if disabled || !is_dragging.get() {
                return;
            }
            let x = event.current.logical_point().x;
            value.set(calc_value_relative(container_id, x));
        })
        .on_event_stop(floem::event::listener::PointerUp, move |_cx, _event| {
            is_dragging.set(false);
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
