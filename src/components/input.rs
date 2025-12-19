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
//! // Basic input
//! let input = Input::new(text);
//!
//! // With placeholder
//! let input = Input::new(text).placeholder("Enter your email");
//! ```

use floem::reactive::RwSignal;
use floem::views::{text_input, Decorators, TextInput};
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// A styled input builder
pub struct Input {
    buffer: RwSignal<String>,
    placeholder_text: Option<String>,
}

impl Input {
    /// Create a new input with the given buffer signal
    pub fn new(buffer: RwSignal<String>) -> Self {
        Self {
            buffer,
            placeholder_text: None,
        }
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder_text = Some(text.into());
        self
    }

    /// Build and return the styled TextInput view
    pub fn build(self) -> TextInput {
        let input = text_input(self.buffer);

        let input = if let Some(placeholder) = self.placeholder_text {
            input.placeholder(placeholder)
        } else {
            input
        };

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

impl floem::IntoView for Input {
    type V = TextInput;
    type Intermediate = TextInput;

    fn into_intermediate(self) -> Self::Intermediate {
        self.build()
    }
}
