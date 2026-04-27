//! Typography components – text styling primitives
//!
//! Based on shadcn/ui Typography.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::typography::*;
//!
//! let heading = TypographyH1::new("Title");
//! let text = TypographyP::new("Some paragraph.");
//! ```

use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::styled::ShadcnStyleExt;

/// H1 heading – largest font size (3xl).
pub struct TypographyH1 {
    id: ViewId,
    text: String,
}
impl TypographyH1 {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for TypographyH1 {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for TypographyH1 {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| s.text_3xl().leading_tight().font_bold().text_foreground()),
        )
    }
}

/// H2 heading – second largest (2xl).
pub struct TypographyH2 {
    id: ViewId,
    text: String,
}
impl TypographyH2 {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for TypographyH2 {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for TypographyH2 {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| s.text_2xl().leading_tight().font_bold().text_foreground()),
        )
    }
}

/// H3 heading – xl size.
pub struct TypographyH3 {
    id: ViewId,
    text: String,
}
impl TypographyH3 {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for TypographyH3 {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for TypographyH3 {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| s.text_xl().leading_normal().font_bold().text_foreground()),
        )
    }
}

/// H4 heading – 16px, medium weight.
pub struct TypographyH4 {
    id: ViewId,
    text: String,
}
impl TypographyH4 {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for TypographyH4 {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for TypographyH4 {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.font_size(16.0)
                .leading_normal()
                .font_medium()
                .text_foreground()
        }))
    }
}

/// Paragraph – standard body text (sm).
pub struct TypographyP {
    id: ViewId,
    text: String,
}
impl TypographyP {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for TypographyP {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for TypographyP {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| s.text_sm().leading_normal().text_foreground()),
        )
    }
}

/// Lead paragraph – larger intro text (lg).
pub struct TypographyLead {
    id: ViewId,
    text: String,
}
impl TypographyLead {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for TypographyLead {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for TypographyLead {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| s.text_lg().leading_normal().text_muted_foreground()),
        )
    }
}

/// Muted text – small, subdued (xs).
pub struct TypographyMuted {
    id: ViewId,
    text: String,
}
impl TypographyMuted {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for TypographyMuted {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for TypographyMuted {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| s.text_xs().leading_normal().text_muted_foreground()),
        )
    }
}

/// Inline code – monospaced, bordered.
pub struct TypographyCode {
    id: ViewId,
    text: String,
}
impl TypographyCode {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for TypographyCode {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for TypographyCode {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.px_1()
                .py_0p5()
                .font_size(13.0) // explicit because text_sm is 14px
                .bg_muted()
                .text_foreground()
                .border_1()
                .border_border()
                .rounded() // 4px
        }))
    }
}

/// Blockquote – indented left border.
pub struct TypographyBlockquote {
    id: ViewId,
    text: String,
}
impl TypographyBlockquote {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for TypographyBlockquote {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for TypographyBlockquote {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.mt_4()
                .mb_4()
                .pl_4()
                .border_left(2.0) // border-l-2 explicit
                .border_border()
                .text_sm()
                .leading_normal()
                .text_muted_foreground()
        }))
    }
}

/// Ordered or unordered list.
pub struct TypographyList {
    id: ViewId,
    items: Vec<String>,
    ordered: bool,
}
impl TypographyList {
    pub fn new(items: Vec<String>, ordered: bool) -> Self {
        Self {
            id: ViewId::new(),
            items,
            ordered,
        }
    }
}
impl HasViewId for TypographyList {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for TypographyList {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let items = self.items;
        let ordered = self.ordered;
        let children: Vec<Box<dyn View>> = items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let bullet = if ordered {
                    format!("{}. ", i + 1)
                } else {
                    "• ".to_string()
                };
                let text = format!("{}{}", bullet, item);
                Box::new(
                    floem::views::Label::new(text)
                        .style(|s| s.text_sm().leading_normal().text_foreground()),
                ) as Box<dyn View>
            })
            .collect();
        Box::new(
            floem::views::Stack::vertical_from_iter(children).style(|s| s.mt_2().mb_2().gap_1()),
        )
    }
}
