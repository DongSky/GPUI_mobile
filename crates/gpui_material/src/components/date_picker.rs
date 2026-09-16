//! Modal date picker (calendar month). Specs: https://m3.material.io/components/date-pickers/specs

use crate::argb::Argb;
use crate::shape::Corners;
use crate::theme::Theme;
use crate::typography::TypeStyle;

pub const DAY_DP: f32 = 40.0;
pub const CORNER_DP: f32 = 28.0;
/// `DatePickerModalTokens.ContainerWidth` / DatePickerDialog `requiredWidth`.
pub const CONTAINER_W_DP: f32 = 360.0;
/// `DatePickerModalTokens.ContainerHeight` / DatePickerDialog `heightIn(max)`.
pub const CONTAINER_H_DP: f32 = 568.0;
/// Catalog / hosts apply official DatePickerDialog container size.
pub const CONTAINER_SIZE: bool = true;
/// Shared header start inset (Compose `DatePickerTitlePadding` start).
pub const HEADER_PAD_DP: f32 = 24.0;
/// Compose `DatePickerTitlePadding` = PaddingValues(start 24, end 12, top 16).
pub const TITLE_PAD_START_DP: f32 = 24.0;
pub const TITLE_PAD_END_DP: f32 = 12.0;
pub const TITLE_PAD_TOP_DP: f32 = 16.0;
/// Compose `DatePickerHeadlinePadding` = PaddingValues(start 24, end 12, bottom 12).
pub const HEADLINE_PAD_START_DP: f32 = 24.0;
pub const HEADLINE_PAD_END_DP: f32 = 12.0;
pub const HEADLINE_PAD_BOTTOM_DP: f32 = 12.0;
/// Catalog / hosts apply official title + headline paddings.
pub const HEADER_PADDINGS: bool = true;
/// `DatePickerModalTokens.HeaderContainerHeight` (title + headline + toggle).
pub const HEADER_CONTAINER_H_DP: f32 = 120.0;
/// `DatePickerModalTokens.RangeSelectionHeaderContainerHeight`.
pub const RANGE_HEADER_CONTAINER_H_DP: f32 = 128.0;
/// Compose `HeaderHeightOffset` — Save/X toolbar is not in the default layout.
pub const RANGE_HEADER_HEIGHT_OFFSET_DP: f32 = 60.0;
/// Compose range `headerMinHeight` = token 128 − offset 60.
pub const RANGE_HEADER_MIN_H_DP: f32 = RANGE_HEADER_CONTAINER_H_DP - RANGE_HEADER_HEIGHT_OFFSET_DP;
/// Catalog / hosts apply official header container min-heights.
pub const HEADER_CONTAINER_HEIGHTS: bool = true;
/// Compose `DateEntryContainer` header `HorizontalDivider` under title / headline / toggle.
/// Color: `DatePickerColors.dividerColor` = `DividerTokens.Color` (outline-variant).
/// Thickness: `DividerDefaults.Thickness`. Docked has no header chrome, so no divider.
pub const DATE_ENTRY_DIVIDER: bool = true;
pub const DATE_ENTRY_DIVIDER_H_DP: f32 = 1.0;

/// CSS `height` for the DateEntryContainer header divider.
pub fn date_entry_divider_height_css() -> String {
    format!("{:.0}px", DATE_ENTRY_DIVIDER_H_DP)
}

/// Compose shows the header divider iff title, headline, or mode toggle is present.
pub fn date_entry_divider_visible(
    has_title: bool,
    has_headline: bool,
    has_mode_toggle: bool,
) -> bool {
    DATE_ENTRY_DIVIDER && (has_title || has_headline || has_mode_toggle)
}

/// CSS `min-height` for `HeaderContainerHeight`.
pub fn header_min_height_css() -> String {
    format!("{:.0}px", HEADER_CONTAINER_H_DP)
}

/// CSS `min-height` for Compose range header (`128 − 60`).
pub fn range_header_min_height_css() -> String {
    format!("{:.0}px", RANGE_HEADER_MIN_H_DP)
}

/// CSS `padding` for `DatePickerTitlePadding` (top / end / bottom / start).
pub fn title_padding_css() -> String {
    format!(
        "{:.0}px {:.0}px 0 {:.0}px",
        TITLE_PAD_TOP_DP, TITLE_PAD_END_DP, TITLE_PAD_START_DP
    )
}

/// CSS `padding` for `DatePickerHeadlinePadding` (top / end / bottom / start).
pub fn headline_padding_css() -> String {
    format!(
        "0 {:.0}px {:.0}px {:.0}px",
        HEADLINE_PAD_END_DP, HEADLINE_PAD_BOTTOM_DP, HEADLINE_PAD_START_DP
    )
}

/// Compose `DateRangePickerTitlePadding` = PaddingValues(start 64, end 12).
/// Official range title has no top inset (unlike `DatePickerTitlePadding` top 16).
/// The 64dp start is the official close/back slot (8 + 48 + 8).
pub const RANGE_TITLE_PAD_START_DP: f32 = 64.0;
pub const RANGE_TITLE_PAD_END_DP: f32 = 12.0;
pub const RANGE_TITLE_PAD_TOP_DP: f32 = 0.0;
/// Compose `DateRangePickerHeadlinePadding` = PaddingValues(start 64, end 12, bottom 12).
pub const RANGE_HEADLINE_PAD_START_DP: f32 = 64.0;
pub const RANGE_HEADLINE_PAD_END_DP: f32 = 12.0;
pub const RANGE_HEADLINE_PAD_BOTTOM_DP: f32 = 12.0;
/// Catalog / hosts apply official range title + headline paddings.
pub const RANGE_HEADER_PADDINGS: bool = true;
/// Official range-header chrome: leading close/back in the 64dp start column.
pub const RANGE_HEADER_CHROME: bool = true;
pub const RANGE_HEADER_CLOSE: bool = true;
/// `Icons.Filled.Close` stand-in (same glyph as fullscreen dialog / snackbar).
pub const RANGE_HEADER_CLOSE_GLYPH: &str = "✕";
pub const RANGE_HEADER_CLOSE_LABEL: &str = "Close";
/// 48dp icon-button target; 8dp start inset + 48 + 8 = 64dp title start.
pub const RANGE_HEADER_CLOSE_TARGET_DP: f32 = 48.0;
pub const RANGE_HEADER_CLOSE_INSET_DP: f32 = 8.0;

/// CSS `padding` for `DateRangePickerTitlePadding` (top / end / bottom / start).
pub fn range_title_padding_css() -> String {
    format!(
        "{:.0}px {:.0}px 0 {:.0}px",
        RANGE_TITLE_PAD_TOP_DP, RANGE_TITLE_PAD_END_DP, RANGE_TITLE_PAD_START_DP
    )
}

/// CSS `padding` for `DateRangePickerHeadlinePadding` (top / end / bottom / start).
pub fn range_headline_padding_css() -> String {
    format!(
        "0 {:.0}px {:.0}px {:.0}px",
        RANGE_HEADLINE_PAD_END_DP, RANGE_HEADLINE_PAD_BOTTOM_DP, RANGE_HEADLINE_PAD_START_DP
    )
}

/// Header close matches Cancel: restore the last committed range.
pub fn apply_range_header_close(committed: DateRangeSelection) -> DateRangeSelection {
    apply_range_dismiss(committed)
}

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
    /// Interior of a selected date range (not the start/end endpoints).
    InRange,
    /// Compose `SelectableDates` — in-month weekend (demo Sat/Sun).
    Disabled,
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
    pub day_range_container: Argb,
    pub day_range: Argb,
    /// Compose `DatePickerDefaults.subheadContentColor` (month subhead).
    pub month_subhead: Argb,
    pub elevation_dp: f32,
    pub day_dp: f32,
    pub year_style: TypeStyle,
    pub date_style: TypeStyle,
    pub weekday_style: TypeStyle,
    pub day_style: TypeStyle,
    /// `DatePickerModalTokens.RangeSelectionMonthSubheadFont` → titleSmall.
    pub month_subhead_style: TypeStyle,
}

pub fn resolve(theme: &Theme) -> DatePickerAppearance {
    let c = theme.color;
    DatePickerAppearance {
        corners: Corners::all(CORNER_DP),
        container: c.surface_container_high,
        header_year: c.on_surface_variant,
        header_date: c.on_surface,
        weekday: c.on_surface,
        day: c.on_surface,
        day_selected_container: c.primary,
        day_selected: c.on_primary,
        day_today_outline: c.primary,
        day_out: c
            .on_surface
            .with_alpha(crate::state::DISABLED_CONTENT_OPACITY)
            .composite_over(c.surface_container_high),
        day_range_container: c.secondary_container,
        day_range: c.on_secondary_container,
        month_subhead: c.on_surface_variant,
        elevation_dp: theme.elevation.level3,
        day_dp: DAY_DP,
        year_style: theme.typography.label_large,
        date_style: theme.typography.headline_large.emphasized(),
        weekday_style: theme.typography.body_large,
        day_style: theme.typography.body_large,
        month_subhead_style: theme.typography.title_small,
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

/// Sakamoto weekday: 0 = Sunday … 6 = Saturday.
fn weekday_sakamoto(year: i32, month: u32, day: u32) -> u32 {
    const T: [u32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 {
        y -= 1;
    }
    let w = (y + y / 4 - y / 100 + y / 400 + T[month as usize - 1] as i32 + day as i32) % 7;
    w as u32
}

/// Official m3.material.io modal date picker is Sunday-first (US locale
/// default on the live overview / modal variant). Catalogs match that rather
/// than ISO Monday-first so Visual QA lines up with the site. `weekday_monday0`
/// remains for ISO callers.
pub const WEEK_STARTS_ON_SUNDAY: bool = true;

/// Sunday = 0 … Saturday = 6. Used by `month_grid` / `WEEKDAYS`.
pub fn weekday_sunday0(year: i32, month: u32, day: u32) -> u32 {
    weekday_sakamoto(year, month, day)
}

/// Monday = 0 … Sunday = 6 (ISO).
pub fn weekday_monday0(year: i32, month: u32, day: u32) -> u32 {
    (weekday_sakamoto(year, month, day) + 6) % 7
}

/// 6×7 cells covering the month; `OutOfMonth` pads prev/next.
/// Column 0 is Sunday, matching the official modal.
pub fn month_grid(year: i32, month: u32) -> [(u32, DayKind); 42] {
    let first = weekday_sunday0(year, month, 1);
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
    } else if !is_selectable_date(date) {
        DayKind::Disabled
    } else if date == today {
        DayKind::Today
    } else {
        DayKind::InMonth
    }
}

/// In-month days that Compose `SelectableDates` still allows to be tapped.
pub fn day_accepts_tap(kind: DayKind) -> bool {
    !matches!(kind, DayKind::OutOfMonth | DayKind::Disabled)
}

/// Sunday-first, matching official modal date picker columns.
pub const WEEKDAYS: [&str; 7] = ["S", "M", "T", "W", "T", "F", "S"];
pub const WEEKDAYS_FULL: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
/// Catalog / hosts apply official `WeekDays` color / type / row min-height.
pub const WEEKDAYS_LABEL: bool = true;
/// Compose `WeekDays` `defaultMinSize(minHeight = RecommendedSizeForAccessibility)`.
pub const WEEKDAY_ROW_MIN_H_DP: f32 = 48.0;

/// CSS `min-height` for the official weekday letter row.
pub fn weekday_row_min_height_css() -> String {
    format!("{:.0}px", WEEKDAY_ROW_MIN_H_DP)
}
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

pub fn header_year_label(year: i32) -> String {
    format!("{year} ▾")
}

pub fn header_date_label(date: CivilDate) -> String {
    let wd = weekday_sunday0(date.year, date.month, date.day) as usize;
    let mon = MONTHS_SHORT
        .get((date.month.saturating_sub(1)) as usize)
        .copied()
        .unwrap_or("?");
    format!("{}, {} {}", WEEKDAYS_FULL[wd.min(6)], mon, date.day)
}

pub fn header_date_short(date: CivilDate) -> String {
    let mon = MONTHS_SHORT
        .get((date.month.saturating_sub(1)) as usize)
        .copied()
        .unwrap_or("?");
    format!("{} {}", mon, date.day)
}

/// Official overview range-hero sample. Live taps follow Compose `DateRangePicker`.
pub const RANGE_HERO_TITLE: &str = "Depart – Return dates";
/// Compose `m3c_date_range_picker_title` (default Picker title, not the travel hero).
pub const RANGE_PICKER_TITLE: &str = "Select dates";
/// Catalog / host sibling for DateRangePicker Picker empty.
pub const RANGE_PICKER_EMPTY: bool = true;
/// Catalog / host range hero starts with a complete depart–return pair.
pub const RANGE_LIVE: bool = true;
/// Compose DateRangePicker month pager (prev / next) on the range hero.
pub const RANGE_MONTH_NAV: bool = true;
pub const RANGE_PREV_MONTH: &str = "Previous month";
pub const RANGE_NEXT_MONTH: &str = "Next month";
/// Range-hero month ▾ opens Compose `YearPicker` (independent of the single-date modal).
pub const RANGE_YEAR_PANE: bool = true;
/// Compose `DateRangePickerDefaults.showModeToggle` on the range hero (not the single-date modal).
pub const RANGE_SHOW_MODE_TOGGLE: bool = true;
/// Compose modal `DateRangePicker` header divider + Confirm/Cancel (draft until OK).
pub const RANGE_ACTIONS: bool = true;
pub const RANGE_DIVIDER_H_DP: f32 = 1.0;
/// Compose `drawRangeBackground` half-cell start/end connectors
/// (`DateRangePicker.kt` firstIsSelectionStart / lastIsSelectionEnd).
pub const RANGE_CONNECTOR: bool = true;
/// Compose `VerticalMonthsList`: stacked months with a month/year subhead.
pub const RANGE_VERTICAL_MONTHS: bool = true;
/// Catalog / host window (Compose LazyColumn; two months stay on-screen).
pub const RANGE_VISIBLE_MONTHS: usize = 2;
/// `DateRangePicker.kt` `CalendarMonthSubheadPadding`.
pub const MONTH_SUBHEAD_PAD_START_DP: f32 = 24.0;
pub const MONTH_SUBHEAD_PAD_TOP_DP: f32 = 20.0;
pub const MONTH_SUBHEAD_PAD_BOTTOM_DP: f32 = 8.0;
/// Compose modal `DatePicker` header divider + Confirm/Cancel (draft until OK).
pub const DATE_ACTIONS: bool = true;
pub const DATE_DIVIDER_H_DP: f32 = 1.0;
/// Compose `DatePickerDialog` `DialogButtonsPadding` = PaddingValues(bottom 8, end 6).
pub const DIALOG_BUTTONS_PAD_END_DP: f32 = 6.0;
pub const DIALOG_BUTTONS_PAD_BOTTOM_DP: f32 = 8.0;
/// Compose `DialogButtonsMainAxisSpacing`.
pub const DIALOG_BUTTONS_MAIN_GAP_DP: f32 = 8.0;
/// Compose `DialogButtonsCrossAxisSpacing` (androidx-main is 8).
pub const DIALOG_BUTTONS_CROSS_GAP_DP: f32 = 8.0;
/// Catalog / hosts apply official DatePickerDialog button paddings.
pub const DIALOG_BUTTONS_PADDINGS: bool = true;

/// CSS `padding` for `DialogButtonsPadding` (top / end / bottom / start).
pub fn dialog_buttons_padding_css() -> String {
    format!(
        "0 {:.0}px {:.0}px 0",
        DIALOG_BUTTONS_PAD_END_DP, DIALOG_BUTTONS_PAD_BOTTOM_DP
    )
}

/// CSS `row-gap` for `DialogButtonsCrossAxisSpacing`.
pub fn dialog_buttons_cross_gap_css() -> String {
    format!("{:.0}px", DIALOG_BUTTONS_CROSS_GAP_DP)
}
/// Compose `DatePicker` month pager (prev / next) on the single-date modal.
pub const DATE_MONTH_NAV: bool = true;
pub const DATE_PREV_MONTH: &str = "Previous month";
pub const DATE_NEXT_MONTH: &str = "Next month";
/// Compose `MonthYearHeight` for `MonthsNavigation`.
pub const MONTH_YEAR_H_DP: f32 = 56.0;
/// Compose `RecommendedSizeForAccessibility` / IconButton target on month arrows.
pub const MONTH_NAV_ICON_DP: f32 = 48.0;
/// Catalog / hosts apply official MonthsNavigation (label start, arrows end).
pub const MONTH_YEAR_NAV: bool = true;
pub const RANGE_DEMO_START: CivilDate = CivilDate {
    year: 2026,
    month: 9,
    day: 15,
};
pub const RANGE_DEMO_END: CivilDate = CivilDate {
    year: 2026,
    month: 9,
    day: 21,
};

/// Official overview range hero headline, e.g. "Aug 17 – Aug 23".
pub fn header_range_label(start: CivilDate, end: CivilDate) -> String {
    format!("{} – {}", header_date_short(start), header_date_short(end))
}

/// Compose `DateRangePicker` selection (start, then end ≥ start; a third tap restarts).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateRangeSelection {
    pub start: Option<CivilDate>,
    pub end: Option<CivilDate>,
}

impl DateRangeSelection {
    pub const fn empty() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub const fn demo() -> Self {
        Self {
            start: Some(RANGE_DEMO_START),
            end: Some(RANGE_DEMO_END),
        }
    }

    /// Compose `DateRangePickerHeadline` after the first tap (start set, end open).
    pub const fn start_only() -> Self {
        Self {
            start: Some(RANGE_DEMO_START),
            end: None,
        }
    }

    /// Compose `DateRangePickerHeadline` when only end is set (`Start date – %1$s`).
    pub const fn end_only() -> Self {
        Self {
            start: None,
            end: Some(RANGE_DEMO_END),
        }
    }
}

/// First tap sets start; second tap ≥ start sets end, else replaces start; both set restarts.
pub fn apply_range_tap(sel: DateRangeSelection, day: CivilDate) -> DateRangeSelection {
    if !is_selectable_date(day) {
        return sel;
    }
    match (sel.start, sel.end) {
        (None, _) => DateRangeSelection {
            start: Some(day),
            end: None,
        },
        (Some(start), None) => {
            if date_ord(day) >= date_ord(start) {
                DateRangeSelection {
                    start: Some(start),
                    end: Some(day),
                }
            } else {
                DateRangeSelection {
                    start: Some(day),
                    end: None,
                }
            }
        }
        (Some(_), Some(_)) => DateRangeSelection {
            start: Some(day),
            end: None,
        },
    }
}

/// Jump the range-hero calendar to a YearPicker year; keep the displayed month.
pub fn apply_range_year(_year: i32, month: u32, picked: i32) -> (i32, u32) {
    (clamp_year(picked), month)
}

/// Page the range-hero calendar; clamp to Compose `YearRange` 1900–2100.
pub fn apply_range_month(year: i32, month: u32, delta: i32) -> (i32, u32) {
    let (y, m) = add_months(year, month, delta);
    if y < YEAR_RANGE_START {
        (YEAR_RANGE_START, 1)
    } else if y > YEAR_RANGE_END {
        (YEAR_RANGE_END, 12)
    } else {
        (y, m)
    }
}

/// Compose `dateFormatter.formatMonthYear` — month subhead, no year ▾.
pub fn month_subhead_label(year: i32, month: u32) -> String {
    month_title(year, month)
}

/// Visible `VerticalMonthsList` window starting at the displayed month.
pub fn range_visible_months(year: i32, month: u32) -> [(i32, u32); RANGE_VISIBLE_MONTHS] {
    let mut out = [(year, month); RANGE_VISIBLE_MONTHS];
    for i in 1..RANGE_VISIBLE_MONTHS {
        out[i] = apply_range_month(out[i - 1].0, out[i - 1].1, 1);
    }
    out
}

/// Headline placeholders use Compose `Start date` / `End date` until both ends exist.
pub fn header_range_selection(sel: DateRangeSelection) -> String {
    match (sel.start, sel.end) {
        (Some(start), Some(end)) => header_range_label(start, end),
        (Some(start), None) => format!("{} – {}", header_date_short(start), RANGE_END_LABEL),
        (None, Some(end)) => format!("{} – {}", RANGE_START_LABEL, header_date_short(end)),
        (None, None) => format!("{} – {}", RANGE_START_LABEL, RANGE_END_LABEL),
    }
}

pub fn month_nav_label(year: i32, month: u32) -> String {
    format!("{} ▾", month_title(year, month))
}

/// Compose docked date picker: outlined field + calendar attached below
/// (https://developer.android.com/reference/kotlin/androidx/compose/material3/package-summary#DatePickerDocked).
pub const DOCKED_FIELD_LABEL: &str = "Date of birth";
/// Catalog starts with the popup open so Visual QA can see the anchored sheet.
pub const DOCKED_OPEN_BY_DEFAULT: bool = true;
/// Selecting a day writes the field and dismisses (desktop/Android popup).
pub const DOCKED_DISMISS_ON_SELECT: bool = true;
/// Clicking outside the field+popup dismisses (desktop/HTML/Android).
pub const DOCKED_DISMISS_ON_OUTSIDE: bool = true;
/// Docked popup month ▾ opens Compose `YearPicker` (independent of the modal / range hero).
pub const DOCKED_YEAR_PANE: bool = true;
/// Official `DatePickerDocked`: tap an in-month day writes the field (hosts + catalog).
pub const DOCKED_LIVE_SELECT: bool = true;

pub fn docked_field_value(date: CivilDate) -> String {
    format!("{}, {}", header_date_short(date), date.year)
}

/// Compose `DatePickerDisplayMode` (Picker calendar ↔ Input text).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatePickerDisplayMode {
    Picker,
    Input,
}

impl DatePickerDisplayMode {
    pub const ALL: [Self; 2] = [Self::Picker, Self::Input];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Picker => "picker",
            Self::Input => "input",
        }
    }

    pub const fn toggle(self) -> Self {
        match self {
            Self::Picker => Self::Input,
            Self::Input => Self::Picker,
        }
    }

    /// Icon for the *other* mode (edit when picking, calendar when typing).
    pub const fn toggle_icon(self) -> &'static str {
        match self {
            Self::Picker => INPUT_TOGGLE_EDIT,
            Self::Input => INPUT_TOGGLE_CALENDAR,
        }
    }

    pub const fn toggle_label(self) -> &'static str {
        match self {
            Self::Picker => "Switch to input mode",
            Self::Input => "Switch to calendar mode",
        }
    }
}

/// Catalog modal-input sibling starts on Input (Compose `DisplayMode.Input`).
pub const DEMO_DISPLAY_MODE: DatePickerDisplayMode = DatePickerDisplayMode::Input;
/// Live modal starts on Picker (Compose `initialDisplayMode` default).
pub const LIVE_DISPLAY_MODE: DatePickerDisplayMode = DatePickerDisplayMode::Picker;
/// Compose `DatePickerDefaults.showModeToggle`.
pub const SHOW_MODE_TOGGLE: bool = true;
/// Compose mode-toggle 48dp target.
pub const TOGGLE_SIZE_DP: f32 = 48.0;
/// Compose `DatePickerModeTogglePadding` = PaddingValues(end 12, bottom 12).
pub const TOGGLE_PAD_END_DP: f32 = 12.0;
pub const TOGGLE_PAD_BOTTOM_DP: f32 = 12.0;
/// Catalog / hosts apply official mode-toggle paddings.
pub const TOGGLE_PADDINGS: bool = true;

/// CSS `padding` for `DatePickerModeTogglePadding` (top / end / bottom / start).
pub fn toggle_padding_css() -> String {
    format!(
        "0 {:.0}px {:.0}px 0",
        TOGGLE_PAD_END_DP, TOGGLE_PAD_BOTTOM_DP
    )
}
pub const INPUT_HEADLINE: &str = "Select date";
/// Compose `m3c_date_input_headline` when Input has no selection.
pub const INPUT_EMPTY_HEADLINE: &str = "Entered date";
/// Compose `m3c_date_picker_headline` when Picker has no selection.
pub const PICKER_EMPTY_HEADLINE: &str = "Selected date";
/// Catalog / host sibling for DatePickerHeadline Input empty.
pub const INPUT_EMPTY: bool = true;
/// Catalog / host sibling for DatePickerHeadline Picker empty.
pub const PICKER_EMPTY: bool = true;
pub const INPUT_SUPPORTING: &str = "Enter date";
pub const INPUT_FIELD_LABEL: &str = "Date";
pub const INPUT_PLACEHOLDER: &str = "MM/DD/YYYY";
/// Compose `InputTextFieldPadding` (start 24 / end 24 / top 10).
pub const INPUT_FIELD_PAD_START_DP: f32 = 24.0;
pub const INPUT_FIELD_PAD_END_DP: f32 = 24.0;
pub const INPUT_FIELD_PAD_TOP_DP: f32 = 10.0;
/// Compose `InputTextNonErroneousBottomPadding`.
pub const INPUT_FIELD_PAD_BOTTOM_DP: f32 = 16.0;
/// Catalog / hosts apply official Date Input field paddings.
pub const INPUT_FIELD_PADDINGS: bool = true;

/// CSS `padding` for Date Input fields (top / end / bottom / start).
pub fn input_field_padding_css() -> String {
    format!(
        "{:.0}px {:.0}px {:.0}px {:.0}px",
        INPUT_FIELD_PAD_TOP_DP,
        INPUT_FIELD_PAD_END_DP,
        INPUT_FIELD_PAD_BOTTOM_DP,
        INPUT_FIELD_PAD_START_DP
    )
}
pub const INPUT_TOGGLE_EDIT: &str = "✎";
pub const INPUT_TOGGLE_CALENDAR: &str = "▦";
pub const INPUT_OK: &str = "OK";
pub const INPUT_CANCEL: &str = "Cancel";
/// Compose `Icons.Default.DateRange` trailing icon button (M3 docked spec item 4).
pub const DOCKED_TRAILING: bool = true;
/// Catalog/host stand-in for the DateRange calendar glyph.
pub const DOCKED_TRAILING_ICON: &str = INPUT_TOGGLE_CALENDAR;
pub const DOCKED_TRAILING_LABEL: &str = "Select date";
pub const DOCKED_TRAILING_DP: f32 = 24.0;
pub const DOCKED_TRAILING_TARGET_DP: f32 = 48.0;
/// Compose `DateRangeInputTitle`.
pub const RANGE_INPUT_HEADLINE: &str = "Enter dates";
/// Compose `DateRangePickerStartHeadline` / `DateRangePickerEndHeadline`.
pub const RANGE_START_LABEL: &str = "Start date";
pub const RANGE_END_LABEL: &str = "End date";
/// Compose `DateRangePickerHeadline` when both ends are unset.
pub const RANGE_EMPTY_HEADLINE: &str = "Start date – End date";
/// Catalog / host sibling for empty range Input.
pub const RANGE_EMPTY: bool = true;
/// Compose `DateRangePickerHeadline` when only start is set (`%1$s – End date`).
pub const RANGE_START_ONLY: bool = true;
/// Compose `mtrl_picker_range_header_only_end_selected` (`Start date – %1$s`).
pub const RANGE_END_ONLY: bool = true;
pub const RANGE_END_ONLY_HEADLINE: &str = "Start date – Sep 21";
pub const RANGE_INPUT_GAP_DP: f32 = 8.0;
/// Compose `DateInputValidator` supporting-text errors on range Input.
pub const RANGE_INPUT_ERRORS: bool = true;
/// Compose `DateInputValidator` supporting-text on single-date Input.
pub const DATE_INPUT_ERRORS: bool = true;
/// `m3c_date_input_invalid_for_pattern`.
pub const INPUT_ERROR_FORMAT: &str = "Date format not recognized";
/// `m3c_date_range_input_invalid_range_input`.
pub const RANGE_INPUT_ERROR_ORDER: &str = "End date can't be before start date";
/// Catalog format-error sample (invalid month/day).
pub const INPUT_ERROR_FORMAT_SAMPLE: &str = "13/40/2026";
/// `m3c_date_input_invalid_year_range` with Compose `YearRange` 1900–2100.
pub const INPUT_ERROR_YEAR_RANGE: &str = "Date out of expected year range 1900 - 2100";
/// Catalog year-range sample (parsed calendar day, outside YearRange).
pub const INPUT_ERROR_YEAR_SAMPLE: &str = "09/15/1890";
/// Compose `SelectableDates` (official DatePicker sample blocks Sat/Sun).
pub const SELECTABLE_DATES: bool = true;
/// Grey out + ignore taps on weekend cells in Picker grids.
pub const SELECTABLE_DATES_GRID: bool = true;
/// `m3c_date_input_invalid_not_allowed` with `header_date_label` for the sample.
pub const INPUT_ERROR_NOT_ALLOWED: &str = "Date not allowed: Sat, Sep 12";
/// Catalog not-allowed sample (Saturday 2026-09-12).
pub const INPUT_ERROR_NOT_ALLOWED_SAMPLE: &str = "09/12/2026";
pub const INPUT_ERROR_NOT_ALLOWED_DATE: CivilDate = CivilDate {
    year: 2026,
    month: 9,
    day: 12,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateInputError {
    None,
    Format,
    YearRange,
    NotAllowed,
    Order,
}

impl DateInputError {
    pub const fn label(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Format => Some(INPUT_ERROR_FORMAT),
            Self::YearRange => Some(INPUT_ERROR_YEAR_RANGE),
            Self::NotAllowed => Some(INPUT_ERROR_NOT_ALLOWED),
            Self::Order => Some(RANGE_INPUT_ERROR_ORDER),
        }
    }
}

pub fn apply_display_toggle(mode: DatePickerDisplayMode) -> DatePickerDisplayMode {
    mode.toggle()
}

/// Compose `DatePickerHeadline`: formatted date, or the mode's empty placeholder.
pub fn date_headline(mode: DatePickerDisplayMode, selected: Option<CivilDate>) -> String {
    match selected {
        Some(date) => header_date_label(date),
        None => empty_headline(mode).to_string(),
    }
}

pub fn empty_headline(mode: DatePickerDisplayMode) -> &'static str {
    match mode {
        DatePickerDisplayMode::Input => INPUT_EMPTY_HEADLINE,
        DatePickerDisplayMode::Picker => PICKER_EMPTY_HEADLINE,
    }
}

/// Overview travel title in Picker; Compose `DateRangeInputTitle` in Input.
pub fn range_title_for(mode: DatePickerDisplayMode) -> &'static str {
    match mode {
        DatePickerDisplayMode::Picker => RANGE_HERO_TITLE,
        DatePickerDisplayMode::Input => RANGE_INPUT_HEADLINE,
    }
}

pub fn range_field_value(sel: DateRangeSelection, end: bool) -> String {
    let date = if end { sel.end } else { sel.start };
    date.map(input_field_value).unwrap_or_default()
}

/// OK commits the draft range. Cancel restores the last committed pair.
pub fn apply_range_confirm(draft: DateRangeSelection) -> DateRangeSelection {
    draft
}

pub fn apply_range_dismiss(committed: DateRangeSelection) -> DateRangeSelection {
    committed
}

/// Month pager follows the draft start, else end, else the demo month.
pub fn range_month_of(sel: DateRangeSelection) -> (i32, u32) {
    match (sel.start, sel.end) {
        (Some(s), _) => (s.year, s.month),
        (None, Some(e)) => (e.year, e.month),
        (None, None) => (RANGE_DEMO_START.year, RANGE_DEMO_START.month),
    }
}

/// OK commits the draft date. Cancel restores the last committed day.
pub fn apply_date_confirm(draft: CivilDate) -> CivilDate {
    draft
}

pub fn apply_date_dismiss(committed: CivilDate) -> CivilDate {
    committed
}

/// Month pager follows the committed / draft civil date.
pub fn date_month_of(date: CivilDate) -> (i32, u32) {
    (date.year, date.month)
}

/// Page the single-date modal calendar; clamp to Compose `YearRange` 1900–2100.
pub fn apply_date_month(year: i32, month: u32, delta: i32) -> (i32, u32) {
    apply_range_month(year, month, delta)
}

/// Jump the docked calendar to a YearPicker year; keep the displayed month.
pub fn apply_docked_year(year: i32, month: u32, picked: i32) -> (i32, u32) {
    apply_range_year(year, month, picked)
}

/// Field or trailing DateRange icon toggles the docked popup.
pub fn apply_docked_toggle(open: bool) -> bool {
    !open
}

/// Tap an in-month docked day: commit the civil date and whether the popup dismisses.
pub fn apply_docked_tap(year: i32, month: u32, day: u32) -> (CivilDate, bool) {
    let date = CivilDate { year, month, day };
    if !is_selectable_date(date) {
        return (date, false);
    }
    (date, DOCKED_DISMISS_ON_SELECT)
}

pub fn supporting_for(mode: DatePickerDisplayMode) -> Option<&'static str> {
    match mode {
        DatePickerDisplayMode::Picker => None,
        DatePickerDisplayMode::Input => Some(INPUT_SUPPORTING),
    }
}

pub fn input_field_value(date: CivilDate) -> String {
    format!("{:02}/{:02}/{:04}", date.month, date.day, date.year)
}

pub fn parse_input_field(s: &str) -> Option<CivilDate> {
    let parts: Vec<&str> = s.trim().split('/').collect();
    if parts.len() != 3 {
        return None;
    }
    let month: u32 = parts[0].parse().ok()?;
    let day: u32 = parts[1].parse().ok()?;
    let year: i32 = parts[2].parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    let max = days_in_month(year, month);
    if day < 1 || day > max {
        return None;
    }
    Some(CivilDate { year, month, day })
}

pub fn is_input_valid(s: &str) -> bool {
    parse_input_field(s).is_some()
}

/// Compose `DatePickerDefaults.YearRange`.
pub const YEAR_RANGE_START: i32 = 1900;
pub const YEAR_RANGE_END: i32 = 2100;
/// Compose `YearsInRow`.
pub const YEARS_IN_ROW: usize = 3;
/// `DatePickerModalTokens.SelectionYearContainerWidth`.
pub const YEAR_CONTAINER_W_DP: f32 = 72.0;
/// `DatePickerModalTokens.SelectionYearContainerHeight`.
pub const YEAR_CONTAINER_H_DP: f32 = 36.0;
/// Compose `YearsVerticalPadding`.
pub const YEAR_GAP_DP: f32 = 16.0;
/// Catalog / host window (3 rows × 3 cols) around the displayed year.
pub const YEAR_WINDOW: usize = 9;
/// Compose `SwitchableDateEntryContent` `HorizontalDivider` under `YearPicker`.
pub const YEAR_PICKER_DIVIDER: bool = true;
/// `DividerDefaults.Thickness` on the YearPicker trailing rule.
pub const YEAR_PICKER_DIVIDER_H_DP: f32 = 1.0;

/// CSS `height` for the YearPicker trailing `HorizontalDivider`.
pub fn year_picker_divider_height_css() -> String {
    format!("{:.0}px", YEAR_PICKER_DIVIDER_H_DP)
}

/// Calendar month grid ↔ Compose `YearPicker`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatePickerPane {
    Calendar,
    Year,
}

impl DatePickerPane {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Calendar => "calendar",
            Self::Year => "year",
        }
    }

    pub const fn toggle(self) -> Self {
        match self {
            Self::Calendar => Self::Year,
            Self::Year => Self::Calendar,
        }
    }
}

/// Live modal starts on the calendar (`yearPickerVisible = false`).
pub const LIVE_PANE: DatePickerPane = DatePickerPane::Calendar;
/// Catalog year-picker sibling starts open.
pub const DEMO_PANE: DatePickerPane = DatePickerPane::Year;

/// Official YearPicker paints the trailing divider only while the year pane is open.
pub fn year_picker_divider_visible(pane: DatePickerPane) -> bool {
    YEAR_PICKER_DIVIDER && pane == DatePickerPane::Year
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YearKind {
    Selected,
    Today,
    Default,
}

pub fn clamp_year(year: i32) -> i32 {
    year.clamp(YEAR_RANGE_START, YEAR_RANGE_END)
}

pub fn apply_pane_toggle(pane: DatePickerPane) -> DatePickerPane {
    pane.toggle()
}

pub fn year_window(center: i32) -> [i32; 9] {
    let half = YEAR_WINDOW as i32 / 2;
    let mut start = clamp_year(center) - half;
    if start < YEAR_RANGE_START {
        start = YEAR_RANGE_START;
    }
    if start + YEAR_WINDOW as i32 - 1 > YEAR_RANGE_END {
        start = YEAR_RANGE_END - YEAR_WINDOW as i32 + 1;
    }
    let mut out = [0; 9];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = start + i as i32;
    }
    out
}

pub fn classify_year(year: i32, displayed: i32, today_year: i32) -> YearKind {
    if year == displayed {
        YearKind::Selected
    } else if year == today_year {
        YearKind::Today
    } else {
        YearKind::Default
    }
}

fn date_ord(d: CivilDate) -> i32 {
    d.year * 400 + d.month as i32 * 32 + d.day as i32
}

pub fn range_input_ordered(start: CivilDate, end: CivilDate) -> bool {
    date_ord(start) <= date_ord(end)
}

pub fn is_range_input_valid(start: &str, end: &str) -> bool {
    matches!(range_input_error(start, end), DateInputError::None)
        && parse_input_field(start).is_some()
        && parse_input_field(end).is_some()
}

fn year_out_of_expected_range(d: CivilDate) -> bool {
    d.year < YEAR_RANGE_START || d.year > YEAR_RANGE_END
}

/// Compose `SelectableDates.isSelectableDate` — weekends blocked in the demo.
pub fn is_selectable_date(date: CivilDate) -> bool {
    if !SELECTABLE_DATES {
        return true;
    }
    let wd = weekday_sunday0(date.year, date.month, date.day);
    wd != 0 && wd != 6
}

pub fn not_allowed_label(date: CivilDate) -> String {
    format!("Date not allowed: {}", header_date_label(date))
}

/// Compose `DateInputValidator`: pattern, year range, SelectableDates, then end-before-start.
pub fn range_input_error(start: &str, end: &str) -> DateInputError {
    let start_trim = start.trim();
    let end_trim = end.trim();
    let start_date = if start_trim.is_empty() {
        None
    } else {
        let parsed = parse_input_field(start_trim);
        if parsed.is_none() {
            return DateInputError::Format;
        }
        parsed
    };
    let end_date = if end_trim.is_empty() {
        None
    } else {
        let parsed = parse_input_field(end_trim);
        if parsed.is_none() {
            return DateInputError::Format;
        }
        parsed
    };
    if start_date.is_some_and(year_out_of_expected_range)
        || end_date.is_some_and(year_out_of_expected_range)
    {
        return DateInputError::YearRange;
    }
    if start_date.is_some_and(|d| !is_selectable_date(d))
        || end_date.is_some_and(|d| !is_selectable_date(d))
    {
        return DateInputError::NotAllowed;
    }
    match (start_date, end_date) {
        (Some(s), Some(e)) if !range_input_ordered(s, e) => DateInputError::Order,
        _ => DateInputError::None,
    }
}

/// Compose `DateInputValidator` for a single field: pattern, YearRange, SelectableDates.
pub fn date_input_error(input: &str) -> DateInputError {
    let trim = input.trim();
    if trim.is_empty() {
        return DateInputError::None;
    }
    match parse_input_field(trim) {
        None => DateInputError::Format,
        Some(date) if year_out_of_expected_range(date) => DateInputError::YearRange,
        Some(date) if !is_selectable_date(date) => DateInputError::NotAllowed,
        Some(_) => DateInputError::None,
    }
}

pub fn is_date_input_valid(input: &str) -> bool {
    matches!(date_input_error(input), DateInputError::None) && parse_input_field(input).is_some()
}

pub fn date_in_range_interior(day: CivilDate, start: CivilDate, end: CivilDate) -> bool {
    let o = date_ord(day);
    o > date_ord(start) && o < date_ord(end)
}

pub fn month_grid_range(
    year: i32,
    month: u32,
    start: CivilDate,
    end: CivilDate,
    today: CivilDate,
) -> [(u32, DayKind); 42] {
    let raw = month_grid(year, month);
    let mut out = raw;
    for (i, (day, kind)) in raw.iter().enumerate() {
        if *kind != DayKind::InMonth {
            continue;
        }
        let date = CivilDate {
            year,
            month,
            day: *day,
        };
        out[i] = (
            *day,
            if date == start || date == end {
                DayKind::Selected
            } else if date_in_range_interior(date, start, end) {
                DayKind::InRange
            } else if !is_selectable_date(date) {
                DayKind::Disabled
            } else if date == today {
                DayKind::Today
            } else {
                DayKind::InMonth
            },
        );
    }
    out
}

pub fn month_grid_range_selection(
    year: i32,
    month: u32,
    sel: DateRangeSelection,
    today: CivilDate,
) -> [(u32, DayKind); 42] {
    match (sel.start, sel.end) {
        (Some(start), Some(end)) => month_grid_range(year, month, start, end, today),
        (Some(start), None) => month_grid_classified(year, month, start, today),
        (None, _) => {
            let raw = month_grid(year, month);
            let mut out = raw;
            for (i, (day, kind)) in raw.iter().enumerate() {
                if *kind != DayKind::InMonth {
                    continue;
                }
                let date = CivilDate {
                    year,
                    month,
                    day: *day,
                };
                out[i] = (
                    *day,
                    if !is_selectable_date(date) {
                        DayKind::Disabled
                    } else if date == today {
                        DayKind::Today
                    } else {
                        DayKind::InMonth
                    },
                );
            }
            out
        }
    }
}

/// Picker grid with no selected day (today outline + SelectableDates still apply).
pub fn month_grid_unselected(year: i32, month: u32, today: CivilDate) -> [(u32, DayKind); 42] {
    let raw = month_grid(year, month);
    let mut out = raw;
    for (i, (day, kind)) in raw.iter().enumerate() {
        if *kind == DayKind::InMonth {
            let date = CivilDate {
                year,
                month,
                day: *day,
            };
            out[i] = (
                *day,
                if !is_selectable_date(date) {
                    DayKind::Disabled
                } else if date == today {
                    DayKind::Today
                } else {
                    DayKind::InMonth
                },
            );
        }
    }
    out
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

/// Compose `drawRangeBackground` fill behind a day cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeFill {
    None,
    /// Interior day (full cell, `dayInSelectionRangeContainer`).
    Full,
    /// Start endpoint: right half only (`itemContainerWidth / 2` offset).
    StartHalf,
    /// End endpoint: left half only.
    EndHalf,
}

impl RangeFill {
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Full => "full",
            Self::StartHalf => "start-half",
            Self::EndHalf => "end-half",
        }
    }

    pub const fn is_some(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Compose `itemContainerWidth / 2` start offset (`firstIsSelectionStart`).
    pub const fn connector_left_dp(self, day_dp: f32) -> f32 {
        match self {
            Self::StartHalf => day_dp * 0.5,
            _ => 0.0,
        }
    }

    /// Full interior, or half-cell at a range endpoint.
    pub const fn connector_width_dp(self, day_dp: f32) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Full => day_dp,
            Self::StartHalf | Self::EndHalf => day_dp * 0.5,
        }
    }
}

pub fn range_fill(kind: DayKind, is_range_start: bool, is_range_end: bool) -> RangeFill {
    match kind {
        DayKind::Selected if is_range_start && is_range_end => RangeFill::None,
        DayKind::Selected if is_range_start => RangeFill::StartHalf,
        DayKind::Selected if is_range_end => RangeFill::EndHalf,
        DayKind::InRange => RangeFill::Full,
        _ => RangeFill::None,
    }
}

pub fn range_fills(
    year: i32,
    month: u32,
    cells: [(u32, DayKind); 42],
    sel: DateRangeSelection,
) -> [RangeFill; 42] {
    let mut out = [RangeFill::None; 42];
    if !RANGE_CONNECTOR {
        return out;
    }
    let (Some(start), Some(end)) = (sel.start, sel.end) else {
        return out;
    };
    if start == end {
        return out;
    }
    for (i, (day, kind)) in cells.iter().enumerate() {
        if *kind == DayKind::OutOfMonth {
            continue;
        }
        let date = CivilDate {
            year,
            month,
            day: *day,
        };
        out[i] = range_fill(*kind, date == start, date == end);
    }
    out
}
