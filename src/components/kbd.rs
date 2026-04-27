//! Kbd component – keyboard shortcut indicator
//!
//! Based on shadcn/ui Kbd.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::kbd::Kbd;
//!
//! let shortcut = Kbd::new("⌘K");
//!
//! // Multiple keys in a group
//! let shortcut = KbdGroup::new(vec!["⌘", "Shift", "K"]);
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

/// Renders a keyboard key (or key combination) with styled border and background.
pub struct Kbd {
    id: ViewId,
    key: String,
}
impl Kbd {
    pub fn new(k: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            key: k.into(),
        }
    }
}
impl HasViewId for Kbd {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for Kbd {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Label::new(self.key).style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.padding_left(4.0)
                    .padding_right(4.0)
                    .padding_top(2.0)
                    .padding_bottom(2.0)
                    .font_size(11.0)
                    .background(t.muted)
                    .color(t.muted_foreground)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius_sm)
                    .line_height(1.0)
            })
        }))
    }
}

/// A group of Kbd elements displayed inline with "+" separator spacing.
///
/// # Example
///
/// ```rust
/// let group = KbdGroup::new(vec!["⌘", "Shift", "K"]);
/// ```
pub struct KbdGroup {
    id: ViewId,
    keys: Vec<String>,
}

impl KbdGroup {
    /// Create a group of keyboard shortcuts to display inline.
    ///
    /// The keys are joined with "+" separators for a clean inline look,
    /// or displayed as individual `Kbd` elements with spacing.
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            id: ViewId::new(),
            keys,
        }
    }
}

impl HasViewId for KbdGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for KbdGroup {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        if self.keys.is_empty() {
            return Box::new(floem::views::Empty::new());
        }

        let len = self.keys.len();
        let children: Vec<Box<dyn View>> = self
            .keys
            .into_iter()
            .enumerate()
            .map(|(i, key)| {
                let kbd = Kbd::new(key);
                let kbd_view = kbd.into_view();
                // Add "+" separator between keys (but not after the last one)
                if i < len - 1 {
                    let separator = floem::views::Label::new("+")
                        .style(|s| s.font_size(11.0).padding_left(2.0).padding_right(2.0));
                    let row = floem::views::Stack::horizontal((kbd_view, separator))
                        .style(|s| s.items_center());
                    Box::new(row)
                } else {
                    kbd_view
                }
            })
            .collect();

        Box::new(
            floem::views::Stack::horizontal_from_iter(children)
                .style(|s| s.items_center().gap(1.0)),
        )
    }
}
