//! Input Group component – input with adornments
//!
//! Based on shadcn/ui InputGroup.
//!
//! # Example
//!
//! ```rust
//! use floem::view::ParentView;
//! use floem_shadcn::components::input::Input;
//! use floem_shadcn::components::input_group::{InputGroup, InputGroupAddon, AddonPosition};
//!
//! InputGroup::new()
//!     .child(InputGroupAddon::new(AddonPosition::Prefix, label("$")))
//!     .child(Input::new());
//! ```

use floem::prelude::*;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// Position of an addon within an input group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddonPosition {
    Prefix,
    Suffix,
}

pub struct InputGroup {
    id: ViewId,
    #[allow(dead_code)]
    disabled: bool,
}
impl InputGroup {
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            disabled: false,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
impl Default for InputGroup {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for InputGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for InputGroup {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| s.flex_row().items_center())
    }
}
impl ParentView for InputGroup {}

pub struct InputGroupAddon<V> {
    id: ViewId,
    #[allow(dead_code)]
    position: AddonPosition,
    child: V,
}
impl<V: IntoView + 'static> InputGroupAddon<V> {
    pub fn new(position: AddonPosition, child: V) -> Self {
        Self {
            id: ViewId::new(),
            position,
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for InputGroupAddon<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for InputGroupAddon<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.padding_left(8.0)
                        .padding_right(8.0)
                        .padding_top(6.0)
                        .padding_bottom(6.0)
                        .font_size(14.0)
                        .background(t.muted)
                        .color(t.muted_foreground)
                        .border(1.0)
                        .border_color(t.input)
                })
            }),
        )
    }
}
