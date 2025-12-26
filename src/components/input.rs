//! Input component with builder-style API
//!
//! Based on shadcn/ui Input component with styled text input.
//!
//! # Example
//!
//! ```rust
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

use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::text::TextInput;
use crate::theme::ShadcnThemeExt;

/// A styled input builder
#[allow(clippy::type_complexity)]
pub struct Input {
    id: ViewId,
    initial_text: String,
    placeholder_text: Option<String>,
    on_enter: Option<Box<dyn Fn(&str)>>,
    on_update: Option<Box<dyn Fn(&str)>>,
    value_fn: Option<Box<dyn Fn() -> String>>,
}

impl Input {
    /// Create a new empty input
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            initial_text: String::new(),
            placeholder_text: None,
            on_enter: None,
            on_update: None,
            value_fn: None,
        }
    }

    /// Create a new input with initial text
    pub fn with_text(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            initial_text: text.into(),
            placeholder_text: None,
            on_enter: None,
            on_update: None,
            value_fn: None,
        }
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder_text = Some(text.into());
        self
    }

    /// Set the callback for when Enter is pressed
    pub fn on_enter(mut self, callback: impl Fn(&str) + 'static) -> Self {
        self.on_enter = Some(Box::new(callback));
        self
    }

    /// Set the callback for when the text changes
    pub fn on_update(mut self, callback: impl Fn(&str) + 'static) -> Self {
        self.on_update = Some(Box::new(callback));
        self
    }

    /// Set the reactive value getter
    pub fn value(mut self, getter: impl Fn() -> String + 'static) -> Self {
        self.value_fn = Some(Box::new(getter));
        self
    }

    /// Build and return the styled TextInput view
    pub fn build(self) -> impl IntoView {
        // Use with_text_and_id to pass our ViewId for proper HasViewId impl
        let mut input = TextInput::with_text_and_id(self.initial_text, self.id);

        // Set placeholder if provided
        if let Some(placeholder) = self.placeholder_text {
            input = input.placeholder(placeholder);
        }

        // Set on_update callback if provided
        if let Some(callback) = self.on_update {
            input = input.on_update(callback);
        }

        // Set reactive value if provided
        if let Some(getter) = self.value_fn {
            input = input.value(getter);
        }

        // Set on_enter callback if provided
        if let Some(callback) = self.on_enter {
            input = input.on_enter(callback);
        }

        input.style(move |s| {
            s.h_10()
                .w_full()
                .rounded_md()
                .border(1.0)
                .px_3()
                .py_2()
                .font_size(14.0)
                .with_shadcn_theme(|s, t| {
                    let ring = t.ring;
                    s.border_color(t.input)
                        .background(t.background)
                        .color(t.foreground)
                        .focus(move |s| s.outline(2.0).outline_color(ring))
                })
        })
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
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
