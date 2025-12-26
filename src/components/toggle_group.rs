//! Toggle Group component with builder-style API
//!
//! Based on shadcn/ui Toggle Group - a group of toggle buttons where one or more can be selected.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::toggle_group::*;
//!
//! // Single selection
//! let selected = RwSignal::new(Some("left".to_string()));
//!
//! ToggleGroup::single(selected, (
//!     ToggleGroupItem::new("left", "Left"),
//!     ToggleGroupItem::new("center", "Center"),
//!     ToggleGroupItem::new("right", "Right"),
//! ));
//!
//! // Multiple selection
//! let selected = RwSignal::new(vec!["bold".to_string()]);
//!
//! ToggleGroup::multiple(selected, (
//!     ToggleGroupItem::new("bold", "B"),
//!     ToggleGroupItem::new("italic", "I"),
//!     ToggleGroupItem::new("underline", "U"),
//! ));
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// Toggle group variant
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ToggleGroupVariant {
    #[default]
    Default,
    Outline,
}

/// Toggle group size
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ToggleGroupSize {
    Sm,
    #[default]
    Default,
    Lg,
}

// ============================================================================
// ToggleGroup (Single Selection)
// ============================================================================

/// Toggle group with single selection
pub struct ToggleGroup<V> {
    id: ViewId,
    selected: RwSignal<Option<String>>,
    child: V,
    variant: ToggleGroupVariant,
    size: ToggleGroupSize,
}

impl<V: IntoView + 'static> ToggleGroup<V> {
    /// Create a toggle group with single selection
    pub fn single(selected: RwSignal<Option<String>>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            selected,
            child,
            variant: ToggleGroupVariant::Default,
            size: ToggleGroupSize::Default,
        }
    }

    /// Set the variant
    pub fn variant(mut self, variant: ToggleGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Use outline variant
    pub fn outline(mut self) -> Self {
        self.variant = ToggleGroupVariant::Outline;
        self
    }

    /// Set the size
    pub fn size(mut self, size: ToggleGroupSize) -> Self {
        self.size = size;
        self
    }
}

impl<V: IntoView + 'static> HasViewId for ToggleGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for ToggleGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let variant = self.variant;

        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let base = s
                        .display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Row)
                        .items_center()
                        .gap(1.0)
                        .border_radius(t.radius);
                    match variant {
                        ToggleGroupVariant::Default => base,
                        ToggleGroupVariant::Outline => {
                            base.border(1.0).border_color(t.input).padding(2.0)
                        }
                    }
                })
            }),
        )
    }
}

// ============================================================================
// ToggleGroupMultiple
// ============================================================================

/// Toggle group with multiple selection
pub struct ToggleGroupMultiple<V> {
    id: ViewId,
    selected: RwSignal<Vec<String>>,
    child: V,
    variant: ToggleGroupVariant,
    size: ToggleGroupSize,
}

impl<V: IntoView + 'static> ToggleGroupMultiple<V> {
    /// Create a toggle group with multiple selection
    pub fn new(selected: RwSignal<Vec<String>>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            selected,
            child,
            variant: ToggleGroupVariant::Default,
            size: ToggleGroupSize::Default,
        }
    }

    /// Set the variant
    pub fn variant(mut self, variant: ToggleGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Use outline variant
    pub fn outline(mut self) -> Self {
        self.variant = ToggleGroupVariant::Outline;
        self
    }

    /// Set the size
    pub fn size(mut self, size: ToggleGroupSize) -> Self {
        self.size = size;
        self
    }
}

impl<V: IntoView + 'static> HasViewId for ToggleGroupMultiple<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for ToggleGroupMultiple<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let variant = self.variant;

        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let base = s
                        .display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Row)
                        .items_center()
                        .gap(1.0)
                        .border_radius(t.radius);
                    match variant {
                        ToggleGroupVariant::Default => base,
                        ToggleGroupVariant::Outline => {
                            base.border(1.0).border_color(t.input).padding(2.0)
                        }
                    }
                })
            }),
        )
    }
}

// ============================================================================
// ToggleGroupItem (for single selection)
// ============================================================================

/// Individual item in a single-selection toggle group
pub struct ToggleGroupItem {
    id: ViewId,
    value: String,
    text: String,
    selected_signal: Option<RwSignal<Option<String>>>,
    disabled: bool,
}

impl ToggleGroupItem {
    /// Create a new toggle group item
    pub fn new(value: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            value: value.into(),
            text: text.into(),
            selected_signal: None,
            disabled: false,
        }
    }

    /// Set the selected signal (connects to parent group)
    pub fn selected(mut self, signal: RwSignal<Option<String>>) -> Self {
        self.selected_signal = Some(signal);
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl HasViewId for ToggleGroupItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ToggleGroupItem {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let value = self.value.clone();
        let text = self.text;
        let selected_signal = self.selected_signal;
        let disabled = self.disabled;
        let value_for_click = self.value.clone();

        let label = floem::views::Label::new(text).style(move |s| {
            let val = value.clone();
            s.with_shadcn_theme(move |s, t| {
                let is_selected = selected_signal
                    .map(|sig| sig.get().as_ref() == Some(&val))
                    .unwrap_or(false);
                let base = s
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .font_size(14.0)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .border_radius(t.radius)
                    .cursor(if disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    });
                if is_selected {
                    base.background(t.accent).color(t.accent_foreground)
                } else if disabled {
                    base.background(floem::peniko::Color::TRANSPARENT)
                        .color(t.muted_foreground)
                } else {
                    base.background(floem::peniko::Color::TRANSPARENT)
                        .color(t.foreground)
                        .hover(|s| s.background(t.muted).color(t.muted_foreground))
                }
            })
        });

        if disabled {
            Box::new(label)
        } else if let Some(signal) = selected_signal {
            Box::new(label.on_click_stop(move |_| {
                signal.update(|v| *v = Some(value_for_click.clone()));
            }))
        } else {
            Box::new(label)
        }
    }
}

// ============================================================================
// ToggleGroupItemMultiple (for multiple selection)
// ============================================================================

/// Individual item in a multiple-selection toggle group
pub struct ToggleGroupItemMultiple {
    id: ViewId,
    value: String,
    text: String,
    selected_signal: Option<RwSignal<Vec<String>>>,
    disabled: bool,
}

impl ToggleGroupItemMultiple {
    /// Create a new toggle group item for multiple selection
    pub fn new(value: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            value: value.into(),
            text: text.into(),
            selected_signal: None,
            disabled: false,
        }
    }

    /// Set the selected signal (connects to parent group)
    pub fn selected(mut self, signal: RwSignal<Vec<String>>) -> Self {
        self.selected_signal = Some(signal);
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl HasViewId for ToggleGroupItemMultiple {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ToggleGroupItemMultiple {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let value = self.value.clone();
        let text = self.text;
        let selected_signal = self.selected_signal;
        let disabled = self.disabled;
        let value_for_click = self.value.clone();

        let label = floem::views::Label::new(text).style(move |s| {
            let val = value.clone();
            s.with_shadcn_theme(move |s, t| {
                let is_selected = selected_signal
                    .map(|sig| sig.get().contains(&val))
                    .unwrap_or(false);
                let base = s
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .font_size(14.0)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .border_radius(t.radius)
                    .cursor(if disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    });
                if is_selected {
                    base.background(t.accent).color(t.accent_foreground)
                } else if disabled {
                    base.background(floem::peniko::Color::TRANSPARENT)
                        .color(t.muted_foreground)
                } else {
                    base.background(floem::peniko::Color::TRANSPARENT)
                        .color(t.foreground)
                        .hover(|s| s.background(t.muted).color(t.muted_foreground))
                }
            })
        });

        if disabled {
            Box::new(label)
        } else if let Some(signal) = selected_signal {
            Box::new(label.on_click_stop(move |_| {
                signal.update(|v| {
                    if v.contains(&value_for_click) {
                        v.retain(|x| x != &value_for_click);
                    } else {
                        v.push(value_for_click.clone());
                    }
                });
            }))
        } else {
            Box::new(label)
        }
    }
}
