//! Theme system for floem-shadcn
//!
//! Based on shadcn/ui CSS variables approach with support for light and dark modes.
//! Integrates with floem's style system via inherited props for automatic theme propagation.

use floem::prop;
use floem::style::{Style, StylePropValue};
use peniko::color::HueDirection;
use peniko::Color;

/// Theme mode (light or dark)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

/// Design tokens for shadcn-style theming
///
/// All colors follow the shadcn/ui convention with background/foreground pairs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadcnTheme {
    pub mode: ThemeMode,

    // Base colors
    pub background: Color,
    pub foreground: Color,

    // Card
    pub card: Color,
    pub card_foreground: Color,

    // Popover
    pub popover: Color,
    pub popover_foreground: Color,

    // Primary (typically used for buttons, links)
    pub primary: Color,
    pub primary_foreground: Color,

    // Secondary
    pub secondary: Color,
    pub secondary_foreground: Color,

    // Muted (for disabled or subtle elements)
    pub muted: Color,
    pub muted_foreground: Color,

    // Accent (for hover states)
    pub accent: Color,
    pub accent_foreground: Color,

    // Destructive (for error/delete actions)
    pub destructive: Color,
    pub destructive_foreground: Color,

    // Border and input
    pub border: Color,
    pub input: Color,
    pub ring: Color,

    // Radius values
    pub radius: f32,
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
}

impl Default for ShadcnTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl ShadcnTheme {
    /// Create a light theme (shadcn default)
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,

            // HSL: 0 0% 100% -> white
            background: Color::from_rgba8(255, 255, 255, 255),
            // HSL: 240 10% 3.9% -> near black
            foreground: Color::from_rgba8(10, 10, 11, 255),

            // Card same as background in light mode
            card: Color::from_rgba8(255, 255, 255, 255),
            card_foreground: Color::from_rgba8(10, 10, 11, 255),

            // Popover same as card
            popover: Color::from_rgba8(255, 255, 255, 255),
            popover_foreground: Color::from_rgba8(10, 10, 11, 255),

            // Primary: HSL 240 5.9% 10% -> dark gray
            primary: Color::from_rgba8(24, 24, 27, 255),
            // Primary foreground: HSL 0 0% 98% -> near white
            primary_foreground: Color::from_rgba8(250, 250, 250, 255),

            // Secondary: HSL 240 4.8% 95.9% -> light gray
            secondary: Color::from_rgba8(244, 244, 245, 255),
            secondary_foreground: Color::from_rgba8(24, 24, 27, 255),

            // Muted: HSL 240 4.8% 95.9%
            muted: Color::from_rgba8(244, 244, 245, 255),
            // Muted foreground: HSL 240 3.8% 46.1%
            muted_foreground: Color::from_rgba8(113, 113, 122, 255),

            // Accent: HSL 240 4.8% 95.9%
            accent: Color::from_rgba8(244, 244, 245, 255),
            accent_foreground: Color::from_rgba8(24, 24, 27, 255),

            // Destructive: HSL 0 84.2% 60.2% -> red
            destructive: Color::from_rgba8(239, 68, 68, 255),
            destructive_foreground: Color::from_rgba8(250, 250, 250, 255),

            // Border: HSL 240 5.9% 90%
            border: Color::from_rgba8(228, 228, 231, 255),
            // Input border same as border
            input: Color::from_rgba8(228, 228, 231, 255),
            // Ring: HSL 240 5.9% 10%
            ring: Color::from_rgba8(24, 24, 27, 255),

            // Radius values (in pixels)
            radius: 6.0,
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 8.0,
        }
    }

    /// Create a dark theme
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,

            // HSL: 240 10% 3.9% -> near black
            background: Color::from_rgba8(10, 10, 11, 255),
            // HSL: 0 0% 98% -> near white
            foreground: Color::from_rgba8(250, 250, 250, 255),

            // Card: slightly lighter than background
            card: Color::from_rgba8(10, 10, 11, 255),
            card_foreground: Color::from_rgba8(250, 250, 250, 255),

            // Popover
            popover: Color::from_rgba8(10, 10, 11, 255),
            popover_foreground: Color::from_rgba8(250, 250, 250, 255),

            // Primary: near white in dark mode
            primary: Color::from_rgba8(250, 250, 250, 255),
            primary_foreground: Color::from_rgba8(24, 24, 27, 255),

            // Secondary: HSL 240 3.7% 15.9%
            secondary: Color::from_rgba8(39, 39, 42, 255),
            secondary_foreground: Color::from_rgba8(250, 250, 250, 255),

            // Muted: HSL 240 3.7% 15.9%
            muted: Color::from_rgba8(39, 39, 42, 255),
            // Muted foreground: HSL 240 5% 64.9%
            muted_foreground: Color::from_rgba8(161, 161, 170, 255),

            // Accent: HSL 240 3.7% 15.9%
            accent: Color::from_rgba8(39, 39, 42, 255),
            accent_foreground: Color::from_rgba8(250, 250, 250, 255),

            // Destructive: HSL 0 62.8% 30.6% -> darker red
            destructive: Color::from_rgba8(127, 29, 29, 255),
            destructive_foreground: Color::from_rgba8(250, 250, 250, 255),

            // Border: HSL 240 3.7% 15.9%
            border: Color::from_rgba8(39, 39, 42, 255),
            input: Color::from_rgba8(39, 39, 42, 255),
            // Ring: HSL 240 4.9% 83.9%
            ring: Color::from_rgba8(212, 212, 216, 255),

            // Radius values (same as light)
            radius: 6.0,
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 8.0,
        }
    }

    /// Get a slightly darker version of a color (for hover states)
    pub fn darken(&self, color: Color) -> Color {
        let rgba = color.to_rgba8();
        Color::from_rgba8(
            (rgba.r as f32 * 0.9) as u8,
            (rgba.g as f32 * 0.9) as u8,
            (rgba.b as f32 * 0.9) as u8,
            rgba.a,
        )
    }

    /// Get a slightly lighter version of a color (for hover states in dark mode)
    pub fn lighten(&self, color: Color) -> Color {
        let rgba = color.to_rgba8();
        Color::from_rgba8(
            (rgba.r as f32 * 1.1).min(255.0) as u8,
            (rgba.g as f32 * 1.1).min(255.0) as u8,
            (rgba.b as f32 * 1.1).min(255.0) as u8,
            rgba.a,
        )
    }

    /// Get hover color based on theme mode
    pub fn hover_color(&self, color: Color) -> Color {
        match self.mode {
            ThemeMode::Light => self.darken(color),
            ThemeMode::Dark => self.lighten(color),
        }
    }

    /// Get active/pressed color based on theme mode (more pronounced than hover)
    pub fn active_color(&self, color: Color) -> Color {
        let rgba = color.to_rgba8();
        match self.mode {
            ThemeMode::Light => {
                // Darken more than hover (0.8 vs 0.9)
                Color::from_rgba8(
                    (rgba.r as f32 * 0.8) as u8,
                    (rgba.g as f32 * 0.8) as u8,
                    (rgba.b as f32 * 0.8) as u8,
                    rgba.a,
                )
            }
            ThemeMode::Dark => {
                // Lighten more than hover (1.2 vs 1.1)
                Color::from_rgba8(
                    (rgba.r as f32 * 1.2).min(255.0) as u8,
                    (rgba.g as f32 * 1.2).min(255.0) as u8,
                    (rgba.b as f32 * 1.2).min(255.0) as u8,
                    rgba.a,
                )
            }
        }
    }

    /// Get a more pronounced active color (for primary buttons)
    pub fn strong_active_color(&self, color: Color) -> Color {
        let rgba = color.to_rgba8();
        match self.mode {
            ThemeMode::Light => {
                Color::from_rgba8(
                    (rgba.r as f32 * 0.65) as u8,
                    (rgba.g as f32 * 0.65) as u8,
                    (rgba.b as f32 * 0.65) as u8,
                    rgba.a,
                )
            }
            ThemeMode::Dark => {
                Color::from_rgba8(
                    (rgba.r as f32 * 1.35).min(255.0) as u8,
                    (rgba.g as f32 * 1.35).min(255.0) as u8,
                    (rgba.b as f32 * 1.35).min(255.0) as u8,
                    rgba.a,
                )
            }
        }
    }
}

/// Helper to interpolate between two colors
fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    a.lerp(b, t, HueDirection::default())
}

/// Helper to interpolate between two f32 values
fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

impl StylePropValue for ShadcnTheme {
    fn interpolate(&self, other: &Self, value: f64) -> Option<Self> {
        let t = value as f32;

        Some(ShadcnTheme {
            // Mode switches at midpoint
            mode: if t < 0.5 { self.mode } else { other.mode },

            // Interpolate all colors
            background: lerp_color(self.background, other.background, t),
            foreground: lerp_color(self.foreground, other.foreground, t),
            card: lerp_color(self.card, other.card, t),
            card_foreground: lerp_color(self.card_foreground, other.card_foreground, t),
            popover: lerp_color(self.popover, other.popover, t),
            popover_foreground: lerp_color(self.popover_foreground, other.popover_foreground, t),
            primary: lerp_color(self.primary, other.primary, t),
            primary_foreground: lerp_color(self.primary_foreground, other.primary_foreground, t),
            secondary: lerp_color(self.secondary, other.secondary, t),
            secondary_foreground: lerp_color(
                self.secondary_foreground,
                other.secondary_foreground,
                t,
            ),
            muted: lerp_color(self.muted, other.muted, t),
            muted_foreground: lerp_color(self.muted_foreground, other.muted_foreground, t),
            accent: lerp_color(self.accent, other.accent, t),
            accent_foreground: lerp_color(self.accent_foreground, other.accent_foreground, t),
            destructive: lerp_color(self.destructive, other.destructive, t),
            destructive_foreground: lerp_color(
                self.destructive_foreground,
                other.destructive_foreground,
                t,
            ),
            border: lerp_color(self.border, other.border, t),
            input: lerp_color(self.input, other.input, t),
            ring: lerp_color(self.ring, other.ring, t),

            // Interpolate radius values
            radius: lerp_f32(self.radius, other.radius, t),
            radius_sm: lerp_f32(self.radius_sm, other.radius_sm, t),
            radius_md: lerp_f32(self.radius_md, other.radius_md, t),
            radius_lg: lerp_f32(self.radius_lg, other.radius_lg, t),
        })
    }
}

// =============================================================================
// ShadcnThemeProp - Inherited style property for theme propagation
// =============================================================================

prop!(
    /// The shadcn theme property. This is an inherited prop that flows down the view tree.
    /// Set it at your app root and all descendant views can access it via `with_shadcn_theme`.
    pub ShadcnThemeProp: ShadcnTheme { inherited } = ShadcnTheme::light()
);

/// Extension trait for accessing the shadcn theme in styles
pub trait ShadcnThemeExt {
    /// Set the shadcn theme for this style and all descendants
    fn shadcn_theme(self, theme: ShadcnTheme) -> Self;

    /// Access the inherited shadcn theme to apply theme-aware styles.
    ///
    /// Use this when you need multiple theme properties in one style block:
    /// ```rust
    /// s.with_shadcn_theme(|s, t| {
    ///     s.background(t.primary)
    ///      .color(t.primary_foreground)
    ///      .border_color(t.border)
    /// })
    /// ```
    fn with_shadcn_theme(self, f: impl Fn(Self, &ShadcnTheme) -> Self + 'static) -> Self
    where
        Self: Sized;
}

impl ShadcnThemeExt for Style {
    fn shadcn_theme(self, theme: ShadcnTheme) -> Self {
        self.set(ShadcnThemeProp, theme)
    }

    fn with_shadcn_theme(self, f: impl Fn(Self, &ShadcnTheme) -> Self + 'static) -> Self {
        self.with_context::<ShadcnThemeProp>(f)
    }
}

