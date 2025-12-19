//! Pagination component with builder-style API
//!
//! Based on shadcn/ui Pagination - navigation controls for paged content.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::pagination::*;
//!
//! let page = RwSignal::new(1);
//! let total_pages = 10;
//!
//! Pagination::new(page, total_pages);
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Pagination
// ============================================================================

/// Pagination navigation component
pub struct Pagination {
    id: ViewId,
    current_page: RwSignal<usize>,
    total_pages: usize,
    show_edges: bool,
    siblings: usize,
}

impl Pagination {
    /// Create a new pagination component
    pub fn new(current_page: RwSignal<usize>, total_pages: usize) -> Self { Self { id: ViewId::new(),
            current_page,
            total_pages,
            show_edges: true,
            siblings: 1,
        }
    }

    /// Show/hide first and last page buttons
    pub fn show_edges(mut self, show: bool) -> Self { self.show_edges = show;
        self
    }

    /// Number of siblings on each side of current page
    pub fn siblings(mut self, count: usize) -> Self { self.siblings = count;
        self
    }
}


impl HasViewId for Pagination {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Pagination {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let current_page = self.current_page;
        let total_pages = self.total_pages;
        let show_edges = self.show_edges;
        let siblings = self.siblings;

        // Previous button
        let prev_btn = floem::views::Label::new("◀")
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let page = current_page.get();
                    let disabled = page <= 1;
                    let base = s
                        .width(36.0)
                        .height(36.0)
                        .font_size(14.0)
                        .display(floem::style::Display::Flex)
                        .items_center()
                        .justify_center()
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(t.radius)
                        .cursor(if disabled {
                            CursorStyle::Default
                        } else {
                            CursorStyle::Pointer
                        });
                    if disabled {
                        base.color(t.muted_foreground).background(t.muted)
                    } else {
                        base.color(t.foreground)
                            .background(t.background)
                            .hover(|s| s.background(t.accent))
                    }
                })
            })
            .on_click_stop(move |_| {
                let page = current_page.get();
                if page > 1 {
                    current_page.set(page - 1);
                }
            });

        // Page numbers - create a static set of page buttons
        let page1 = create_page_button(current_page, 1, total_pages, siblings, show_edges);
        let page2 = create_page_button(current_page, 2, total_pages, siblings, show_edges);
        let page3 = create_page_button(current_page, 3, total_pages, siblings, show_edges);
        let page4 = create_page_button(current_page, 4, total_pages, siblings, show_edges);
        let page5 = create_page_button(current_page, 5, total_pages, siblings, show_edges);
        let page6 = create_page_button(current_page, 6, total_pages, siblings, show_edges);
        let page7 = create_page_button(current_page, 7, total_pages, siblings, show_edges);

        // Next button
        let next_btn = floem::views::Label::new("▶")
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let page = current_page.get();
                    let disabled = page >= total_pages;
                    let base = s
                        .width(36.0)
                        .height(36.0)
                        .font_size(14.0)
                        .display(floem::style::Display::Flex)
                        .items_center()
                        .justify_center()
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(t.radius)
                        .cursor(if disabled {
                            CursorStyle::Default
                        } else {
                            CursorStyle::Pointer
                        });
                    if disabled {
                        base.color(t.muted_foreground).background(t.muted)
                    } else {
                        base.color(t.foreground)
                            .background(t.background)
                            .hover(|s| s.background(t.accent))
                    }
                })
            })
            .on_click_stop(move |_| {
                let page = current_page.get();
                if page < total_pages {
                    current_page.set(page + 1);
                }
            });

        Box::new(
            floem::views::h_stack((
                prev_btn, page1, page2, page3, page4, page5, page6, page7, next_btn,
            ))
            .style(|s| s.gap(4.0).items_center()),
        )
    }
}

fn create_page_button(
    current_page: RwSignal<usize>,
    page_num: usize,
    total_pages: usize,
    siblings: usize,
    _show_edges: bool,
) -> impl IntoView {
    floem::views::Label::derived(move || {
        let current = current_page.get();

        // Calculate which page number to show in this slot
        let display_page = calculate_display_page(page_num, current, total_pages, siblings);

        if display_page == 0 {
            "...".to_string()
        } else if display_page > total_pages {
            String::new()
        } else {
            display_page.to_string()
        }
    })
    .style(move |s| {
        s.with_shadcn_theme(move |s, t| {
            let current = current_page.get();
            let display_page = calculate_display_page(page_num, current, total_pages, siblings);
            let is_current = display_page == current && display_page > 0;
            let is_ellipsis = display_page == 0;
            let is_hidden = display_page > total_pages;
            let base = s
                .min_width(36.0)
                .height(36.0)
                .padding_left(12.0)
                .padding_right(12.0)
                .font_size(14.0)
                .display(floem::style::Display::Flex)
                .items_center()
                .justify_center()
                .border_radius(t.radius);
            if is_hidden {
                base.display(floem::style::Display::None)
            } else if is_ellipsis {
                base.color(t.muted_foreground)
            } else if is_current {
                base.background(t.primary)
                    .color(t.primary_foreground)
                    .border(1.0)
                    .border_color(t.primary)
            } else {
                base.color(t.foreground)
                    .border(1.0)
                    .border_color(t.border)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.background(t.accent))
            }
        })
    })
    .on_click_stop(move |_| {
        let current = current_page.get();
        let display_page = calculate_display_page(page_num, current, total_pages, siblings);

        if display_page > 0 && display_page <= total_pages && display_page != current {
            current_page.set(display_page);
        }
    })
}

fn calculate_display_page(slot: usize, current: usize, total: usize, siblings: usize) -> usize {
    // For small page counts, just show all pages
    if total <= 7 {
        return slot;
    }

    // Calculate the range of pages to show around current
    let start = if current <= 3 + siblings {
        1
    } else if current >= total - 2 - siblings {
        total - 6
    } else {
        current - siblings - 1
    };

    let page = start + slot - 1;

    // Handle ellipsis
    if slot == 2 && start > 2 {
        return 0; // Show ellipsis
    }
    if slot == 6 && start + 5 < total - 1 {
        return 0; // Show ellipsis
    }

    // Show first page in slot 1
    if slot == 1 && start > 1 {
        return 1;
    }

    // Show last page in slot 7
    if slot == 7 && page < total {
        return total;
    }

    page
}

// ============================================================================
// PaginationContent
// ============================================================================

/// Container for pagination items
pub struct PaginationContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> PaginationContent<V> {
    /// Create new pagination content
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for PaginationContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for PaginationContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Row)
                .gap(4.0)
                .items_center()
        }))
    }
}

// ============================================================================
// PaginationItem
// ============================================================================

/// Individual pagination item
pub struct PaginationItem<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> PaginationItem<V> {
    /// Create a new item
    pub fn new(child: V) -> Self { Self { id: ViewId::new(), child }
    }
}


impl<V: IntoView + 'static> HasViewId for PaginationItem<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for PaginationItem<V> {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child))
    }
}

// ============================================================================
// PaginationLink
// ============================================================================

/// Page number link
pub struct PaginationLink {
    id: ViewId,
    page: usize,
    current_page: RwSignal<usize>,
    is_active: bool,
}

impl PaginationLink {
    /// Create a new page link
    pub fn new(page: usize, current_page: RwSignal<usize>) -> Self { Self { id: ViewId::new(),
            page,
            current_page,
            is_active: false,
        }
    }

    /// Mark as active/current
    pub fn active(mut self, is_active: bool) -> Self { self.is_active = is_active;
        self
    }
}


impl HasViewId for PaginationLink {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for PaginationLink {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let page = self.page;
        let current_page = self.current_page;
        let is_active = self.is_active;

        Box::new(
            floem::views::Label::new(page.to_string())
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let base = s
                            .min_width(36.0)
                            .height(36.0)
                            .padding_left(12.0)
                            .padding_right(12.0)
                            .font_size(14.0)
                            .display(floem::style::Display::Flex)
                            .items_center()
                            .justify_center()
                            .border(1.0)
                            .border_radius(t.radius)
                            .cursor(CursorStyle::Pointer);
                        if is_active {
                            base.background(t.primary)
                                .color(t.primary_foreground)
                                .border_color(t.primary)
                        } else {
                            base.background(t.background)
                                .color(t.foreground)
                                .border_color(t.border)
                                .hover(|s| s.background(t.accent))
                        }
                    })
                })
                .on_click_stop(move |_| {
                    current_page.set(page);
                }),
        )
    }
}

// ============================================================================
// PaginationPrevious
// ============================================================================

/// Previous page button
pub struct PaginationPrevious {
    id: ViewId,
    current_page: RwSignal<usize>,
    label: String,
}

impl PaginationPrevious {
    /// Create a new previous button
    pub fn new(current_page: RwSignal<usize>) -> Self { Self { id: ViewId::new(),
            current_page,
            label: "Previous".to_string(),
        }
    }

    /// Set custom label
    pub fn label(mut self, label: impl Into<String>) -> Self { self.label = label.into();
        self
    }
}


impl HasViewId for PaginationPrevious {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for PaginationPrevious {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let current_page = self.current_page;
        let label = self.label;

        Box::new(
            floem::views::h_stack((
                floem::views::Label::new("◀").style(|s| s.font_size(12.0)),
                floem::views::Label::new(label),
            ))
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let page = current_page.get();
                    let disabled = page <= 1;
                    let base = s
                        .gap(4.0)
                        .padding_left(12.0)
                        .padding_right(12.0)
                        .height(36.0)
                        .font_size(14.0)
                        .items_center()
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(t.radius)
                        .cursor(if disabled {
                            CursorStyle::Default
                        } else {
                            CursorStyle::Pointer
                        });
                    if disabled {
                        base.color(t.muted_foreground).background(t.muted)
                    } else {
                        base.color(t.foreground)
                            .background(t.background)
                            .hover(|s| s.background(t.accent))
                    }
                })
            })
            .on_click_stop(move |_| {
                let page = current_page.get();
                if page > 1 {
                    current_page.set(page - 1);
                }
            }),
        )
    }
}

// ============================================================================
// PaginationNext
// ============================================================================

/// Next page button
pub struct PaginationNext {
    id: ViewId,
    current_page: RwSignal<usize>,
    total_pages: usize,
    label: String,
}

impl PaginationNext {
    /// Create a new next button
    pub fn new(current_page: RwSignal<usize>, total_pages: usize) -> Self { Self { id: ViewId::new(),
            current_page,
            total_pages,
            label: "Next".to_string(),
        }
    }

    /// Set custom label
    pub fn label(mut self, label: impl Into<String>) -> Self { self.label = label.into();
        self
    }
}


impl HasViewId for PaginationNext {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for PaginationNext {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let current_page = self.current_page;
        let total_pages = self.total_pages;
        let label = self.label;

        Box::new(
            floem::views::h_stack((
                floem::views::Label::new(label),
                floem::views::Label::new("▶").style(|s| s.font_size(12.0)),
            ))
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let page = current_page.get();
                    let disabled = page >= total_pages;
                    let base = s
                        .gap(4.0)
                        .padding_left(12.0)
                        .padding_right(12.0)
                        .height(36.0)
                        .font_size(14.0)
                        .items_center()
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(t.radius)
                        .cursor(if disabled {
                            CursorStyle::Default
                        } else {
                            CursorStyle::Pointer
                        });
                    if disabled {
                        base.color(t.muted_foreground).background(t.muted)
                    } else {
                        base.color(t.foreground)
                            .background(t.background)
                            .hover(|s| s.background(t.accent))
                    }
                })
            })
            .on_click_stop(move |_| {
                let page = current_page.get();
                if page < total_pages {
                    current_page.set(page + 1);
                }
            }),
        )
    }
}

// ============================================================================
// PaginationEllipsis
// ============================================================================

/// Ellipsis indicator for skipped pages
pub struct PaginationEllipsis;

impl PaginationEllipsis {
    /// Create a new ellipsis
    pub fn new() -> Self {
        Self
    }
}

impl Default for PaginationEllipsis {
    fn default() -> Self {
        Self::new()
    }
}


impl HasViewId for PaginationEllipsis {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}

impl IntoView for PaginationEllipsis {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Label::new("...").style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.min_width(36.0)
                    .height(36.0)
                    .font_size(14.0)
                    .color(t.muted_foreground)
                    .display(floem::style::Display::Flex)
                    .items_center()
                    .justify_center()
            })
        }))
    }
}
