//! Input component with builder-style API
//!
//! Based on shadcn/ui Input component with styled text input.
//!
//! # Example
//!
//! ``````rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::input::Input;
//!
//! let text = RwSignal::new(String::new());
//!
//! // Basic input with reactive value
//! let input = Input::new()
//!     .value(move || text.get())
//!     .on_update(move |s| text.set(s.to_string()));
//!
//! // With placeholder
//! let input = Input::new().placeholder("Enter your email");
//!
//! // With initial text
//! let input = Input::with_text("Hello");
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::{Effect, RwSignal, SignalGet, SignalUpdate};
use floem::views::{Decorators, TextInput, TextInputEnter};
use floem::{HasViewId, ViewId};

/// A styled input builder with support for reactive value binding,
/// placeholder, on_update, and on_enter callbacks.
pub struct Input {
    id: ViewId,
    buffer: RwSignal<String>,
    placeholder: Option<String>,
    on_enter: Option<Box<dyn Fn(&str)>>,
}
impl Input {
    pub fn new() -> Self {
        Self::with_text("")
    }

    pub fn with_text(initial: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            buffer: RwSignal::new(initial.into()),
            placeholder: None,
            on_enter: None,
        }
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    pub fn on_update(self, f: impl Fn(&str) + 'static) -> Self {
        let buf = self.buffer;
        Effect::new(move |_| {
            let text = buf.get();
            f(&text);
        });
        self
    }

    pub fn on_enter(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_enter = Some(Box::new(f));
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

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for Input {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for Input {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let mut view = TextInput::new(self.buffer);
        if let Some(ph) = self.placeholder {
            view = view.placeholder(ph);
        }
        if let Some(cb) = self.on_enter {
            let buffer = self.buffer;
            view = view.on_event_stop(TextInputEnter::listener(), move |_, _| {
                cb(&buffer.get_untracked());
            });
        }
        Box::new(view.style(move |s| {
            s.height(40.0)
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
