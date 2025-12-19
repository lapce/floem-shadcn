//! Textarea component with builder-style API
//!
//! Based on shadcn/ui Textarea - a multi-line text input.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::textarea::Textarea;
//!
//! let content = RwSignal::new(String::new());
//!
//! // Basic textarea
//! let textarea = Textarea::new(content);
//!
//! // With placeholder
//! let textarea = Textarea::new(content).placeholder("Enter your message...");
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::RwSignal;
use floem::style::CursorStyle;
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

/// A styled textarea (multi-line input) builder
pub struct Textarea {
    id: ViewId,
    value: RwSignal<String>,
    placeholder: Option<String>,
    rows: u32,
    disabled: bool,
}

impl Textarea {
    /// Create a new textarea with the given value signal
    pub fn new(value: RwSignal<String>) -> Self { Self { id: ViewId::new(),
            value,
            placeholder: None,
            rows: 3,
            disabled: false,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self { self.placeholder = Some(text.into());
        self
    }

    /// Set the number of visible rows (default: 3)
    pub fn rows(mut self, rows: u32) -> Self { self.rows = rows;
        self
    }

    /// Set the textarea as disabled
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled;
        self
    }

    /// Build the textarea view
    pub fn build(self) -> impl IntoView {
        let value = self.value;
        let placeholder = self.placeholder;
        let disabled = self.disabled;
        let min_height = (self.rows as f64) * 20.0 + 16.0; // Approximate line height + padding

        // Use floem's text_input for now - it handles basic text editing
        // Note: floem doesn't have a native multi-line textarea, so we use text_input
        // with styling to approximate it
        let input = floem::views::text_input(value)
            .placeholder(placeholder.unwrap_or_default())
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width_full()
                        .min_height(min_height)
                        .padding(8.0)
                        .padding_left(12.0)
                        .padding_right(12.0)
                        .background(t.background)
                        .color(t.foreground)
                        .border(1.0)
                        .border_color(t.input)
                        .border_radius(6.0)
                        .font_size(14.0)
                        .cursor(if disabled {
                            CursorStyle::Default
                        } else {
                            CursorStyle::Text
                        })
                        .focus(|s| s.border_color(t.ring))
                        .disabled(|s| s.background(t.muted).color(t.muted_foreground))
                })
            });

        input
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
