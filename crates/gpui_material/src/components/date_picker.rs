//! Modal date picker (calendar month). Specs: https://m3.material.io/components/date-pickers/specs

use crate::argb::Argb;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const DAY_DP: f32 = 40.0;
pub const CORNER_DP: f32 = 28.0;
pub const HEADER_PAD_DP: f32 = 24.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CivilDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DayKind {
    Selected,
    Today,
    InMonth,
    OutOfMonth,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DatePickerAppearance {
    pub corners: Corners,
    pub container: Argb,
    pub header_year: Argb,
    pub header_date: Argb,
    pub weekday: Argb,
    pub day: Argb,
    pub day_selected_container: Argb,
    pub day_selected: Argb,
    pub day_today_outline: Argb,
    pub day_out: Argb,
    pub elevation_dp: f32,
    pub day_dp: f32,
    pub year_style: TypeStyle,
    pub date_style: TypeStyle,
    pub weekday_style: TypeStyle,
    pub day_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> DatePickerAppearance {
    let c = theme.color;
    DatePickerAppearance {
        corners: Corners::all(CORNER_DP),
        container: c.surface_container_high,
        header_year: c.on_surface_variant,
        header_date: c.on_surface,
        weekday: c.on_surface_variant,
        day: c.on_surface,
        day_selected_container: c.primary,
        day_selected: c.on_primary,
        day_today_outline: c.primary,
        day_out: c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface_container_high),
        elevation_dp: theme.elevation.level3,
        day_dp: DAY_DP,
        year_style: theme.typography.label_large,
        date_style: theme.typography.headline_large,
        weekday_style: theme.typography.body_small,
        day_style: theme.typography.body_large,
    }
}

pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(year) => 29,
        2 => 28,
        _ => 0,
    }
}

pub fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Monday = 0 … Sunday = 6 (ISO), for the 1st of the month.
pub fn weekday_monday0(year: i32, month: u32, day: u32) -> u32 {
    // Sakamoto
    const T: [u32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 {
        y -= 1;
    }
    let w = (y + y / 4 - y / 100 + y / 400 + T[month as usize - 1] as i32 + day as i32) % 7;
    // Sakamoto: 0 = Sunday. Convert to Monday = 0.
    ((w + 6) % 7) as u32
}

/// 6×7 cells covering the month; `OutOfMonth` pads prev/next.
pub fn month_grid(year: i32, month: u32) -> [(u32, DayKind); 42] {
    let first = weekday_monday0(year, month, 1);
    let dim = days_in_month(year, month);
    let prev_month = if month == 1 { 12 } else { month - 1 };
    let prev_year = if month == 1 { year - 1 } else { year };
    let prev_dim = days_in_month(prev_year, prev_month);
    let mut cells = [(0u32, DayKind::OutOfMonth); 42];
    for i in 0..first {
        cells[i as usize] = (prev_dim - first + 1 + i, DayKind::OutOfMonth);
    }
    for d in 1..=dim {
        cells[(first + d - 1) as usize] = (d, DayKind::InMonth);
    }
    let used = first + dim;
    for i in used..42 {
        cells[i as usize] = (i - used + 1, DayKind::OutOfMonth);
    }
    cells
}

pub fn classify_day(
    year: i32,
    month: u32,
    day: u32,
    selected: CivilDate,
    today: CivilDate,
) -> DayKind {
    let date = CivilDate { year, month, day };
    if date == selected {
        DayKind::Selected
    } else if date == today {
        DayKind::Today
    } else {
        DayKind::InMonth
    }
}

pub const WEEKDAYS: [&str; 7] = ["M", "T", "W", "T", "F", "S", "S"];
pub const WEEKDAYS_FULL: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
pub const MONTHS: [&str; 12] = [
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
pub const MONTHS_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub fn add_months(year: i32, month: u32, delta: i32) -> (i32, u32) {
    let idx = year * 12 + (month as i32 - 1) + delta;
    let y = idx.div_euclid(12);
    let m = (idx.rem_euclid(12) + 1) as u32;
    (y, m)
}

pub fn month_title(year: i32, month: u32) -> String {
    let name = MONTHS
        .get((month.saturating_sub(1)) as usize)
        .copied()
        .unwrap_or("?");
    format!("{name} {year}")
}

pub fn header_date_label(date: CivilDate) -> String {
    let wd = weekday_monday0(date.year, date.month, date.day) as usize;
    let mon = MONTHS_SHORT
        .get((date.month.saturating_sub(1)) as usize)
        .copied()
        .unwrap_or("?");
    format!("{}, {} {}", WEEKDAYS_FULL[wd.min(6)], mon, date.day)
}

pub fn month_grid_classified(
    year: i32,
    month: u32,
    selected: CivilDate,
    today: CivilDate,
) -> [(u32, DayKind); 42] {
    let raw = month_grid(year, month);
    let mut out = raw;
    for (i, (day, kind)) in raw.iter().enumerate() {
        if *kind == DayKind::InMonth {
            out[i] = (*day, classify_day(year, month, *day, selected, today));
        }
    }
    out
}
