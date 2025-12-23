//! Badge component with builder-style API
//!
//! Based on shadcn/ui Badge component with support for multiple variants.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::badge::Badge;
//!
//! // Default badge
//! let badge = Badge::new("New");
//!
//! // Builder-style customization
//! let secondary = Badge::new("Beta").secondary();
//! let error = Badge::new("Error").destructive();
//! let outlined = Badge::new("v1.0").outline();
//! ```

use floem::prelude::*;
use floem::style::Style;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::{ShadcnTheme, ShadcnThemeExt};

/// Badge variants following shadcn/ui conventions
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

/// A styled badge builder
pub struct Badge<V> {
    id: ViewId,
    child: V,
    variant: BadgeVariant,
}

impl<V: IntoView + 'static> Badge<V> {
    /// Create a new badge with the given content
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            variant: BadgeVariant::Default,
        }
    }

    // === Variant methods ===

    /// Set badge to secondary variant
    pub fn secondary(mut self) -> Self {
        self.variant = BadgeVariant::Secondary;
        self
    }

    /// Set badge to destructive variant (red/danger)
    pub fn destructive(mut self) -> Self {
        self.variant = BadgeVariant::Destructive;
        self
    }

    /// Set badge to outline variant (bordered, transparent background)
    pub fn outline(mut self) -> Self {
        self.variant = BadgeVariant::Outline;
        self
    }

    /// Set the badge variant explicitly
    pub fn with_variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Build the badge view with reactive styling
    pub fn build(self) -> impl IntoView {
        let variant = self.variant;

        floem::views::Container::with_id(self.id, self.child)
            .style(move |s| build_badge_style(s, variant))
    }
}

impl<V: IntoView + 'static> HasViewId for Badge<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Badge<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

fn build_badge_style(s: Style, variant: BadgeVariant) -> Style {
    // Base styles using floem-tailwind
    // Use rounded_xl (12px) for pill shape - half the badge height (~22px)
    let s = s
        .flex()
        .items_center()
        .rounded_full() // 12px radius for pill shape
        .border_1()
        .px_2() // 8px horizontal padding
        .py_0p5() // 2px vertical padding
        .text_xs() // 12px font size
        .font_medium()
        .transition(
            floem::style::Background,
            floem::style::Transition::linear(millis(100)),
        );

    // Theme-dependent styles
    s.with_shadcn_theme(move |s, t| apply_variant_style(s, variant, t))
}

fn apply_variant_style(s: Style, variant: BadgeVariant, t: &ShadcnTheme) -> Style {
    match variant {
        BadgeVariant::Default => s
            .border_color(peniko::Color::TRANSPARENT)
            .background(t.primary)
            .color(t.primary_foreground),
        BadgeVariant::Secondary => s
            .border_color(peniko::Color::TRANSPARENT)
            .background(t.secondary)
            .color(t.secondary_foreground),
        BadgeVariant::Destructive => s
            .border_color(peniko::Color::TRANSPARENT)
            .background(t.destructive)
            .color(t.destructive_foreground),
        BadgeVariant::Outline => s
            .border_color(t.border)
            .background(peniko::Color::TRANSPARENT)
            .color(t.foreground),
    }
}

fn millis(ms: u64) -> std::time::Duration {
    std::time::Duration::from_millis(ms)
}
