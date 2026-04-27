//! Theme system for floem-shadcn

use floem::style::{Style, StylePropValue};
use peniko::Color;
use peniko::color::{AlphaColor, Oklch, Srgb};

#[inline]
fn oklch(l: f32, c: f32, h: f32) -> Color {
    AlphaColor::<Oklch>::new([l, c, h, 1.0]).convert::<Srgb>()
}
#[inline]
fn oklcha(l: f32, c: f32, h: f32, a: f32) -> Color {
    AlphaColor::<Oklch>::new([l, c, h, a]).convert::<Srgb>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadcnTheme {
    pub mode: ThemeMode,
    pub background: Color,
    pub foreground: Color,
    pub card: Color,
    pub card_foreground: Color,
    pub popover: Color,
    pub popover_foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub destructive: Color,
    pub destructive_foreground: Color,
    pub border: Color,
    pub input: Color,
    pub ring: Color,
    pub radius: f32,
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
}
thread_local! { static THEME: std::cell::RefCell<ShadcnTheme> = std::cell::RefCell::new(ShadcnTheme::light()); }
pub fn current_theme() -> ShadcnTheme {
    THEME.with(|t| *t.borrow())
}
pub fn set_theme(t: ShadcnTheme) {
    THEME.with(|c| *c.borrow_mut() = t);
}
impl Default for ShadcnTheme {
    fn default() -> Self {
        Self::light()
    }
}
impl ShadcnTheme {
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            background: oklch(1.0, 0., 0.),
            foreground: oklch(0.145, 0., 0.),
            card: oklch(1.0, 0., 0.),
            card_foreground: oklch(0.145, 0., 0.),
            popover: oklch(1.0, 0., 0.),
            popover_foreground: oklch(0.145, 0., 0.),
            primary: oklch(0.205, 0., 0.),
            primary_foreground: oklch(0.985, 0., 0.),
            secondary: oklch(0.97, 0., 0.),
            secondary_foreground: oklch(0.205, 0., 0.),
            muted: oklch(0.97, 0., 0.),
            muted_foreground: oklch(0.556, 0., 0.),
            accent: oklch(0.97, 0., 0.),
            accent_foreground: oklch(0.205, 0., 0.),
            destructive: oklch(0.577, 0.245, 27.325),
            destructive_foreground: oklch(0.985, 0., 0.),
            border: oklch(0.922, 0., 0.),
            input: oklch(0.922, 0., 0.),
            ring: oklch(0.708, 0., 0.),
            radius: 6.0,
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 8.0,
        }
    }
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            background: oklch(0.145, 0., 0.),
            foreground: oklch(0.985, 0., 0.),
            card: oklch(0.205, 0., 0.),
            card_foreground: oklch(0.985, 0., 0.),
            popover: oklch(0.205, 0., 0.),
            popover_foreground: oklch(0.985, 0., 0.),
            primary: oklch(0.922, 0., 0.),
            primary_foreground: oklch(0.205, 0., 0.),
            secondary: oklch(0.269, 0., 0.),
            secondary_foreground: oklch(0.985, 0., 0.),
            muted: oklch(0.269, 0., 0.),
            muted_foreground: oklch(0.708, 0., 0.),
            accent: oklch(0.269, 0., 0.),
            accent_foreground: oklch(0.985, 0., 0.),
            destructive: oklch(0.704, 0.191, 22.216),
            destructive_foreground: oklch(0.985, 0., 0.),
            border: oklcha(1.0, 0., 0., 0.1),
            input: oklcha(1.0, 0., 0., 0.1),
            ring: oklch(0.556, 0., 0.),
            radius: 6.0,
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 8.0,
        }
    }
    fn adj(c: Color, d: f32) -> Color {
        let o: AlphaColor<Oklch> = c.convert();
        let [l, ch, h, a] = o.components;
        AlphaColor::<Oklch>::new([(l + d).clamp(0., 1.), ch, h, a]).convert::<Srgb>()
    }
    pub fn darken(&self, c: Color) -> Color {
        Self::adj(c, -0.05)
    }
    pub fn lighten(&self, c: Color) -> Color {
        Self::adj(c, 0.05)
    }
    pub fn hover_color(&self, c: Color) -> Color {
        match self.mode {
            ThemeMode::Light => self.darken(c),
            ThemeMode::Dark => self.lighten(c),
        }
    }
    pub fn active_color(&self, c: Color) -> Color {
        match self.mode {
            ThemeMode::Light => Self::adj(c, -0.10),
            ThemeMode::Dark => Self::adj(c, 0.10),
        }
    }
}
fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
impl StylePropValue for ShadcnTheme {
    fn interpolate(&self, o: &Self, val: f64) -> Option<Self> {
        let t = val as f32;
        Some(Self {
            mode: if t < 0.5 { self.mode } else { o.mode },
            background: self.background.lerp(o.background, t, Default::default()),
            foreground: self.foreground.lerp(o.foreground, t, Default::default()),
            card: self.card.lerp(o.card, t, Default::default()),
            card_foreground: self
                .card_foreground
                .lerp(o.card_foreground, t, Default::default()),
            popover: self.popover.lerp(o.popover, t, Default::default()),
            popover_foreground: self.popover_foreground.lerp(
                o.popover_foreground,
                t,
                Default::default(),
            ),
            primary: self.primary.lerp(o.primary, t, Default::default()),
            primary_foreground: self.primary_foreground.lerp(
                o.primary_foreground,
                t,
                Default::default(),
            ),
            secondary: self.secondary.lerp(o.secondary, t, Default::default()),
            secondary_foreground: self.secondary_foreground.lerp(
                o.secondary_foreground,
                t,
                Default::default(),
            ),
            muted: self.muted.lerp(o.muted, t, Default::default()),
            muted_foreground: self
                .muted_foreground
                .lerp(o.muted_foreground, t, Default::default()),
            accent: self.accent.lerp(o.accent, t, Default::default()),
            accent_foreground: self.accent_foreground.lerp(
                o.accent_foreground,
                t,
                Default::default(),
            ),
            destructive: self.destructive.lerp(o.destructive, t, Default::default()),
            destructive_foreground: self.destructive_foreground.lerp(
                o.destructive_foreground,
                t,
                Default::default(),
            ),
            border: self.border.lerp(o.border, t, Default::default()),
            input: self.input.lerp(o.input, t, Default::default()),
            ring: self.ring.lerp(o.ring, t, Default::default()),
            radius: lerp_f32(self.radius, o.radius, t),
            radius_sm: lerp_f32(self.radius_sm, o.radius_sm, t),
            radius_md: lerp_f32(self.radius_md, o.radius_md, t),
            radius_lg: lerp_f32(self.radius_lg, o.radius_lg, t),
        })
    }
}
pub trait ShadcnThemeExt {
    fn with_shadcn_theme(
        self,
        f: impl Fn(floem::style::Style, &ShadcnTheme) -> floem::style::Style + 'static,
    ) -> Self
    where
        Self: Sized;
}
impl ShadcnThemeExt for Style {
    fn with_shadcn_theme(
        self,
        f: impl Fn(floem::style::Style, &ShadcnTheme) -> floem::style::Style + 'static,
    ) -> Self {
        let t = current_theme();
        f(self, &t)
    }
}
