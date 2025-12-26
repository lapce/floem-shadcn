//! Tooltip component with shadcn styling
//!
//! Wraps floem's built-in tooltip with shadcn-style theming.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::tooltip::TooltipExt;
//! use floem_shadcn::components::button::Button;
//!
//! // Add a tooltip to any view
//! Button::new("Hover me").tooltip_styled("This is a tooltip");
//! ```

use floem::prelude::*;
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

/// Extension trait for adding styled tooltips to views
pub trait TooltipExt: IntoView + Sized + 'static {
    /// Add a styled tooltip to this view
    fn tooltip_styled(self, text: impl Into<String>) -> impl IntoView {
        let text = text.into();
        floem::views::TooltipExt::tooltip(self, move || tooltip_content(text.clone()))
    }

    /// Add a styled tooltip with custom content
    fn tooltip_view<V: IntoView + 'static>(
        self,
        content: impl Fn() -> V + 'static,
    ) -> impl IntoView {
        floem::views::TooltipExt::tooltip(self, content)
    }
}

impl<T: IntoView + Sized + 'static> TooltipExt for T {}

/// Create styled tooltip content
fn tooltip_content(text: String) -> impl IntoView {
    floem::views::Label::new(text).style(|s| {
        s.padding_left(8.0)
            .padding_right(8.0)
            .padding_top(4.0)
            .padding_bottom(4.0)
            .border(1.0)
            .border_radius(4.0)
            .font_size(12.0)
            .with_shadcn_theme(|s, t| {
                s.background(t.popover)
                    .color(t.popover_foreground)
                    .border_color(t.border)
            })
    })
}
