//! Item component – flexible list item container
//!
//! Based on shadcn/ui Item.
//!
//! # Example
//!
//! ```rust
//! use floem::view::ParentView;
//! use floem_shadcn::components::item::*;
//!
//! Item::new()
//!     .child(ItemContent::new()
//!         .child(ItemTitle::new("Settings"))
//!         .child(ItemDescription::new("Manage your preferences")));
//! ```

use crate::styled::ShadcnStyleExt;
use floem::prelude::*;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

/// Main item container – renders as a horizontal row.
pub struct Item {
    id: ViewId,
}
impl Item {
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}
impl Default for Item {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for Item {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for Item {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ())
            .style(|s| s.flex_row().items_center().gap_3().w_full().py_3().px_4())
    }
}
impl ParentView for Item {}

/// Content area inside an Item.
pub struct ItemContent {
    id: ViewId,
}
impl ItemContent {
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}
impl Default for ItemContent {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ItemContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemContent {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| s.flex_grow(1.0).flex_col().gap_0p5())
    }
}
impl ParentView for ItemContent {}

/// Title text inside an ItemContent.
pub struct ItemTitle {
    id: ViewId,
    text: String,
}
impl ItemTitle {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for ItemTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemTitle {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| s.text_sm().text_foreground()))
    }
}

/// Description text inside an ItemContent.
pub struct ItemDescription {
    id: ViewId,
    text: String,
}
impl ItemDescription {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for ItemDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemDescription {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| s.text_xs().text_muted_foreground()))
    }
}
