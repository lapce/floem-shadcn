//! Empty state component – placeholder for empty content
//!
//! Based on shadcn/ui Empty.
//!
//! # Example
//!
//! ```rust
//! use floem::view::ParentView;
//! use floem_shadcn::components::empty::*;
//!
//! Empty::new()
//!     .child(EmptyMedia::new().child(svg_icon))
//!     .child(EmptyContent::new()
//!         .child(EmptyTitle::new("No results"))
//!         .child(EmptyDescription::new("Try adjusting your search")));
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

/// Main empty state container – centers its children vertically.
pub struct Empty {
    id: ViewId,
}
impl Empty {
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}
impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for Empty {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for Empty {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ())
            .style(|s| s.flex_col().items_center().justify_center().w_full().py_6())
    }
}
impl ParentView for Empty {}

/// Media slot for the empty state (icons, images).
pub struct EmptyMedia {
    id: ViewId,
}
impl EmptyMedia {
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}
impl Default for EmptyMedia {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for EmptyMedia {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyMedia {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| s.mb_4())
    }
}
impl ParentView for EmptyMedia {}

/// Content slot for the empty state (title, description, actions).
pub struct EmptyContent {
    id: ViewId,
}
impl EmptyContent {
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}
impl Default for EmptyContent {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for EmptyContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyContent {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ())
            .style(|s| s.flex_col().items_center().max_width(420.0).gap_1())
    }
}
impl ParentView for EmptyContent {}

/// Title text inside the empty state.
pub struct EmptyTitle {
    id: ViewId,
    text: String,
}
impl EmptyTitle {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for EmptyTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyTitle {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| s.with_shadcn_theme(|s, t| s.text_lg().color(t.foreground))),
        )
    }
}

/// Description text inside the empty state.
pub struct EmptyDescription {
    id: ViewId,
    text: String,
}
impl EmptyDescription {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for EmptyDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyDescription {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| s.with_shadcn_theme(|s, t| s.text_sm().color(t.muted_foreground))),
        )
    }
}

/// Actions slot for the empty state (buttons, links).
pub struct EmptyActions {
    id: ViewId,
}
impl EmptyActions {
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}
impl Default for EmptyActions {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for EmptyActions {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyActions {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| s.flex_row().gap_2().mt_2())
    }
}
impl ParentView for EmptyActions {}
