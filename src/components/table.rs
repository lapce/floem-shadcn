//! Table component with builder-style API
//!
//! Based on shadcn/ui Table - a responsive table component.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::table::*;
//! use floem::view::ParentView;
//!
//! Table::new()
//!     .child(
//!         TableHeader::new().child(
//!             TableRow::new()
//!                 .child(TableHead::new("Name"))
//!                 .child(TableHead::new("Email"))
//!                 .child(TableHead::new("Status"))
//!         )
//!     )
//!     .child(
//!         TableBody::new()
//!             .child(
//!                 TableRow::new()
//!                     .child(TableCell::new("John Doe"))
//!                     .child(TableCell::new("john@example.com"))
//!                     .child(TableCell::new("Active"))
//!             )
//!             .child(
//!                 TableRow::new()
//!                     .child(TableCell::new("Jane Smith"))
//!                     .child(TableCell::new("jane@example.com"))
//!                     .child(TableCell::new("Pending"))
//!             )
//!     )
//!     .into_view();
//! ```

use floem::prelude::*;
use floem::taffy::{
    GridAutoFlow,
    geometry::MinMax,
    style::{MaxTrackSizingFunction, MinTrackSizingFunction},
};
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Table
// ============================================================================

/// Table container
pub struct Table {
    id: ViewId,
}

impl Table {
    /// Create a new table
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl HasViewId for Table {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Table {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.width_full()
                    .grid()
                    .grid_auto_flow(GridAutoFlow::Row)
                    .font_size(14.0)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius)
            })
        })
    }
}

impl ParentView for Table {}

// ============================================================================
// TableHeader
// ============================================================================

/// Table header section (thead)
pub struct TableHeader {
    id: ViewId,
}

impl TableHeader {
    /// Create a new table header
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl HasViewId for TableHeader {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for TableHeader {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| s.width_full().grid())
    }
}

impl ParentView for TableHeader {}

// ============================================================================
// TableBody
// ============================================================================

/// Table body section (tbody)
pub struct TableBody {
    id: ViewId,
}

impl TableBody {
    /// Create a new table body
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl HasViewId for TableBody {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for TableBody {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| s.width_full().grid())
    }
}

impl ParentView for TableBody {}

// ============================================================================
// TableFooter
// ============================================================================

/// Table footer section (tfoot)
pub struct TableFooter {
    id: ViewId,
}

impl TableFooter {
    /// Create a new table footer
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl HasViewId for TableFooter {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for TableFooter {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.width_full()
                    .grid()
                    .font_weight(floem::text::Weight::MEDIUM)
                    .background(t.muted.with_alpha(0.5))
                    .border_top(1.0)
                    .border_color(t.border)
            })
        })
    }
}

impl ParentView for TableFooter {}

// ============================================================================
// TableRow
// ============================================================================

/// Table row (tr)
pub struct TableRow {
    id: ViewId,
}

impl TableRow {
    /// Create a new table row
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl HasViewId for TableRow {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for TableRow {
    type V = floem::views::Stem;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        floem::views::Stem::with_id(self.id).style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.width_full()
                    .grid() // Use Grid for cells
                    .grid_auto_flow(GridAutoFlow::Column) // Cells flow horizontally
                    .grid_auto_columns(vec![MinMax {
                        min: MinTrackSizingFunction::length(0.0), // No minimum - use fr for distribution
                        max: MaxTrackSizingFunction::fr(1.0),     // Equal column widths
                    }]) // Each cell gets equal width using 1fr
                    .border_bottom(1.0)
                    .border_color(t.border)
                    .hover(|s| s.background(t.muted.with_alpha(0.5))) // hover:bg-muted/50
                    .transition(
                        floem::style::Background,
                        floem::style::Transition::linear(std::time::Duration::from_millis(150)),
                    )
            })
        })
    }
}

impl ParentView for TableRow {}

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

impl HasViewId for TableHead {
    fn view_id(&self) -> ViewId {
        self.id
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
                    .height(40.0) // h-10
                    .padding_horiz(8.0) // px-2
                    .font_size(14.0) // text-sm
                    .font_weight(floem::text::Weight::MEDIUM) // font-medium
                    .color(t.foreground) // text-foreground
                    .items_center(); // align-middle vertically
                if let Some(w) = width {
                    base.width(w)
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
                        .height(40.0) // h-10
                        .padding_horiz(8.0) // px-2
                        .font_size(14.0) // text-sm
                        .font_weight(floem::text::Weight::MEDIUM) // font-medium
                        .color(t.foreground) // text-foreground
                        .items_center(); // align-middle vertically
                    if let Some(w) = width {
                        base.width(w)
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
                    .padding(8.0) // p-2
                    .font_size(14.0) // text-sm
                    .color(t.foreground) // text-foreground
                    .items_center(); // align-middle
                if let Some(w) = width {
                    base.width(w)
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
                        .padding(8.0) // p-2
                        .font_size(14.0) // text-sm
                        .color(t.foreground) // text-foreground
                        .items_center(); // align-middle
                    if let Some(w) = width {
                        base.width(w)
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
                s.margin_top(16.0) // mt-4
                    .font_size(14.0) // text-sm
                    .color(t.muted_foreground) // text-muted-foreground
                    .width_full()
            })
        }))
    }
}
