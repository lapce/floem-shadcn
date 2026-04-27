//! Button component with builder-style API
//!
//! Based on shadcn/ui Button component with support for multiple variants and sizes.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::button::Button;
//!
//! // Default button
//! let btn = Button::new("Click me");
//!
//! // Builder-style customization
//! let small_ghost = Button::new("Settings").sm().ghost();
//! let large_destructive = Button::new("Delete").lg().destructive();
//! let icon_button = Button::new(icon_view).icon().outline();
//! ```

use floem::prelude::*;
use floem::style::Style;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::{ShadcnTheme, ShadcnThemeExt};

/// Button variants following shadcn/ui conventions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

/// Button sizes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
}

/// A styled button builder.
///
/// Use the builder methods to set variant, size, and attach event handlers.
///
/// # Example
///
/// ```rust
/// use floem_shadcn::components::button::Button;
///
/// let btn = Button::new("Click me")
///     .secondary()
///     .sm()
///     .on_event_stop(floem::event::listener::Click, move |_, _| println!("clicked"));
/// ```
pub struct Button<V> {
    id: ViewId,
    child: V,
    variant: ButtonVariant,
    size: ButtonSize,
}

impl<V: IntoView + 'static> Button<V> {
    /// Create a new button with the given content.
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
        }
    }

    /// Set button to small size.
    pub fn sm(mut self) -> Self {
        self.size = ButtonSize::Sm;
        self
    }

    /// Set button to large size.
    pub fn lg(mut self) -> Self {
        self.size = ButtonSize::Lg;
        self
    }

    /// Set button to icon size (square).
    pub fn icon(mut self) -> Self {
        self.size = ButtonSize::Icon;
        self
    }

    /// Set button to destructive variant (red/danger).
    pub fn destructive(mut self) -> Self {
        self.variant = ButtonVariant::Destructive;
        self
    }

    /// Set button to outline variant (bordered).
    pub fn outline(mut self) -> Self {
        self.variant = ButtonVariant::Outline;
        self
    }

    /// Set button to secondary variant.
    pub fn secondary(mut self) -> Self {
        self.variant = ButtonVariant::Secondary;
        self
    }

    /// Set button to ghost variant (transparent background).
    pub fn ghost(mut self) -> Self {
        self.variant = ButtonVariant::Ghost;
        self
    }

    /// Set button to link variant (looks like a link).
    pub fn link(mut self) -> Self {
        self.variant = ButtonVariant::Link;
        self
    }

    /// Set the button variant explicitly.
    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the button size explicitly.
    pub fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Build the button view with reactive styling.
    pub fn build(self) -> impl IntoView {
        let size = self.size;
        let variant = self.variant;

        floem::views::Container::with_id(self.id, self.child)
            .style(move |s| build_button_style(s, size, variant))
    }
}

impl<V: IntoView + 'static> HasViewId for Button<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Button<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

fn build_button_style(s: Style, size: ButtonSize, variant: ButtonVariant) -> Style {
    let s = s
        .flex()
        .items_center()
        .justify_center()
        .flex_grow(0.0)
        .cursor(floem::style::CursorStyle::Pointer)
        .transition(
            floem::style::Background,
            floem::style::Transition::linear(millis(100)),
        )
        .transition(
            floem::style::TextColor,
            floem::style::Transition::linear(millis(100)),
        );

    let s = match size {
        ButtonSize::Sm => s.h_9().px_3().rounded_md().text_xs().font_medium(),
        ButtonSize::Default => s.h_10().px_4().py_2().rounded_md().text_sm().font_medium(),
        ButtonSize::Lg => s.h_11().px_8().rounded_md().text_sm().font_medium(),
        ButtonSize::Icon => s.h_10().w_10().rounded_md().font_medium(),
    };

    s.with_shadcn_theme(move |s, t| {
        let s = apply_variant_style(s, variant, t);
        let s = apply_hover_style(s, variant, t);
        apply_active_style(s, variant, t)
    })
}

fn apply_variant_style(s: Style, variant: ButtonVariant, t: &ShadcnTheme) -> Style {
    match variant {
        ButtonVariant::Default => s
            .background(t.primary)
            .color(t.primary_foreground)
            .border_1()
            .border_color(peniko::Color::TRANSPARENT),
        ButtonVariant::Destructive => s
            .background(t.destructive)
            .color(t.destructive_foreground)
            .border_1()
            .border_color(peniko::Color::TRANSPARENT),
        ButtonVariant::Outline => s
            .background(t.background)
            .color(t.foreground)
            .border_1()
            .border_color(t.input),
        ButtonVariant::Secondary => s
            .background(t.secondary)
            .color(t.secondary_foreground)
            .border_1()
            .border_color(peniko::Color::TRANSPARENT),
        ButtonVariant::Ghost => s
            .background(peniko::Color::TRANSPARENT)
            .color(t.foreground)
            .border_1()
            .border_color(peniko::Color::TRANSPARENT),
        ButtonVariant::Link => s
            .background(peniko::Color::TRANSPARENT)
            .color(t.primary)
            .border_1()
            .border_color(peniko::Color::TRANSPARENT),
    }
}

fn apply_hover_style(s: Style, variant: ButtonVariant, t: &ShadcnTheme) -> Style {
    let hover_primary = with_alpha(t.primary, 0.9);
    let hover_destructive = with_alpha(t.destructive, 0.9);
    let hover_secondary = with_alpha(t.secondary, 0.8);
    let accent = t.accent;
    let accent_foreground = t.accent_foreground;

    s.hover(move |s| match variant {
        ButtonVariant::Default => s.background(hover_primary),
        ButtonVariant::Destructive => s.background(hover_destructive),
        ButtonVariant::Outline => s.background(accent).color(accent_foreground),
        ButtonVariant::Secondary => s.background(hover_secondary),
        ButtonVariant::Ghost => s.background(accent).color(accent_foreground),
        ButtonVariant::Link => s,
    })
}

fn apply_active_style(s: Style, variant: ButtonVariant, t: &ShadcnTheme) -> Style {
    let active_primary = with_alpha(t.primary, 0.8);
    let active_destructive = with_alpha(t.destructive, 0.8);
    let active_secondary = with_alpha(t.secondary, 0.7);
    let active_accent = with_alpha(t.accent, 0.9);
    let accent_foreground = t.accent_foreground;

    s.active(move |s| match variant {
        ButtonVariant::Default => s.background(active_primary),
        ButtonVariant::Destructive => s.background(active_destructive),
        ButtonVariant::Outline => s.background(active_accent).color(accent_foreground),
        ButtonVariant::Secondary => s.background(active_secondary),
        ButtonVariant::Ghost => s.background(active_accent).color(accent_foreground),
        ButtonVariant::Link => s,
    })
}

fn with_alpha(color: peniko::Color, alpha: f32) -> peniko::Color {
    color.with_alpha(alpha)
}

fn millis(ms: u64) -> std::time::Duration {
    std::time::Duration::from_millis(ms)
}
