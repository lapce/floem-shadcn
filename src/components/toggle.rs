//! Toggle component with builder-style API
//!
//! Based on shadcn/ui Toggle - a two-state button that can be on or off.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::toggle::Toggle;
//!
//! let bold = RwSignal::new(false);
//!
//! Toggle::new(bold, "B")
//!     .variant(ToggleVariant::Outline);
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// Toggle variant for styling
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

/// Toggle size
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ToggleSize {
    Sm,
    #[default]
    Default,
    Lg,
}

// ============================================================================
// Toggle
// ============================================================================

/// A two-state toggle button
pub struct Toggle {
    id: ViewId,
    pressed: RwSignal<bool>,
    text: String,
    variant: ToggleVariant,
    size: ToggleSize,
    disabled: bool,
}

impl Toggle {
    /// Create a new toggle with pressed state and text
    pub fn new(pressed: RwSignal<bool>, text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            pressed,
            text: text.into(),
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            disabled: false,
        }
    }

    /// Set the variant
    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Use outline variant
    pub fn outline(mut self) -> Self {
        self.variant = ToggleVariant::Outline;
        self
    }

    /// Set the size
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    /// Use small size
    pub fn sm(mut self) -> Self {
        self.size = ToggleSize::Sm;
        self
    }

    /// Use large size
    pub fn lg(mut self) -> Self {
        self.size = ToggleSize::Lg;
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl HasViewId for Toggle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Toggle {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let pressed = self.pressed;
        let text = self.text;
        let variant = self.variant;
        let size = self.size;
        let disabled = self.disabled;

        let label = floem::views::Label::new(text).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let is_pressed = pressed.get();
                // Size-based padding and font
                let (px, py, font_size) = match size {
                    ToggleSize::Sm => (8.0, 6.0, 12.0),
                    ToggleSize::Default => (12.0, 8.0, 14.0),
                    ToggleSize::Lg => (16.0, 10.0, 16.0),
                };
                let base = s
                    .padding_left(px)
                    .padding_right(px)
                    .padding_top(py)
                    .padding_bottom(py)
                    .font_size(font_size)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .border_radius(t.radius)
                    .cursor(if disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    });
                // Apply variant and pressed state styling
                let styled = match variant {
                    ToggleVariant::Default => {
                        if is_pressed {
                            base.background(t.accent).color(t.accent_foreground)
                        } else {
                            base.background(floem::peniko::Color::TRANSPARENT)
                                .color(t.foreground)
                                .hover(|s| s.background(t.muted).color(t.muted_foreground))
                        }
                    }
                    ToggleVariant::Outline => {
                        if is_pressed {
                            base.background(t.accent)
                                .color(t.accent_foreground)
                                .border(1.0)
                                .border_color(t.input)
                        } else {
                            base.background(floem::peniko::Color::TRANSPARENT)
                                .color(t.foreground)
                                .border(1.0)
                                .border_color(t.input)
                                .hover(|s| s.background(t.accent).color(t.accent_foreground))
                        }
                    }
                };
                if disabled {
                    styled.color(t.muted_foreground)
                } else {
                    styled
                }
            })
        });

        if disabled {
            Box::new(label)
        } else {
            Box::new(label.on_click_stop(move |_| {
                pressed.update(|v| *v = !*v);
            }))
        }
    }
}

// ============================================================================
// ToggleCustom - Toggle with custom content
// ============================================================================

/// Toggle with custom content (e.g., icons)
pub struct ToggleCustom<V> {
    id: ViewId,
    pressed: RwSignal<bool>,
    child: V,
    variant: ToggleVariant,
    size: ToggleSize,
    disabled: bool,
}

impl<V: IntoView + 'static> ToggleCustom<V> {
    /// Create a new toggle with custom content
    pub fn new(pressed: RwSignal<bool>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            pressed,
            child,
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            disabled: false,
        }
    }

    /// Set the variant
    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Use outline variant
    pub fn outline(mut self) -> Self {
        self.variant = ToggleVariant::Outline;
        self
    }

    /// Set the size
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    /// Use small size
    pub fn sm(mut self) -> Self {
        self.size = ToggleSize::Sm;
        self
    }

    /// Use large size
    pub fn lg(mut self) -> Self {
        self.size = ToggleSize::Lg;
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<V: IntoView + 'static> HasViewId for ToggleCustom<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for ToggleCustom<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let pressed = self.pressed;
        let variant = self.variant;
        let size = self.size;
        let disabled = self.disabled;

        let container = floem::views::Container::new(self.child).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let is_pressed = pressed.get();
                // Size-based padding
                let (px, py) = match size {
                    ToggleSize::Sm => (8.0, 6.0),
                    ToggleSize::Default => (12.0, 8.0),
                    ToggleSize::Lg => (16.0, 10.0),
                };
                let base = s
                    .padding_left(px)
                    .padding_right(px)
                    .padding_top(py)
                    .padding_bottom(py)
                    .border_radius(t.radius)
                    .cursor(if disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    })
                    .display(floem::style::Display::Flex)
                    .items_center()
                    .justify_center();
                // Apply variant and pressed state styling
                match variant {
                    ToggleVariant::Default => {
                        if is_pressed {
                            base.background(t.accent)
                        } else {
                            base.background(floem::peniko::Color::TRANSPARENT)
                                .hover(|s| s.background(t.muted))
                        }
                    }
                    ToggleVariant::Outline => {
                        if is_pressed {
                            base.background(t.accent).border(1.0).border_color(t.input)
                        } else {
                            base.background(floem::peniko::Color::TRANSPARENT)
                                .border(1.0)
                                .border_color(t.input)
                                .hover(|s| s.background(t.accent))
                        }
                    }
                }
            })
        });

        if disabled {
            Box::new(container)
        } else {
            Box::new(container.on_click_stop(move |_| {
                pressed.update(|v| *v = !*v);
            }))
        }
    }
}
