//! Textarea component with builder-style API
//!
//! Based on shadcn/ui Textarea - a multi-line text input.
//!
//! # Example
//!
//! ``````rust
//! use floem_shadcn::components::textarea::Textarea;
//!
//! // Basic textarea
//! let textarea = Textarea::new("Initial text");
//!
//! // With placeholder, rows, and change handler
//! let textarea = Textarea::new("")
//!     .placeholder("Enter your message...")
//!     .rows(5)
//!     .on_change(|text| println!("Text changed: {}", text));
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::{Effect, RwSignal, SignalGet, SignalUpdate};
use floem::views::{Decorators, TextInput};
use floem::{HasViewId, ViewId};

/// A styled multi-line text area.
/// Uses floem's TextInput internally with a minimum height.
pub struct Textarea {
    id: ViewId,
    buffer: RwSignal<String>,
    placeholder: Option<String>,
    rows: u32,
    on_change: Option<Box<dyn Fn(&str)>>,
}

impl Textarea {
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            buffer: RwSignal::new(initial.into()),
            placeholder: None,
            rows: 3,
            on_change: None,
        }
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = rows;
        self
    }

    pub fn on_change(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn value(self, getter: impl Fn() -> String + 'static) -> Self {
        let buf = self.buffer;
        Effect::new(move |_| {
            let new_value = getter();
            if buf.get_untracked() != new_value {
                buf.set(new_value);
            }
        });
        self
    }

    pub fn buffer(&self) -> RwSignal<String> {
        self.buffer
    }
}

impl HasViewId for Textarea {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Textarea {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let min_height = (self.rows as f64) * 24.0 + 16.0;
        let buf = self.buffer;

        // Wire on_change if provided
        if let Some(cb) = self.on_change {
            let buf = buf;
            Effect::new(move |_| {
                let text = buf.get_untracked();
                cb(&text);
            });
        }

        let mut view = TextInput::new(buf);
        if let Some(ph) = self.placeholder {
            view = view.placeholder(ph);
        }

        Box::new(view.style(move |s| {
            s.min_height(min_height)
                .width_full()
                .border_radius(6.0)
                .border(1.0)
                .padding_left(12.0)
                .padding_right(12.0)
                .padding_top(8.0)
                .padding_bottom(8.0)
                .font_size(14.0)
                .with_shadcn_theme(|s, t| {
                    let ring = t.ring;
                    s.border_color(t.input)
                        .background(t.background)
                        .color(t.foreground)
                        .focus(move |s| s.outline(2.0).outline_color(ring))
                })
        }))
    }
}
