//! Card component with builder-style API
//!
//! Based on shadcn/ui Card component with header, content, and footer sections.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::card::{Card, CardHeader, CardContent, CardFooter};
//!
//! let card = Card::new((
//!     CardHeader::new()
//!         .title("Create project")
//!         .description("Deploy your new project in one-click."),
//!     CardContent::new(content),
//!     CardFooter::new(buttons),
//! ));
//! ```

use floem::prelude::*;
use floem::text::Weight;
use floem::view::IntoViewIter;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Card
// ============================================================================

/// Card container builder
pub struct Card<C> {
    id: ViewId,
    children: C,
}

impl<C: IntoViewIter> Card<C> {
    /// Create a new card with the given children
    pub fn new(children: C) -> Self {
        Self {
            id: ViewId::new(),
            children,
        }
    }

    /// Build the card view with reactive styling
    pub fn build(self) -> impl IntoView {
        floem::views::Stack::vertical_from_iter(self.children.into_view_iter()).style(|s| {
            s.gap_6()
                .rounded_xl()
                .border(1.0)
                .py_6()
                // Note: shadow-sm not available in floem-tailwind yet
                .with_shadcn_theme(|s, t| {
                    s.border_color(t.border)
                        .background(t.card)
                        .color(t.card_foreground)
                })
        })
    }
}

impl<C: IntoViewIter> HasViewId for Card<C> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<C: IntoViewIter> IntoView for Card<C> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// ============================================================================
// CardHeader
// ============================================================================

/// Card header builder
pub struct CardHeader {
    id: ViewId,
    title: Option<String>,
    description: Option<String>,
}

impl CardHeader {
    /// Create a new card header
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            title: None,
            description: None,
        }
    }

    /// Set the header title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the header description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl Default for CardHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for CardHeader {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CardHeader {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let mut children: Vec<Box<dyn View>> = Vec::new();

        if let Some(title) = self.title {
            children
                .push(Box::new(Label::derived(move || title.clone()).style(|s| {
                    s.text_lg().font_weight(Weight::SEMIBOLD).leading_none()
                })));
        }

        if let Some(description) = self.description {
            children.push(Box::new(Label::derived(move || description.clone()).style(
                |s| {
                    s.text_sm()
                        .line_height(1.43) // 20px / 14px
                        .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                },
            )));
        }

        Box::new(floem::views::Stack::vertical_from_iter(children).style(|s| s.gap(8.0).px_6())) // gap-2 px-6
    }
}

// ============================================================================
// CardContent
// ============================================================================

/// Card content section builder
pub struct CardContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CardContent<V> {
    /// Create a new card content section
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CardContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CardContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| s.px_6()))
    }
}

// ============================================================================
// CardFooter
// ============================================================================

/// Card footer section builder
pub struct CardFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CardFooter<V> {
    /// Create a new card footer section
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CardFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CardFooter<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.flex().items_center().px_6() // flex items-center px-6
            }),
        )
    }
}

// ============================================================================
// CardTitle (standalone, for custom headers)
// ============================================================================

/// Standalone card title builder
pub struct CardTitle<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CardTitle<V> {
    /// Create a new card title
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CardTitle<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CardTitle<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.text_lg().font_weight(Weight::SEMIBOLD).leading_none()),
        )
    }
}

// ============================================================================
// CardDescription (standalone, for custom headers)
// ============================================================================

/// Standalone card description builder
pub struct CardDescription<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CardDescription<V> {
    /// Create a new card description
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CardDescription<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CardDescription<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.text_sm()
                    .line_height(1.43) // 20px / 14px
                    .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
            }),
        )
    }
}
