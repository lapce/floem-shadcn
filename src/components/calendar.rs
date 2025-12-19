//! Calendar component with builder-style API
//!
//! Based on shadcn/ui Calendar - a date picker component.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::calendar::*;
//!
//! let selected_date = RwSignal::new(None);
//!
//! Calendar::new(selected_date);
//! ```

use floem::prelude::*;
use floem::{HasViewId, ViewId};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;

use crate::theme::ShadcnThemeExt;

/// Simple date representation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u32, // 1-12
    pub day: u32,   // 1-31
}

impl SimpleDate {
    /// Create a new date
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Get current date (approximate - uses a simple algorithm)
    pub fn today() -> Self {
        // Simple approximation - in a real app you'd use chrono or time crate
        Self::new(2025, 1, 1)
    }

    /// Days in a given month
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    /// Day of week for the first of the month (0 = Sunday, 6 = Saturday)
    /// Uses Zeller's formula (simplified)
    pub fn first_day_of_week(year: i32, month: u32) -> u32 {
        let mut y = year;
        let mut m = month as i32;

        if m < 3 {
            m += 12;
            y -= 1;
        }

        let q = 1; // first day
        let k = y % 100;
        let j = y / 100;

        let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;

        // Convert from Zeller (0=Sat) to standard (0=Sun)
        ((h + 6) % 7) as u32
    }

    /// Get month name
    pub fn month_name(month: u32) -> &'static str {
        match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }
}

// ============================================================================
// Calendar
// ============================================================================

/// A calendar date picker
pub struct Calendar {
    id: ViewId,
    selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
}

impl Calendar {
    /// Create a new calendar
    pub fn new(selected: RwSignal<Option<SimpleDate>>) -> Self {
        let today = SimpleDate::today();
        Self {
            id: ViewId::new(),
            selected,
            view_year: RwSignal::new(today.year),
            view_month: RwSignal::new(today.month),
        }
    }

    /// Set initial view year
    pub fn year(self, year: i32) -> Self {
        self.view_year.set(year);
        self
    }

    /// Set initial view month
    pub fn month(self, month: u32) -> Self {
        self.view_month.set(month);
        self
    }
}

impl HasViewId for Calendar {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Calendar {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let selected = self.selected;
        let view_year = self.view_year;
        let view_month = self.view_month;

        // Navigation header
        let prev_button = floem::views::Label::new("<")
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.padding(8.0)
                        .font_size(14.0)
                        .color(t.foreground)
                        .cursor(CursorStyle::Pointer)
                        .border_radius(t.radius)
                        .hover(|s| s.background(t.muted))
                })
            })
            .on_click_stop(move |_| {
                let m = view_month.get();
                if m == 1 {
                    view_month.set(12);
                    view_year.update(|y| *y -= 1);
                } else {
                    view_month.set(m - 1);
                }
            });

        let next_button = floem::views::Label::new(">")
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.padding(8.0)
                        .font_size(14.0)
                        .color(t.foreground)
                        .cursor(CursorStyle::Pointer)
                        .border_radius(t.radius)
                        .hover(|s| s.background(t.muted))
                })
            })
            .on_click_stop(move |_| {
                let m = view_month.get();
                if m == 12 {
                    view_month.set(1);
                    view_year.update(|y| *y += 1);
                } else {
                    view_month.set(m + 1);
                }
            });

        let month_label = floem::views::Label::derived(move || {
            let m = view_month.get();
            let y = view_year.get();
            format!("{} {}", SimpleDate::month_name(m), y)
        })
        .style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.flex_grow(1.0)
                    .font_size(14.0)
                    .font_weight(floem::text::Weight::SEMIBOLD)
                    .color(t.foreground)
                    .justify_center()
            })
        });

        let header = floem::views::h_stack((prev_button, month_label, next_button))
            .style(|s| s.width_full().padding_bottom(8.0).items_center());

        // Day names row
        let day_names = floem::views::h_stack((
            day_header("Su"),
            day_header("Mo"),
            day_header("Tu"),
            day_header("We"),
            day_header("Th"),
            day_header("Fr"),
            day_header("Sa"),
        ))
        .style(|s| s.width_full().margin_bottom(4.0));

        // Calendar grid - create cells for all 42 positions (6 weeks x 7 days)
        // We'll use reactive labels that update based on the current month
        let grid = floem::views::v_stack((
            create_week_row(0, selected, view_year, view_month),
            create_week_row(1, selected, view_year, view_month),
            create_week_row(2, selected, view_year, view_month),
            create_week_row(3, selected, view_year, view_month),
            create_week_row(4, selected, view_year, view_month),
            create_week_row(5, selected, view_year, view_month),
        ))
        .style(|s| s.gap(2.0));

        Box::new(floem::views::v_stack((header, day_names, grid)).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.padding(12.0)
                    .background(t.card)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius)
            })
        }))
    }
}

fn create_week_row(
    week: i32,
    selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
) -> impl IntoView {
    floem::views::h_stack((
        create_day_cell(week * 7 + 0, selected, view_year, view_month),
        create_day_cell(week * 7 + 1, selected, view_year, view_month),
        create_day_cell(week * 7 + 2, selected, view_year, view_month),
        create_day_cell(week * 7 + 3, selected, view_year, view_month),
        create_day_cell(week * 7 + 4, selected, view_year, view_month),
        create_day_cell(week * 7 + 5, selected, view_year, view_month),
        create_day_cell(week * 7 + 6, selected, view_year, view_month),
    ))
    .style(|s| s.width_full())
}

fn create_day_cell(
    cell_index: i32,
    selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
) -> impl IntoView {
    floem::views::Label::derived(move || {
        let year = view_year.get();
        let month = view_month.get();
        let first_day = SimpleDate::first_day_of_week(year, month) as i32;
        let days_in_month = SimpleDate::days_in_month(year, month) as i32;

        let day_num = cell_index - first_day + 1;

        if day_num < 1 || day_num > days_in_month {
            String::new()
        } else {
            day_num.to_string()
        }
    })
    .style(move |s| {
        s.with_shadcn_theme(move |s, t| {
            let year = view_year.get();
            let month = view_month.get();
            let sel = selected.get();
            let first_day = SimpleDate::first_day_of_week(year, month) as i32;
            let days_in_month = SimpleDate::days_in_month(year, month) as i32;
            let day_num = cell_index - first_day + 1;
            let is_valid = day_num >= 1 && day_num <= days_in_month;
            let is_selected = is_valid
                && sel
                    .map(|d| d.year == year && d.month == month && d.day == day_num as u32)
                    .unwrap_or(false);
            let base = s
                .width(32.0)
                .height(32.0)
                .font_size(14.0)
                .border_radius(t.radius)
                .display(floem::style::Display::Flex)
                .items_center()
                .justify_center();
            if !is_valid {
                base
            } else if is_selected {
                base.background(t.primary)
                    .color(t.primary_foreground)
                    .cursor(CursorStyle::Pointer)
            } else {
                base.color(t.foreground)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.background(t.accent).color(t.accent_foreground))
            }
        })
    })
    .on_click_stop(move |_| {
        let year = view_year.get();
        let month = view_month.get();
        let first_day = SimpleDate::first_day_of_week(year, month) as i32;
        let days_in_month = SimpleDate::days_in_month(year, month) as i32;

        let day_num = cell_index - first_day + 1;
        if day_num >= 1 && day_num <= days_in_month {
            selected.set(Some(SimpleDate::new(year, month, day_num as u32)));
        }
    })
}

fn day_header(text: &'static str) -> impl IntoView {
    floem::views::Label::new(text).style(|s| {
        s.with_shadcn_theme(move |s, t| {
            s.width(32.0)
                .font_size(12.0)
                .font_weight(floem::text::Weight::MEDIUM)
                .color(t.muted_foreground)
                .justify_center()
        })
    })
}

// ============================================================================
// CalendarSimple - Simplified calendar without state management
// ============================================================================

/// A simpler calendar that just displays a month (static)
pub struct CalendarSimple {
    id: ViewId,
    year: i32,
    month: u32,
}

impl CalendarSimple {
    /// Create a simple calendar for a specific month
    pub fn new(year: i32, month: u32) -> Self {
        Self { id: ViewId::new(), year, month }
    }
}

impl HasViewId for CalendarSimple {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CalendarSimple {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let year = self.year;
        let month = self.month;

        // Header
        let title = floem::views::Label::new(format!("{} {}", SimpleDate::month_name(month), year))
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width_full()
                        .padding_bottom(8.0)
                        .font_size(14.0)
                        .font_weight(floem::text::Weight::SEMIBOLD)
                        .color(t.foreground)
                        .justify_center()
                })
            });

        // Day names
        let day_names = floem::views::h_stack((
            day_header("Su"),
            day_header("Mo"),
            day_header("Tu"),
            day_header("We"),
            day_header("Th"),
            day_header("Fr"),
            day_header("Sa"),
        ))
        .style(|s| s.width_full().margin_bottom(4.0));

        // Build grid - static version
        let first_day = SimpleDate::first_day_of_week(year, month) as i32;
        let days_in_month = SimpleDate::days_in_month(year, month) as i32;

        let grid = floem::views::v_stack((
            create_static_week(0, first_day, days_in_month),
            create_static_week(1, first_day, days_in_month),
            create_static_week(2, first_day, days_in_month),
            create_static_week(3, first_day, days_in_month),
            create_static_week(4, first_day, days_in_month),
            create_static_week(5, first_day, days_in_month),
        ))
        .style(|s| s.gap(2.0));

        Box::new(floem::views::v_stack((title, day_names, grid)).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.padding(12.0)
                    .background(t.card)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius)
            })
        }))
    }
}

fn create_static_week(week: i32, first_day: i32, days_in_month: i32) -> impl IntoView {
    floem::views::h_stack((
        static_day_cell(week * 7 + 0, first_day, days_in_month),
        static_day_cell(week * 7 + 1, first_day, days_in_month),
        static_day_cell(week * 7 + 2, first_day, days_in_month),
        static_day_cell(week * 7 + 3, first_day, days_in_month),
        static_day_cell(week * 7 + 4, first_day, days_in_month),
        static_day_cell(week * 7 + 5, first_day, days_in_month),
        static_day_cell(week * 7 + 6, first_day, days_in_month),
    ))
    .style(|s| s.width_full())
}

fn static_day_cell(cell_index: i32, first_day: i32, days_in_month: i32) -> impl IntoView {
    let day_num = cell_index - first_day + 1;
    let text = if day_num >= 1 && day_num <= days_in_month {
        day_num.to_string()
    } else {
        String::new()
    };

    floem::views::Label::new(text).style(move |s| {
        s.with_shadcn_theme(move |s, t| {
            s.width(32.0)
                .height(32.0)
                .font_size(14.0)
                .border_radius(t.radius)
                .color(t.foreground)
                .display(floem::style::Display::Flex)
                .items_center()
                .justify_center()
                .hover(|s| {
                    if day_num >= 1 && day_num <= days_in_month {
                        s.background(t.accent).color(t.accent_foreground)
                    } else {
                        s
                    }
                })
        })
    })
}
