//! Alert component with builder-style API
//!
//! Based on shadcn/ui Alert component for displaying feedback messages.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::alert::Alert;
//!
//! // Default alert
//! let alert = Alert::new()
//!     .title("Heads up!")
//!     .description("You can add components to your app using the cli.");
//!
//! // Destructive alert
//! let alert = Alert::new()
//!     .destructive()
//!     .title("Error")
//!     .description("Something went wrong.");
//! ```

use floem::prelude::*;
use floem::text::Weight;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// Alert variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

/// A styled alert builder
pub struct Alert {
    id: ViewId,
    variant: AlertVariant,
    title: Option<String>,
    description: Option<String>,
}

impl Alert {
    /// Create a new alert
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            variant: AlertVariant::Default,
            title: None,
            description: None,
        }
    }

    /// Set the alert variant to destructive
    pub fn destructive(mut self) -> Self {
        self.variant = AlertVariant::Destructive;
        self
    }

    /// Set the alert title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the alert description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Build the alert view
    pub fn build(self) -> impl IntoView {
        let variant = self.variant;
        let mut children: Vec<Box<dyn View>> = Vec::new();

        // Icon based on variant
        let icon_svg = match variant {
            AlertVariant::Default => INFO_ICON_SVG,
            AlertVariant::Destructive => ALERT_ICON_SVG,
        };

        children.push(Box::new(
            floem::views::svg(move || icon_svg.to_string()).style(move |s| {
                s.width(16.0)
                    .height(16.0)
                    .flex_shrink(0.0)
                    .with_shadcn_theme(move |s, t| {
                        let color = match variant {
                            AlertVariant::Default => t.foreground,
                            AlertVariant::Destructive => t.destructive,
                        };
                        s.color(color)
                    })
            }),
        ));

        // Content container
        let mut content_children: Vec<Box<dyn View>> = Vec::new();

        if let Some(title) = self.title {
            content_children.push(Box::new(floem::views::Label::new(title).style(move |s| {
                s.font_size(14.0)
                    .font_weight(Weight::MEDIUM)
                    .line_height(1.0)
                    .with_shadcn_theme(move |s, t| {
                        let color = match variant {
                            AlertVariant::Default => t.foreground,
                            AlertVariant::Destructive => t.destructive,
                        };
                        s.color(color)
                    })
            })));
        }

        if let Some(description) = self.description {
            content_children.push(Box::new(floem::views::Label::new(description).style(
                move |s| {
                    s.font_size(14.0)
                        .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                },
            )));
        }

        children.push(Box::new(
            floem::views::Stack::vertical_from_iter(content_children)
                .style(|s| s.gap(4.0).flex_grow(1.0)),
        ));

        floem::views::Stack::horizontal_from_iter(children).style(move |s| {
            s.width_full()
                .padding(16.0)
                .border_radius(8.0)
                .border(1.0)
                .gap(12.0)
                .items_start()
                .with_shadcn_theme(move |s, t| {
                    let (bg, border_color) = match variant {
                        AlertVariant::Default => (t.background, t.border),
                        AlertVariant::Destructive => {
                            // Subtle red background
                            let destructive_bg = peniko::Color::from_rgba8(
                                t.destructive.to_rgba8().r,
                                t.destructive.to_rgba8().g,
                                t.destructive.to_rgba8().b,
                                25, // Low alpha for subtle background
                            );
                            (destructive_bg, t.destructive)
                        }
                    };
                    s.border_color(border_color).background(bg)
                })
        })
    }
}

impl Default for Alert {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for Alert {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Alert {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// Info icon SVG
const INFO_ICON_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>"#;

// Alert triangle icon SVG
const ALERT_ICON_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>"#;
