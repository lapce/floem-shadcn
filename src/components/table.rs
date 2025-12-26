//! Table component with builder-style API
//!
//! Based on shadcn/ui Table - a responsive table component.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::table::*;
//!
//! Table::new((
//!     TableHeader::new(
//!         TableRow::new((
//!             TableHead::new("Name"),
//!             TableHead::new("Email"),
//!             TableHead::new("Status"),
//!         ))
//!     ),
//!     TableBody::new((
//!         TableRow::new((
//!             TableCell::new("John Doe"),
//!             TableCell::new("john@example.com"),
//!             TableCell::new("Active"),
//!         )),
//!         TableRow::new((
//!             TableCell::new("Jane Smith"),
//!             TableCell::new("jane@example.com"),
//!             TableCell::new("Pending"),
//!         )),
//!     )),
//! ));
//! ```

use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Table
// ============================================================================

/// Table container
pub struct Table<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> Table<V> {
    /// Create a new table
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for Table<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Table<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width_full()
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(t.radius)
                })
            }),
        )
    }
}

// ============================================================================
// TableHeader
// ============================================================================

/// Table header section (thead)
pub struct TableHeader<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> TableHeader<V> {
    /// Create a new table header
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for TableHeader<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl HasViewId for TableHead {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for TableHeader<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.with_shadcn_theme(move |s, t| s.width_full().background(t.muted))),
        )
    }
}

// ============================================================================
// TableBody
// ============================================================================

/// Table body section (tbody)
pub struct TableBody<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> TableBody<V> {
    /// Create a new table body
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for TableBody<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for TableBody<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.width_full()
                    .display(floem::style::Display::Flex)
                    .flex_direction(floem::style::FlexDirection::Column)
            }),
        )
    }
}

// ============================================================================
// TableFooter
// ============================================================================

/// Table footer section (tfoot)
pub struct TableFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> TableFooter<V> {
    /// Create a new table footer
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for TableFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for TableFooter<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width_full()
                        .background(t.muted)
                        .border_top(1.0)
                        .border_color(t.border)
                })
            }),
        )
    }
}

// ============================================================================
// TableRow
// ============================================================================

/// Table row (tr)
pub struct TableRow<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> TableRow<V> {
    /// Create a new table row
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for TableRow<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for TableRow<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width_full()
                        .display(floem::style::Display::Flex)
                        .flex_direction(floem::style::FlexDirection::Row)
                        .border_bottom(1.0)
                        .border_color(t.border)
                        .hover(|s| s.background(t.muted.with_alpha(0.5)))
                })
            }),
        )
    }
}

// ============================================================================
// TableHead
// ============================================================================

/// Table header cell (th)
pub struct TableHead {
    id: ViewId,
    text: String,
    width: Option<f64>,
}

impl TableHead {
    /// Create a new table header cell
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            width: None,
        }
    }

    /// Set fixed width for the column
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}

impl IntoView for TableHead {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        let width = self.width;

        Box::new(floem::views::Label::with_id(self.id, text).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .padding(12.0)
                    .font_size(14.0)
                    .font_weight(floem::text::Weight::MEDIUM)
                    .color(t.muted_foreground)
                    .flex_grow(1.0);
                if let Some(w) = width {
                    base.width(w).flex_grow(0.0)
                } else {
                    base
                }
            })
        }))
    }
}

// ============================================================================
// TableHeadCustom
// ============================================================================

/// Table header cell with custom content
pub struct TableHeadCustom<V> {
    id: ViewId,
    child: V,
    width: Option<f64>,
}

impl<V: IntoView + 'static> TableHeadCustom<V> {
    /// Create a new table header cell with custom content
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            width: None,
        }
    }

    /// Set fixed width for the column
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}

impl<V: IntoView + 'static> HasViewId for TableHeadCustom<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for TableHeadCustom<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let width = self.width;

        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let base = s
                        .padding(12.0)
                        .font_size(14.0)
                        .font_weight(floem::text::Weight::MEDIUM)
                        .color(t.muted_foreground)
                        .flex_grow(1.0);
                    if let Some(w) = width {
                        base.width(w).flex_grow(0.0)
                    } else {
                        base
                    }
                })
            }),
        )
    }
}

// ============================================================================
// TableCell
// ============================================================================

/// Table data cell (td)
pub struct TableCell {
    id: ViewId,
    text: String,
    width: Option<f64>,
}

impl TableCell {
    /// Create a new table cell
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            width: None,
        }
    }

    /// Set fixed width for the cell
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}

impl HasViewId for TableCell {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for TableCell {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        let width = self.width;

        Box::new(floem::views::Label::with_id(self.id, text).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .padding(12.0)
                    .font_size(14.0)
                    .color(t.foreground)
                    .flex_grow(1.0);
                if let Some(w) = width {
                    base.width(w).flex_grow(0.0)
                } else {
                    base
                }
            })
        }))
    }
}

// ============================================================================
// TableCellCustom
// ============================================================================

/// Table data cell with custom content
pub struct TableCellCustom<V> {
    id: ViewId,
    child: V,
    width: Option<f64>,
}

impl<V: IntoView + 'static> TableCellCustom<V> {
    /// Create a new table cell with custom content
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
            width: None,
        }
    }

    /// Set fixed width for the cell
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}

impl<V: IntoView + 'static> HasViewId for TableCellCustom<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for TableCellCustom<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let width = self.width;

        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let base = s
                        .padding(12.0)
                        .font_size(14.0)
                        .color(t.foreground)
                        .flex_grow(1.0);
                    if let Some(w) = width {
                        base.width(w).flex_grow(0.0)
                    } else {
                        base
                    }
                })
            }),
        )
    }
}

// ============================================================================
// TableCaption
// ============================================================================

/// Table caption
pub struct TableCaption {
    id: ViewId,
    text: String,
}

impl TableCaption {
    /// Create a new table caption
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for TableCaption {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for TableCaption {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let text = self.text;

        Box::new(floem::views::Label::with_id(self.id, text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.padding(12.0)
                    .font_size(14.0)
                    .color(t.muted_foreground)
                    .justify_center()
                    .width_full()
            })
        }))
    }
}
