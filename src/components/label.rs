//! Label component with builder-style API
//!
//! Based on shadcn/ui Label - accessible label for form controls.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::label::FormLabel;
//!
//! // Simple label
//! FormLabel::new("Email");
//!
//! // Label with modifiers
//! FormLabel::new("Password")
//!     .required()
//!     .disabled(false);
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// FormLabel
// ============================================================================

/// A styled label for form controls (named FormLabel to avoid conflict with floem::views::Label)
pub struct FormLabel {
    id: ViewId,
    text: String,
    required: bool,
    disabled: bool,
    error: bool,
}

impl FormLabel {
    /// Create a new label
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            required: false,
            disabled: false,
            error: false,
        }
    }

    /// Mark as required (shows asterisk)
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set as error state
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }
}

impl HasViewId for FormLabel {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FormLabel {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        let required = self.required;
        let disabled = self.disabled;
        let error = self.error;

        // Build the display text
        let display_text = if required {
            format!("{} *", text)
        } else {
            text
        };

        Box::new(
            floem::views::Label::new(display_text)
                .style(move |s| {
                    s.font_size(14.0)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .line_height(1.0)
                        .with_shadcn_theme(move |s, t| {
                            if error {
                                s.color(t.destructive)
                            } else if disabled {
                                s.color(t.muted_foreground)
                            } else {
                                s.color(t.foreground)
                            }
                        })
                })
        )
    }
}

// ============================================================================
// LabelWithIcon
// ============================================================================

/// Label with an icon prefix
pub struct LabelWithIcon<V> {
    id: ViewId,
    icon: V,
    text: String,
    required: bool,
    disabled: bool,
}

impl<V: IntoView + 'static> LabelWithIcon<V> {
    /// Create a new label with icon
    pub fn new(icon: V, text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            icon,
            text: text.into(),
            required: false,
            disabled: false,
        }
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<V: IntoView + 'static> HasViewId for LabelWithIcon<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for LabelWithIcon<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        let required = self.required;
        let disabled = self.disabled;

        let display_text = if required {
            format!("{} *", text)
        } else {
            text
        };

        let label = floem::views::Label::new(display_text)
            .style(move |s| {
                s.font_size(14.0)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .line_height(1.0)
                    .with_shadcn_theme(move |s, t| {
                        if disabled {
                            s.color(t.muted_foreground)
                        } else {
                            s.color(t.foreground)
                        }
                    })
            });

        Box::new(
            floem::views::h_stack((self.icon, label))
                .style(|s| {
                    s.display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Row)
                        .items_center()
                        .gap(6.0)
                })
        )
    }
}

// ============================================================================
// FormField
// ============================================================================

/// A form field with label and input grouped together
pub struct FormField<L, I> {
    id: ViewId,
    label: L,
    input: I,
    description: Option<String>,
    error_message: Option<String>,
}

impl<L: IntoView + 'static, I: IntoView + 'static> FormField<L, I> {
    /// Create a new form field
    pub fn new(label: L, input: I) -> Self {
        Self {
            id: ViewId::new(),
            label,
            input,
            description: None,
            error_message: None,
        }
    }

    /// Add description text
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Add error message
    pub fn error(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }
}

impl<L: IntoView + 'static, I: IntoView + 'static> HasViewId for FormField<L, I> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<L: IntoView + 'static, I: IntoView + 'static> IntoView for FormField<L, I> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let description = self.description;
        let error_message = self.error_message;

        // Description text (if any)
        let desc_view = if let Some(desc) = description {
            floem::views::Label::new(desc)
                .style(move |s| {
                    s.font_size(12.0)
                        .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        // Error message (if any)
        let error_view = if let Some(err) = error_message {
            floem::views::Label::new(err)
                .style(move |s| {
                    s.font_size(12.0)
                        .with_shadcn_theme(|s, t| s.color(t.destructive))
                })
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        Box::new(
            floem::views::v_stack((self.label, self.input, desc_view, error_view))
                .style(|s| {
                    s.display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Column)
                        .gap(6.0)
                })
        )
    }
}
