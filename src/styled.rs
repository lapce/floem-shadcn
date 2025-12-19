//! Semantic Tailwind-style utilities for floem-shadcn
//!
//! This module extends floem's `Style` with semantic color utilities
//! that read from the inherited ShadcnTheme prop, similar to how shadcn extends Tailwind.
//!
//! These utilities use floem's `with_context` mechanism, so the theme is resolved
//! at style application time (not construction time), enabling proper theme inheritance.
//!
//! # Example
//!
//! ```rust
//! use floem::style::Style;
//! use floem_shadcn::styled::ShadcnStyleExt;
//!
//! // Simple single-property styling
//! let style = Style::new()
//!     .bg_primary()
//!     .text_primary_foreground();
//!
//! // For multiple properties, prefer with_shadcn_theme for efficiency:
//! use floem_shadcn::theme::ShadcnThemeExt;
//! let style = Style::new()
//!     .with_shadcn_theme(|s, t| {
//!         s.background(t.primary)
//!          .color(t.primary_foreground)
//!          .border_color(t.border)
//!     });
//! ```

use crate::theme::ShadcnThemeProp;
use floem::style::Style;

/// Extension trait adding semantic theme-aware styling methods to `Style`
///
/// Each method uses `with_context` internally to access the inherited `ShadcnThemeProp`.
/// For efficiency when applying multiple theme properties, consider using
/// `with_shadcn_theme` from `ShadcnThemeExt` instead.
pub trait ShadcnStyleExt: Sized {
    // === Background colors ===

    fn bg_background(self) -> Self;
    fn bg_foreground(self) -> Self;
    fn bg_card(self) -> Self;
    fn bg_card_foreground(self) -> Self;
    fn bg_popover(self) -> Self;
    fn bg_popover_foreground(self) -> Self;
    fn bg_primary(self) -> Self;
    fn bg_primary_foreground(self) -> Self;
    fn bg_secondary(self) -> Self;
    fn bg_secondary_foreground(self) -> Self;
    fn bg_muted(self) -> Self;
    fn bg_muted_foreground(self) -> Self;
    fn bg_accent(self) -> Self;
    fn bg_accent_foreground(self) -> Self;
    fn bg_destructive(self) -> Self;
    fn bg_destructive_foreground(self) -> Self;

    // === Text colors ===

    fn text_background(self) -> Self;
    fn text_foreground(self) -> Self;
    fn text_card(self) -> Self;
    fn text_card_foreground(self) -> Self;
    fn text_popover(self) -> Self;
    fn text_popover_foreground(self) -> Self;
    fn text_primary(self) -> Self;
    fn text_primary_foreground(self) -> Self;
    fn text_secondary(self) -> Self;
    fn text_secondary_foreground(self) -> Self;
    fn text_muted(self) -> Self;
    fn text_muted_foreground(self) -> Self;
    fn text_accent(self) -> Self;
    fn text_accent_foreground(self) -> Self;
    fn text_destructive(self) -> Self;
    fn text_destructive_foreground(self) -> Self;

    // === Border colors ===

    fn border_border(self) -> Self;
    fn border_input(self) -> Self;
    fn border_ring(self) -> Self;
    fn border_primary(self) -> Self;
    fn border_secondary(self) -> Self;
    fn border_destructive(self) -> Self;
    fn border_muted(self) -> Self;
    fn border_accent(self) -> Self;

    // === Outline colors ===

    fn outline_ring(self) -> Self;
    fn outline_primary(self) -> Self;
    fn outline_destructive(self) -> Self;

    // === Border radius (from theme) ===

    fn rounded_radius(self) -> Self;
    fn rounded_radius_sm(self) -> Self;
    fn rounded_radius_md(self) -> Self;
    fn rounded_radius_lg(self) -> Self;
}

impl ShadcnStyleExt for Style {
    // === Background colors ===

    fn bg_background(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.background))
    }

    fn bg_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.foreground))
    }

    fn bg_card(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.card))
    }

    fn bg_card_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.card_foreground))
    }

    fn bg_popover(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.popover))
    }

    fn bg_popover_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.popover_foreground))
    }

    fn bg_primary(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.primary))
    }

    fn bg_primary_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.primary_foreground))
    }

    fn bg_secondary(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.secondary))
    }

    fn bg_secondary_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.secondary_foreground))
    }

    fn bg_muted(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.muted))
    }

    fn bg_muted_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.muted_foreground))
    }

    fn bg_accent(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.accent))
    }

    fn bg_accent_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.accent_foreground))
    }

    fn bg_destructive(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.destructive))
    }

    fn bg_destructive_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.destructive_foreground))
    }

    // === Text colors ===

    fn text_background(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.background))
    }

    fn text_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.foreground))
    }

    fn text_card(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.card))
    }

    fn text_card_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.card_foreground))
    }

    fn text_popover(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.popover))
    }

    fn text_popover_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.popover_foreground))
    }

    fn text_primary(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.primary))
    }

    fn text_primary_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.primary_foreground))
    }

    fn text_secondary(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.secondary))
    }

    fn text_secondary_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.secondary_foreground))
    }

    fn text_muted(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.muted))
    }

    fn text_muted_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.muted_foreground))
    }

    fn text_accent(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.accent))
    }

    fn text_accent_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.accent_foreground))
    }

    fn text_destructive(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.destructive))
    }

    fn text_destructive_foreground(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.color(t.destructive_foreground))
    }

    // === Border colors ===

    fn border_border(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_color(t.border))
    }

    fn border_input(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_color(t.input))
    }

    fn border_ring(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_color(t.ring))
    }

    fn border_primary(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_color(t.primary))
    }

    fn border_secondary(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_color(t.secondary))
    }

    fn border_destructive(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_color(t.destructive))
    }

    fn border_muted(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_color(t.muted))
    }

    fn border_accent(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_color(t.accent))
    }

    // === Outline colors ===

    fn outline_ring(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.outline_color(t.ring))
    }

    fn outline_primary(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.outline_color(t.primary))
    }

    fn outline_destructive(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.outline_color(t.destructive))
    }

    // === Border radius ===

    fn rounded_radius(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_radius(t.radius))
    }

    fn rounded_radius_sm(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_radius(t.radius_sm))
    }

    fn rounded_radius_md(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_radius(t.radius_md))
    }

    fn rounded_radius_lg(self) -> Self {
        self.with_context::<ShadcnThemeProp>(|s, t| s.border_radius(t.radius_lg))
    }
}
