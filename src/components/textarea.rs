//! Textarea component with builder-style API
//!
//! Based on shadcn/ui Textarea - a multi-line text input.
//!
//! # Example
//!
//! ```rust
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

use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::text::TextArea as TextAreaView;
use crate::theme::ShadcnThemeExt;

/// A styled textarea (multi-line input) builder
pub struct Textarea {
    id: ViewId,
    initial_value: String,
    placeholder: Option<String>,
    rows: u32,
    on_change: Option<Box<dyn Fn(&str)>>,
    resizable: bool,
}

impl Textarea {
    /// Create a new textarea with the given initial value
    pub fn new(initial_value: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            initial_value: initial_value.into(),
            placeholder: None,
            rows: 3,
            on_change: None,
            resizable: false,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set the number of visible rows (default: 3)
    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = rows;
        self
    }

    /// Set a callback for when the text changes
    pub fn on_change(mut self, on_change: impl Fn(&str) + 'static) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Enable or disable the resize handle (drag grip at bottom-right corner)
    pub fn resizable(mut self, enabled: bool) -> Self {
        self.resizable = enabled;
        self
    }

    /// Build the textarea view
    pub fn build(self) -> impl IntoView {
        let min_height = (self.rows as f64) * 24.0 + 16.0; // line height * rows + padding

        // Use our custom multi-line TextArea, passing our ViewId for proper HasViewId impl
        let mut textarea = TextAreaView::with_text_and_id(self.initial_value, self.id)
            .resizable(self.resizable);

        if let Some(on_change) = self.on_change {
            textarea = textarea.on_update(move |text| {
                on_change(text);
            });
        }

        textarea.style(move |s| {
            s.min_height(min_height)
                .w_full()
                .rounded_md()
                .border_1()
                .px_3()
                .py_2()
                .text_sm()
                .with_shadcn_theme(|s, t| {
                    let ring = t.ring;
                    s.border_color(t.input)
                        .background(t.background)
                        .color(t.foreground)
                        .focus(move |s| s.outline(2.0).outline_color(ring))
                        .disabled(|s| s.background(t.muted).color(t.muted_foreground))
                })
        })
    }
}

impl HasViewId for Textarea {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Textarea {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
