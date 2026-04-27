//! Label component with builder-style API
//!
//! Based on shadcn/ui Label - accessible label for form controls.
//!
//! # Example
//!
//! ```
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
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::styled::ShadcnStyleExt;
use crate::theme::ShadcnThemeExt;

pub struct FormLabel {
    id: ViewId,
    text: String,
    required: bool,
    disabled: bool,
    error: bool,
}

impl FormLabel {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            required: false,
            disabled: false,
            error: false,
        }
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        let required = self.required;
        let disabled = self.disabled;
        let error = self.error;
        let display_text = if required {
            format!("{} *", text)
        } else {
            text
        };

        Box::new(floem::views::Label::new(display_text).style(move |s| {
            s.text_sm()
                .font_medium()
                .leading_none()
                .with_shadcn_theme(move |s, t| {
                    if error {
                        s.color(t.destructive)
                    } else if disabled {
                        s.color(t.muted_foreground)
                    } else {
                        s.color(t.foreground)
                    }
                })
        }))
    }
}

pub struct LabelWithIcon<V> {
    id: ViewId,
    icon: V,
    text: String,
    required: bool,
    disabled: bool,
}

impl<V: IntoView + 'static> LabelWithIcon<V> {
    pub fn new(icon: V, text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            icon,
            text: text.into(),
            required: false,
            disabled: false,
        }
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
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

        let label = floem::views::Label::new(display_text).style(move |s| {
            s.text_sm()
                .font_medium()
                .leading_none()
                .with_shadcn_theme(move |s, t| {
                    if disabled {
                        s.color(t.muted_foreground)
                    } else {
                        s.color(t.foreground)
                    }
                })
        });

        Box::new(
            floem::views::Stack::horizontal((self.icon, label))
                .style(|s| s.flex_row().items_center().gap_1p5()),
        )
    }
}

pub struct FormField<L, I> {
    id: ViewId,
    label: L,
    input: I,
    description: Option<String>,
    error_message: Option<String>,
}

impl<L: IntoView + 'static, I: IntoView + 'static> FormField<L, I> {
    pub fn new(label: L, input: I) -> Self {
        Self {
            id: ViewId::new(),
            label,
            input,
            description: None,
            error_message: None,
        }
    }
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let description = self.description;
        let error_message = self.error_message;

        let desc_view = if let Some(desc) = description {
            floem::views::Label::new(desc)
                .style(move |s| s.text_xs().text_muted_foreground())
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        let error_view = if let Some(err) = error_message {
            floem::views::Label::new(err)
                .style(move |s| s.text_xs().with_shadcn_theme(|s, t| s.color(t.destructive)))
                .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        Box::new(
            floem::views::Stack::vertical((self.label, self.input, desc_view, error_view))
                .style(|s| s.flex_col().gap_1p5()),
        )
    }
}
