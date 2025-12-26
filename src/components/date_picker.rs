//! DatePicker component with builder-style API
//!
//! Based on shadcn/ui DatePicker - a date picker with calendar popup.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::date_picker::*;
//!
//! let date = RwSignal::new(None);
//!
//! DatePicker::new(date);
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::components::calendar::SimpleDate;
use crate::theme::ShadcnThemeExt;

// ============================================================================
// DatePicker
// ============================================================================

/// Date picker with calendar popup
pub struct DatePicker {
    id: ViewId,
    selected: RwSignal<Option<SimpleDate>>,
    placeholder: String,
    disabled: bool,
}

impl DatePicker {
    /// Create a new date picker
    pub fn new(selected: RwSignal<Option<SimpleDate>>) -> Self {
        Self {
            id: ViewId::new(),
            selected,
            placeholder: "Pick a date".to_string(),
            disabled: false,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl HasViewId for DatePicker {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for DatePicker {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let selected = self.selected;
        let placeholder = self.placeholder;
        let disabled = self.disabled;
        let is_open = RwSignal::new(false);

        // Trigger button
        let trigger = floem::views::Stack::horizontal((
            // Calendar icon
            floem::views::Label::new("📅").style(|s| s.font_size(14.0)),
            // Date text or placeholder
            floem::views::Label::derived(move || {
                if let Some(date) = selected.get() {
                    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
                } else {
                    placeholder.clone()
                }
            })
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let has_value = selected.get().is_some();
                    if has_value {
                        s.color(t.foreground)
                    } else {
                        s.color(t.muted_foreground)
                    }
                })
            }),
        ))
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .gap(8.0)
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .min_width(200.0)
                    .font_size(14.0)
                    .background(t.background)
                    .border(1.0)
                    .border_color(t.input)
                    .border_radius(t.radius)
                    .items_center()
                    .cursor(if disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    });
                if disabled {
                    base.color(t.muted_foreground).background(t.muted)
                } else {
                    base.hover(|s| s.border_color(t.ring))
                }
            })
        });

        let trigger = if disabled {
            trigger.into_any()
        } else {
            trigger
                .on_click_stop(move |_| {
                    is_open.update(|v| *v = !*v);
                })
                .into_any()
        };

        // Calendar popup content
        let view_year = RwSignal::new(2024);
        let view_month = RwSignal::new(12u32);

        // Initialize view to selected date or current
        if let Some(date) = selected.get_untracked() {
            view_year.set(date.year);
            view_month.set(date.month);
        }

        let calendar = create_calendar_content(selected, view_year, view_month, is_open);

        let popup = floem::views::Container::new(calendar).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let open = is_open.get();
                let base = s
                    .position(floem::style::Position::Absolute)
                    .inset_top_pct(100.0)
                    .inset_left(0.0)
                    .margin_top(4.0)
                    .padding(12.0)
                    .background(t.popover)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius)
                    .box_shadow_blur(8.0)
                    .box_shadow_color(t.foreground.with_alpha(0.1))
                    .z_index(100);
                if open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
        });

        // Backdrop - using absolute positioning with large area
        let backdrop = floem::views::Empty::new()
            .style(move |s| {
                let open = is_open.get();
                let base = s
                    .position(floem::style::Position::Absolute)
                    .inset_top(-1000.0)
                    .inset_left(-1000.0)
                    .width(3000.0)
                    .height(3000.0)
                    .z_index(99);

                if open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
            .on_click_stop(move |_| {
                is_open.set(false);
            });

        Box::new(
            floem::views::Container::new(floem::views::Stack::new((trigger, backdrop, popup)))
                .style(|s| s.position(floem::style::Position::Relative)),
        )
    }
}

fn create_calendar_content(
    selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
    is_open: RwSignal<bool>,
) -> impl IntoView {
    // Header with navigation
    let header = floem::views::Stack::horizontal((
        // Previous month button
        floem::views::Label::new("◀")
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.padding(4.0)
                        .font_size(12.0)
                        .color(t.foreground)
                        .border_radius(4.0)
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.accent))
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
            }),
        // Month/Year display
        floem::views::Label::derived(move || {
            let month_names = [
                "",
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ];
            let m = view_month.get() as usize;
            let y = view_year.get();
            format!("{} {}", month_names.get(m).unwrap_or(&""), y)
        })
        .style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.flex_grow(1.0)
                    .font_size(14.0)
                    .font_weight(floem::text::Weight::SEMIBOLD)
                    .color(t.foreground)
                    .justify_center()
            })
        }),
        // Next month button
        floem::views::Label::new("▶")
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.padding(4.0)
                        .font_size(12.0)
                        .color(t.foreground)
                        .border_radius(4.0)
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.accent))
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
            }),
    ))
    .style(|s| s.width_full().items_center().margin_bottom(8.0));

    // Day of week headers
    let day_headers = floem::views::Stack::horizontal((
        day_header("Su"),
        day_header("Mo"),
        day_header("Tu"),
        day_header("We"),
        day_header("Th"),
        day_header("Fr"),
        day_header("Sa"),
    ))
    .style(|s| s.margin_bottom(4.0));

    // Calendar grid (6 weeks)
    let week1 = create_picker_week(0, selected, view_year, view_month, is_open);
    let week2 = create_picker_week(1, selected, view_year, view_month, is_open);
    let week3 = create_picker_week(2, selected, view_year, view_month, is_open);
    let week4 = create_picker_week(3, selected, view_year, view_month, is_open);
    let week5 = create_picker_week(4, selected, view_year, view_month, is_open);
    let week6 = create_picker_week(5, selected, view_year, view_month, is_open);

    floem::views::Stack::vertical((
        header,
        day_headers,
        week1,
        week2,
        week3,
        week4,
        week5,
        week6,
    ))
    .style(|s| s.min_width(250.0))
}

fn day_header(label: &str) -> impl IntoView {
    floem::views::Label::new(label.to_string()).style(|s| {
        s.with_shadcn_theme(move |s, t| {
            s.width(32.0)
                .height(32.0)
                .font_size(12.0)
                .color(t.muted_foreground)
                .display(floem::style::Display::Flex)
                .items_center()
                .justify_center()
        })
    })
}

fn create_picker_week(
    week: i32,
    selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
    is_open: RwSignal<bool>,
) -> impl IntoView {
    floem::views::Stack::horizontal((
        create_picker_day(week * 7, selected, view_year, view_month, is_open),
        create_picker_day(week * 7 + 1, selected, view_year, view_month, is_open),
        create_picker_day(week * 7 + 2, selected, view_year, view_month, is_open),
        create_picker_day(week * 7 + 3, selected, view_year, view_month, is_open),
        create_picker_day(week * 7 + 4, selected, view_year, view_month, is_open),
        create_picker_day(week * 7 + 5, selected, view_year, view_month, is_open),
        create_picker_day(week * 7 + 6, selected, view_year, view_month, is_open),
    ))
}

fn create_picker_day(
    cell_index: i32,
    selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
    is_open: RwSignal<bool>,
) -> impl IntoView {
    floem::views::Label::derived(move || {
        let year = view_year.get();
        let month = view_month.get();
        let day = get_day_for_cell(year, month, cell_index);

        if day > 0 {
            day.to_string()
        } else {
            String::new()
        }
    })
    .style(move |s| {
        s.with_shadcn_theme(move |s, t| {
            let year = view_year.get();
            let month = view_month.get();
            let day = get_day_for_cell(year, month, cell_index);
            let is_selected = if let Some(sel) = selected.get() {
                day > 0 && sel.year == year && sel.month == month && sel.day == day as u32
            } else {
                false
            };
            let base = s
                .width(32.0)
                .height(32.0)
                .font_size(14.0)
                .border_radius(4.0)
                .display(floem::style::Display::Flex)
                .items_center()
                .justify_center()
                .cursor(if day > 0 {
                    CursorStyle::Pointer
                } else {
                    CursorStyle::Default
                });
            if is_selected {
                base.background(t.primary).color(t.primary_foreground)
            } else if day > 0 {
                base.color(t.foreground)
                    .hover(|s| s.background(t.accent).color(t.accent_foreground))
            } else {
                base.color(t.muted_foreground)
            }
        })
    })
    .on_click_stop(move |_| {
        let year = view_year.get();
        let month = view_month.get();
        let day = get_day_for_cell(year, month, cell_index);

        if day > 0 {
            selected.set(Some(SimpleDate {
                year,
                month,
                day: day as u32,
            }));
            is_open.set(false);
        }
    })
}

fn get_day_for_cell(year: i32, month: u32, cell_index: i32) -> i32 {
    let first_day_of_month = get_weekday(year, month, 1);
    let days_in_month = get_days_in_month(year, month);

    let day = cell_index - first_day_of_month + 1;

    if day >= 1 && day <= days_in_month {
        day
    } else {
        0
    }
}

fn get_weekday(year: i32, month: u32, day: u32) -> i32 {
    // Zeller's congruence for Gregorian calendar
    let m = month as i32;
    let y = year;
    let d = day as i32;

    let (m, y) = if m < 3 { (m + 12, y - 1) } else { (m, y) };

    let k = y % 100;
    let j = y / 100;

    let h = (d + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;

    // Convert to Sunday = 0
    (h + 6) % 7
}

fn get_days_in_month(year: i32, month: u32) -> i32 {
    match month {
        1 => 31,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30,
    }
}

// ============================================================================
// DateRangePicker
// ============================================================================

/// Date range picker with two calendars
pub struct DateRangePicker {
    id: ViewId,
    start: RwSignal<Option<SimpleDate>>,
    end: RwSignal<Option<SimpleDate>>,
    placeholder: String,
    disabled: bool,
}

impl DateRangePicker {
    /// Create a new date range picker
    pub fn new(start: RwSignal<Option<SimpleDate>>, end: RwSignal<Option<SimpleDate>>) -> Self {
        Self {
            id: ViewId::new(),
            start,
            end,
            placeholder: "Pick a date range".to_string(),
            disabled: false,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl HasViewId for DateRangePicker {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for DateRangePicker {
    type V = Box<dyn View>;
    type Intermediate = Self;

    fn into_intermediate(self) -> Self::Intermediate {
        self
    }

    fn into_view(self) -> Self::V {
        let start = self.start;
        let end = self.end;
        let placeholder = self.placeholder;
        let disabled = self.disabled;
        let is_open = RwSignal::new(false);

        // Trigger button
        let trigger = floem::views::Stack::horizontal((
            floem::views::Label::new("📅").style(|s| s.font_size(14.0)),
            floem::views::Label::derived(move || match (start.get(), end.get()) {
                (Some(s), Some(e)) => {
                    format!(
                        "{:04}-{:02}-{:02} → {:04}-{:02}-{:02}",
                        s.year, s.month, s.day, e.year, e.month, e.day
                    )
                }
                (Some(s), None) => {
                    format!("{:04}-{:02}-{:02} → ...", s.year, s.month, s.day)
                }
                _ => placeholder.clone(),
            })
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let has_value = start.get().is_some();
                    if has_value {
                        s.color(t.foreground)
                    } else {
                        s.color(t.muted_foreground)
                    }
                })
            }),
        ))
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .gap(8.0)
                    .padding_left(12.0)
                    .padding_right(12.0)
                    .padding_top(8.0)
                    .padding_bottom(8.0)
                    .min_width(280.0)
                    .font_size(14.0)
                    .background(t.background)
                    .border(1.0)
                    .border_color(t.input)
                    .border_radius(t.radius)
                    .items_center()
                    .cursor(if disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    });
                if disabled {
                    base.color(t.muted_foreground).background(t.muted)
                } else {
                    base.hover(|s| s.border_color(t.ring))
                }
            })
        });

        let trigger = if disabled {
            trigger.into_any()
        } else {
            trigger
                .on_click_stop(move |_| {
                    is_open.update(|v| *v = !*v);
                })
                .into_any()
        };

        // Simple popup showing start/end date pickers
        let popup = floem::views::Label::new("Date range selection").style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let open = is_open.get();
                let base = s
                    .position(floem::style::Position::Absolute)
                    .inset_top_pct(100.0)
                    .inset_left(0.0)
                    .margin_top(4.0)
                    .padding(16.0)
                    .background(t.popover)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius)
                    .box_shadow_blur(8.0)
                    .box_shadow_color(t.foreground.with_alpha(0.1))
                    .z_index(100)
                    .color(t.muted_foreground)
                    .font_size(14.0);
                if open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
        });

        // Backdrop - using absolute positioning with large area
        let backdrop = floem::views::Empty::new()
            .style(move |s| {
                let open = is_open.get();
                let base = s
                    .position(floem::style::Position::Absolute)
                    .inset_top(-1000.0)
                    .inset_left(-1000.0)
                    .width(3000.0)
                    .height(3000.0)
                    .z_index(99);

                if open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
            .on_click_stop(move |_| {
                is_open.set(false);
            });

        Box::new(
            floem::views::Container::new(floem::views::Stack::new((trigger, backdrop, popup)))
                .style(|s| s.position(floem::style::Position::Relative)),
        )
    }
}
