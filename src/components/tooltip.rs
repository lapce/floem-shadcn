//! Tooltip component with shadcn styling
//!
//! Wraps floem's built-in tooltip with shadcn-style theming.
//!
//! # Example
//!
//! ```
//! use floem_shadcn::components::tooltip::TooltipExt;
//! use floem_shadcn::components::button::Button;
//!
//! // Add a tooltip to any view
//! Button::new("Hover me").tooltip_styled("This is a tooltip");
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::views::Decorators;
use floem_tailwind::TailwindExt;

pub trait TooltipExt: IntoView + Sized + 'static {
    fn tooltip_styled(self, text: impl Into<String>) -> impl IntoView {
        let text = text.into();
        floem::views::TooltipExt::tooltip(self, move || tooltip_content(text.clone()))
    }
    fn tooltip_view<V: IntoView + 'static>(
        self,
        content: impl Fn() -> V + 'static,
    ) -> impl IntoView {
        floem::views::TooltipExt::tooltip(self, content)
    }
}

impl<T: IntoView + Sized + 'static> TooltipExt for T {}

fn tooltip_content(text: String) -> impl IntoView {
    floem::views::Label::new(text).style(|s| {
        s.px_2()
            .py_1()
            .border_1()
            .rounded_sm()
            .text_xs()
            .with_shadcn_theme(|s, t| {
                s.background(t.popover)
                    .color(t.popover_foreground)
                    .border_color(t.border)
            })
    })
}
