//! Button Group component – groups related buttons with connected styling
//!
//! Based on shadcn/ui ButtonGroup.
//!
//! # Example
//!
//! ```rust
//! use floem::view::ParentView;
//! use floem_shadcn::components::button::Button;
//! use floem_shadcn::components::button_group::{ButtonGroup, ButtonGroupSeparator};
//!
//! ButtonGroup::new()
//!     .child(Button::new("Left"))
//!     .child(ButtonGroupSeparator::new())
//!     .child(Button::new("Right"));
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

/// A container that groups buttons horizontally with no gap between them.
pub struct ButtonGroup {
    id: ViewId,
}
impl ButtonGroup {
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}
impl Default for ButtonGroup {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ButtonGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ButtonGroup {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| s.flex_row().items_center())
    }
}
impl ParentView for ButtonGroup {}

/// A vertical separator line placed between buttons in a group.
pub struct ButtonGroupSeparator {
    id: ViewId,
}
impl ButtonGroupSeparator {
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}
impl Default for ButtonGroupSeparator {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ButtonGroupSeparator {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ButtonGroupSeparator {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.w_px()
                .h_full()
                .with_shadcn_theme(|s, t| s.background(t.border))
        }))
    }
}
