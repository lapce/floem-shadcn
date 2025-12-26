//! Theme system for floem-shadcn
//!
//! Based on shadcn/ui CSS variables approach with support for light and dark modes.
//! Integrates with floem's style system via inherited props for automatic theme propagation.
//! Uses OKLCH color values directly from shadcn/ui for accurate color reproduction.

use floem::prop;
use floem::style::{Style, StylePropValue};
use peniko::Color;
use peniko::color::{AlphaColor, HueDirection, Oklch, Srgb};

/// Create a Color from OKLCH values.
/// - `l`: Lightness (0.0 to 1.0)
/// - `c`: Chroma (0.0+, typically 0 to ~0.4)
/// - `h`: Hue angle in degrees (0 to 360)
#[inline]
fn oklch(l: f32, c: f32, h: f32) -> Color {
    AlphaColor::<Oklch>::new([l, c, h, 1.0]).convert::<Srgb>()
}

/// Create a Color from OKLCH values with alpha.
#[inline]
fn oklcha(l: f32, c: f32, h: f32, a: f32) -> Color {
    AlphaColor::<Oklch>::new([l, c, h, a]).convert::<Srgb>()
}

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
    /// Colors from: https://ui.shadcn.com/themes (Zinc, default)
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,

            background: oklch(1.0, 0.0, 0.0), // --background: oklch(1 0 0)
            foreground: oklch(0.145, 0.0, 0.0), // --foreground: oklch(0.145 0 0)

            card: oklch(1.0, 0.0, 0.0), // --card: oklch(1 0 0)
            card_foreground: oklch(0.145, 0.0, 0.0), // --card-foreground: oklch(0.145 0 0)

            popover: oklch(1.0, 0.0, 0.0), // --popover: oklch(1 0 0)
            popover_foreground: oklch(0.145, 0.0, 0.0), // --popover-foreground: oklch(0.145 0 0)

            primary: oklch(0.205, 0.0, 0.0), // --primary: oklch(0.205 0 0)
            primary_foreground: oklch(0.985, 0.0, 0.0), // --primary-foreground: oklch(0.985 0 0)

            secondary: oklch(0.97, 0.0, 0.0), // --secondary: oklch(0.97 0 0)
            secondary_foreground: oklch(0.205, 0.0, 0.0), // --secondary-foreground: oklch(0.205 0 0)

            muted: oklch(0.97, 0.0, 0.0), // --muted: oklch(0.97 0 0)
            muted_foreground: oklch(0.556, 0.0, 0.0), // --muted-foreground: oklch(0.556 0 0)

            accent: oklch(0.97, 0.0, 0.0), // --accent: oklch(0.97 0 0)
            accent_foreground: oklch(0.205, 0.0, 0.0), // --accent-foreground: oklch(0.205 0 0)

            destructive: oklch(0.577, 0.245, 27.325), // --destructive: oklch(0.577 0.245 27.325)
            destructive_foreground: oklch(0.985, 0.0, 0.0), // --destructive-foreground: oklch(0.985 0 0)

            border: oklch(0.922, 0.0, 0.0), // --border: oklch(0.922 0 0)
            input: oklch(0.922, 0.0, 0.0),  // --input: oklch(0.922 0 0)
            ring: oklch(0.708, 0.0, 0.0),   // --ring: oklch(0.708 0 0)

            // Radius values (in pixels)
            radius: 6.0,
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 8.0,
        }
    }

    /// Create a dark theme
    /// Colors from: https://ui.shadcn.com/themes (Zinc, default)
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,

            background: oklch(0.145, 0.0, 0.0), // --background: oklch(0.145 0 0)
            foreground: oklch(0.985, 0.0, 0.0), // --foreground: oklch(0.985 0 0)

            card: oklch(0.205, 0.0, 0.0), // --card: oklch(0.205 0 0)
            card_foreground: oklch(0.985, 0.0, 0.0), // --card-foreground: oklch(0.985 0 0)

            popover: oklch(0.205, 0.0, 0.0), // --popover: oklch(0.205 0 0)
            popover_foreground: oklch(0.985, 0.0, 0.0), // --popover-foreground: oklch(0.985 0 0)

            primary: oklch(0.922, 0.0, 0.0), // --primary: oklch(0.922 0 0)
            primary_foreground: oklch(0.205, 0.0, 0.0), // --primary-foreground: oklch(0.205 0 0)

            secondary: oklch(0.269, 0.0, 0.0), // --secondary: oklch(0.269 0 0)
            secondary_foreground: oklch(0.985, 0.0, 0.0), // --secondary-foreground: oklch(0.985 0 0)

            muted: oklch(0.269, 0.0, 0.0), // --muted: oklch(0.269 0 0)
            muted_foreground: oklch(0.708, 0.0, 0.0), // --muted-foreground: oklch(0.708 0 0)

            accent: oklch(0.269, 0.0, 0.0), // --accent: oklch(0.269 0 0)
            accent_foreground: oklch(0.985, 0.0, 0.0), // --accent-foreground: oklch(0.985 0 0)

            destructive: oklch(0.704, 0.191, 22.216), // --destructive: oklch(0.704 0.191 22.216)
            destructive_foreground: oklch(0.985, 0.0, 0.0), // --destructive-foreground: oklch(0.985 0 0)

            border: oklcha(1.0, 0.0, 0.0, 0.1), // --border: oklch(1 0 0 / 10%)
            input: oklcha(1.0, 0.0, 0.0, 0.1),  // --input: oklch(1 0 0 / 10%)
            ring: oklch(0.556, 0.0, 0.0),       // --ring: oklch(0.556 0 0)

            // Radius values (same as light)
            radius: 6.0,
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 8.0,
        }
    }

    /// Adjust the lightness of a color in OKLCH space.
    /// Positive delta increases lightness, negative decreases.
    fn adjust_lightness(color: Color, delta: f32) -> Color {
        let oklch: AlphaColor<Oklch> = color.convert();
        let [l, c, h, a] = oklch.components;
        let new_l = (l + delta).clamp(0.0, 1.0);
        AlphaColor::<Oklch>::new([new_l, c, h, a]).convert::<Srgb>()
    }

    /// Get a slightly darker version of a color (for hover states)
    /// Uses OKLCH for perceptually uniform darkening.
    pub fn darken(&self, color: Color) -> Color {
        Self::adjust_lightness(color, -0.05)
    }

    /// Get a slightly lighter version of a color (for hover states in dark mode)
    /// Uses OKLCH for perceptually uniform lightening.
    pub fn lighten(&self, color: Color) -> Color {
        Self::adjust_lightness(color, 0.05)
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
        match self.mode {
            ThemeMode::Light => Self::adjust_lightness(color, -0.10),
            ThemeMode::Dark => Self::adjust_lightness(color, 0.10),
        }
    }

    /// Get a more pronounced active color (for primary buttons)
    pub fn strong_active_color(&self, color: Color) -> Color {
        match self.mode {
            ThemeMode::Light => Self::adjust_lightness(color, -0.15),
            ThemeMode::Dark => Self::adjust_lightness(color, 0.15),
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
