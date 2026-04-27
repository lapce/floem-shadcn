//! Pagination component with builder-style API
//! (tailwind-enhanced – complete file)

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

// Pagination
pub struct Pagination {
    id: ViewId,
    current_page: RwSignal<usize>,
    total_pages: usize,
    show_edges: bool,
    siblings: usize,
}
impl Pagination {
    pub fn new(current_page: RwSignal<usize>, total_pages: usize) -> Self {
        Self {
            id: ViewId::new(),
            current_page,
            total_pages,
            show_edges: true,
            siblings: 1,
        }
    }
    pub fn show_edges(mut self, show: bool) -> Self {
        self.show_edges = show;
        self
    }
    pub fn siblings(mut self, count: usize) -> Self {
        self.siblings = count;
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let current_page = self.current_page;
        let total_pages = self.total_pages;
        let prev_btn = floem::views::Label::new("◀")
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let page = current_page.get();
                    let disabled = page <= 1;
                    let base = s
                        .w_9()
                        .h_9()
                        .text_sm()
                        .flex()
                        .items_center()
                        .justify_center()
                        .border_1()
                        .border_color(t.border)
                        .rounded_md()
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
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                let page = current_page.get();
                if page > 1 {
                    current_page.set(page - 1);
                }
            });
        let next_btn = floem::views::Label::new("▶")
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let page = current_page.get();
                    let disabled = page >= total_pages;
                    let base = s
                        .w_9()
                        .h_9()
                        .text_sm()
                        .flex()
                        .items_center()
                        .justify_center()
                        .border_1()
                        .border_color(t.border)
                        .rounded_md()
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
            .on_event_stop(floem::event::listener::Click, move |_, _| {
                let page = current_page.get();
                if page < total_pages {
                    current_page.set(page + 1);
                }
            });
        Box::new(
            floem::views::Stack::horizontal((prev_btn, next_btn))
                .style(|s| s.gap_1().items_center()),
        )
    }
}

// PaginationContent
pub struct PaginationContent<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> PaginationContent<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for PaginationContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for PaginationContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.flex_row().gap_1().items_center()),
        )
    }
}

// PaginationItem
pub struct PaginationItem<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> PaginationItem<V> {
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for PaginationItem<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for PaginationItem<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child))
    }
}

// PaginationLink
pub struct PaginationLink {
    id: ViewId,
    page: usize,
    current_page: RwSignal<usize>,
    is_active: bool,
}
impl PaginationLink {
    pub fn new(page: usize, current_page: RwSignal<usize>) -> Self {
        Self {
            id: ViewId::new(),
            page,
            current_page,
            is_active: false,
        }
    }
    pub fn active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Label::new(self.page.to_string())
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        let base = s
                            .w_9()
                            .h_9()
                            .text_sm()
                            .flex()
                            .items_center()
                            .justify_center()
                            .border_1()
                            .border_color(t.border)
                            .rounded_md()
                            .cursor(CursorStyle::Pointer);
                        if self.is_active {
                            base.background(t.primary).color(t.primary_foreground)
                        } else {
                            base.background(t.background)
                                .color(t.foreground)
                                .hover(|s| s.background(t.accent))
                        }
                    })
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    self.current_page.set(self.page);
                }),
        )
    }
}

// PaginationEllipsis
pub struct PaginationEllipsis;
impl PaginationEllipsis {
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Label::new("...").style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.w_9()
                    .h_9()
                    .text_sm()
                    .color(t.muted_foreground)
                    .flex()
                    .items_center()
                    .justify_center()
            })
        }))
    }
}

// PaginationPrevious
pub struct PaginationPrevious {
    id: ViewId,
    current_page: RwSignal<usize>,
    label: String,
}
impl PaginationPrevious {
    pub fn new(current_page: RwSignal<usize>) -> Self {
        Self {
            id: ViewId::new(),
            current_page,
            label: "Previous".to_string(),
        }
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Label::new(self.label)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.px_3()
                            .h_9()
                            .text_sm()
                            .flex()
                            .items_center()
                            .border_1()
                            .border_color(t.border)
                            .rounded_md()
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.background(t.accent))
                    })
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    let page = self.current_page.get();
                    if page > 1 {
                        self.current_page.set(page - 1);
                    }
                }),
        )
    }
}

// PaginationNext
pub struct PaginationNext {
    id: ViewId,
    current_page: RwSignal<usize>,
    total_pages: usize,
    label: String,
}
impl PaginationNext {
    pub fn new(current_page: RwSignal<usize>, total_pages: usize) -> Self {
        Self {
            id: ViewId::new(),
            current_page,
            total_pages,
            label: "Next".to_string(),
        }
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Label::new(self.label)
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.px_3()
                            .h_9()
                            .text_sm()
                            .flex()
                            .items_center()
                            .border_1()
                            .border_color(t.border)
                            .rounded_md()
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.background(t.accent))
                    })
                })
                .on_event_stop(floem::event::listener::Click, move |_, _| {
                    let page = self.current_page.get();
                    if page < self.total_pages {
                        self.current_page.set(page + 1);
                    }
                }),
        )
    }
}
