#![allow(clippy::doc_markdown)]

/// Nanoseconds per microsecond
pub const NANOS_PER_MICRO: u64 = 1_000;
/// Nanoseconds per millisecond
pub const NANOS_PER_MILLI: u64 = NANOS_PER_MICRO * MICROS_PER_MILLI;
/// Nanoseconds per second
pub const NANOS_PER_SEC: u64 = NANOS_PER_MILLI * MILLIS_PER_SEC;
/// Nanoseconds per minute
pub const NANOS_PER_MIN: u64 = NANOS_PER_SEC * SECS_PER_MIN;
/// Nanoseconds per hour
pub const NANOS_PER_HOUR: u64 = NANOS_PER_MIN * MINS_PER_HOUR;
/// Nanoseconds per day
pub const NANOS_PER_DAY: u64 = NANOS_PER_HOUR * HOURS_PER_DAY;
/// Nanoseconds per week
pub const NANOS_PER_WEEK: u64 = NANOS_PER_DAY * DAYS_PER_WEEK;
/// Nanoseconds per year
pub const NANOS_PER_YEAR: u64 = NANOS_PER_DAY * DAYS_PER_YEAR;
/// Nanoseconds per leap year
pub const NANOS_PER_LEAP_YEAR: u64 = NANOS_PER_DAY * DAYS_PER_LEAP_YEAR;

/// Microseconds per millisecond
pub const MICROS_PER_MILLI: u64 = 1_000;
/// Microseconds per second
pub const MICROS_PER_SEC: u64 = MICROS_PER_MILLI * MILLIS_PER_SEC;
/// Microseconds per minute
pub const MICROS_PER_MIN: u64 = MICROS_PER_SEC * SECS_PER_MIN;
/// Microseconds per hour
pub const MICROS_PER_HOUR: u64 = MICROS_PER_MIN * MINS_PER_HOUR;
/// Microseconds per day
pub const MICROS_PER_DAY: u64 = MICROS_PER_HOUR * HOURS_PER_DAY;
/// Microseconds per week
pub const MICROS_PER_WEEK: u64 = MICROS_PER_DAY * DAYS_PER_WEEK;
/// Microseconds per year
pub const MICROS_PER_YEAR: u64 = MICROS_PER_DAY * DAYS_PER_YEAR;
/// Microseconds per leap year
pub const MICROS_PER_LEAP_YEAR: u64 = MICROS_PER_DAY * DAYS_PER_LEAP_YEAR;

/// Milliseconds per second
pub const MILLIS_PER_SEC: u64 = 1_000;
/// Milliseconds per minute
pub const MILLIS_PER_MIN: u64 = MILLIS_PER_SEC * SECS_PER_MIN;
/// Milliseconds per hour
pub const MILLIS_PER_HOUR: u64 = MILLIS_PER_MIN * MINS_PER_HOUR;
/// Milliseconds per day
pub const MILLIS_PER_DAY: u64 = MILLIS_PER_HOUR * HOURS_PER_DAY;
/// Milliseconds per week
pub const MILLIS_PER_WEEK: u64 = MILLIS_PER_DAY * DAYS_PER_WEEK;
/// Milliseconds per year
pub const MILLIS_PER_YEAR: u64 = MILLIS_PER_DAY * DAYS_PER_YEAR;
/// Milliseconds per leap year
pub const MILLIS_PER_LEAP_YEAR: u64 = MILLIS_PER_DAY * DAYS_PER_LEAP_YEAR;

/// Seconds per minute
pub const SECS_PER_MIN: u64 = 60;
/// Seconds per hour
pub const SECS_PER_HOUR: u64 = SECS_PER_MIN * MINS_PER_HOUR;
/// Seconds per day
pub const SECS_PER_DAY: u64 = SECS_PER_HOUR * HOURS_PER_DAY;
/// Seconds per week
pub const SECS_PER_WEEK: u64 = SECS_PER_DAY * DAYS_PER_WEEK;
/// Seconds per year
pub const SECS_PER_YEAR: u64 = SECS_PER_DAY * DAYS_PER_YEAR;
/// Seconds per leap year
pub const SECS_PER_LEAP_YEAR: u64 = SECS_PER_DAY * DAYS_PER_LEAP_YEAR;

/// Minutes per hour
pub const MINS_PER_HOUR: u64 = 60;
/// Minutes per day
pub const MINS_PER_DAY: u64 = MINS_PER_HOUR * HOURS_PER_DAY;
/// Minutes per week
pub const MINS_PER_WEEK: u64 = MINS_PER_DAY * DAYS_PER_WEEK;
/// Minutes per year
pub const MINS_PER_YEAR: u64 = MINS_PER_DAY * DAYS_PER_YEAR;
/// Minutes per leap year
pub const MINS_PER_LEAP_YEAR: u64 = MINS_PER_DAY * DAYS_PER_LEAP_YEAR;

/// Hours per day
pub const HOURS_PER_DAY: u64 = 24;
/// Hours per week
pub const HOURS_PER_WEEK: u64 = HOURS_PER_DAY * DAYS_PER_WEEK;
/// Hours per year
pub const HOURS_PER_YEAR: u64 = HOURS_PER_DAY * DAYS_PER_YEAR;
/// Hours per leap year
pub const HOURS_PER_LEAP_YEAR: u64 = HOURS_PER_DAY * DAYS_PER_LEAP_YEAR;

/// Days per week
pub const DAYS_PER_WEEK: u64 = 7;

/// Days per year NOT [`DAYS_PER_LEAP_YEAR`]
pub const DAYS_PER_YEAR: u64 = 365;

/// Days per leap year NOT [`DAYS_PER_YEAR`]
pub const DAYS_PER_LEAP_YEAR: u64 = 366;

/// Weeks per year
pub const WEEKS_PER_YEAR: f64 = DAYS_PER_YEAR as f64 / DAYS_PER_WEEK as f64;

/// Weeks per leap year
pub const WEEKS_PER_LEAP_YEAR: f64 =
    DAYS_PER_LEAP_YEAR as f64 / DAYS_PER_WEEK as f64;

// TODO: For the below; actually use the smallest

/// Microseconds in one nanosecond: 1 / 1_000
/// Minimum precision: f16 (11-bit mantissa is sufficient for 3 decimal places)
pub const MICROS_PER_NANO: f64 = 1.0 / NANOS_PER_MICRO as f64;

/// Milliseconds in one nanosecond: 1 / 1_000_000
/// Minimum precision: f32 (24-bit mantissa needed for 6 decimal places)
pub const MILLIS_PER_NANO: f64 = 1.0 / NANOS_PER_MILLI as f64;

/// Seconds in one nanosecond: 1 / 1_000_000_000
/// Minimum precision: f64 (53-bit mantissa needed for 9 decimal places)
pub const SECS_PER_NANO: f64 = 1.0 / NANOS_PER_SEC as f64;

/// Minutes in one nanosecond: 1 / 60_000_000_000
/// Minimum precision: f64 (53-bit mantissa needed)
pub const MINS_PER_NANO: f64 = 1.0 / NANOS_PER_MIN as f64;

/// Hours in one nanosecond: 1 / 3_600_000_000_000
/// Minimum precision: f64 (53-bit mantissa needed)
pub const HOURS_PER_NANO: f64 = 1.0 / NANOS_PER_HOUR as f64;

/// Days in one nanosecond: 1 / 86_400_000_000_000
/// Minimum precision: f64 (53-bit mantissa needed)
pub const DAYS_PER_NANO: f64 = 1.0 / NANOS_PER_DAY as f64;

/// Weeks in one nanosecond: 1 / 604_800_000_000_000
/// Minimum precision: f64 (53-bit mantissa needed)
pub const WEEKS_PER_NANO: f64 = 1.0 / NANOS_PER_WEEK as f64;

/// Years in one nanosecond: 1 / 31_536_000_000_000_000
/// Minimum precision: f64 (53-bit mantissa needed)
pub const YEARS_PER_NANO: f64 = 1.0 / NANOS_PER_YEAR as f64;

/// Leap years in one nanosecond: 1 / 31_622_400_000_000_000
/// Minimum precision: f64 (53-bit mantissa needed)
pub const LEAP_YEARS_PER_NANO: f64 = 1.0 / NANOS_PER_LEAP_YEAR as f64;

/// Milliseconds in one microsecond: 1 / 1_000
/// Minimum precision: f16 (11-bit mantissa is sufficient for 3 decimal places)
pub const MILLIS_PER_MICRO: f64 = 1.0 / MICROS_PER_MILLI as f64;

/// Seconds in one microsecond: 1 / 1_000_000
/// Minimum precision: f32 (24-bit mantissa needed for 6 decimal places)
pub const SECS_PER_MICRO: f64 = 1.0 / MICROS_PER_SEC as f64;

/// Minutes in one microsecond: 1 / 60_000_000
/// Minimum precision: f32 (24-bit mantissa needed)
pub const MINS_PER_MICRO: f64 = 1.0 / MICROS_PER_MIN as f64;

/// Hours in one microsecond: 1 / 3_600_000_000
/// Minimum precision: f32 (24-bit mantissa needed)
pub const HOURS_PER_MICRO: f64 = 1.0 / MICROS_PER_HOUR as f64;

/// Days in one microsecond: 1 / 86_400_000_000
/// Minimum precision: f32 (24-bit mantissa needed)
pub const DAYS_PER_MICRO: f64 = 1.0 / MICROS_PER_DAY as f64;

/// Weeks in one microsecond: 1 / 604_800_000_000
/// Minimum precision: f32 (24-bit mantissa needed)
pub const WEEKS_PER_MICRO: f64 = 1.0 / MICROS_PER_WEEK as f64;

/// Years in one microsecond: 1 / 31_536_000_000_000
/// Minimum precision: f64 (53-bit mantissa needed)
pub const YEARS_PER_MICRO: f64 = 1.0 / MICROS_PER_YEAR as f64;

/// Leap years in one microsecond: 1 / 31_622_400_000_000
/// Minimum precision: f64 (53-bit mantissa needed)
pub const LEAP_YEARS_PER_MICRO: f64 = 1.0 / MICROS_PER_LEAP_YEAR as f64;

/// Seconds in one millisecond: 1 / 1_000
/// Minimum precision: f16 (11-bit mantissa is sufficient for 3 decimal places)
pub const SECS_PER_MILLI: f64 = 1.0 / MILLIS_PER_SEC as f64;

/// Minutes in one millisecond: 1 / 60_000
/// Minimum precision: f32 (24-bit mantissa needed)
pub const MINS_PER_MILLI: f64 = 1.0 / MILLIS_PER_MIN as f64;

/// Hours in one millisecond: 1 / 3_600_000
/// Minimum precision: f32 (24-bit mantissa needed)
pub const HOURS_PER_MILLI: f64 = 1.0 / MILLIS_PER_HOUR as f64;

/// Days in one millisecond: 1 / 86_400_000
/// Minimum precision: f32 (24-bit mantissa needed)
pub const DAYS_PER_MILLI: f64 = 1.0 / MILLIS_PER_DAY as f64;

/// Weeks in one millisecond: 1 / 604_800_000
/// Minimum precision: f32 (24-bit mantissa needed)
pub const WEEKS_PER_MILLI: f64 = 1.0 / MILLIS_PER_WEEK as f64;

/// Years in one millisecond: 1 / 31_536_000_000
/// Minimum precision: f64 (53-bit mantissa needed)
pub const YEARS_PER_MILLI: f64 = 1.0 / MILLIS_PER_YEAR as f64;

/// Leap years in one millisecond: 1 / 31_622_400_000
/// Minimum precision: f64 (53-bit mantissa needed)
pub const LEAP_YEARS_PER_MILLI: f64 = 1.0 / MILLIS_PER_LEAP_YEAR as f64;

/// Minutes in one second: 1 / 60
/// Minimum precision: f16 (11-bit mantissa sufficient; ~0.0167)
pub const MINS_PER_SEC: f64 = 1.0 / SECS_PER_MIN as f64;

/// Hours in one second: 1 / 3_600
/// Minimum precision: f32 (24-bit mantissa needed; ~0.000278)
pub const HOURS_PER_SEC: f64 = 1.0 / SECS_PER_HOUR as f64;

/// Days in one second: 1 / 86_400
/// Minimum precision: f32 (24-bit mantissa needed; ~0.0000116)
pub const DAYS_PER_SEC: f64 = 1.0 / SECS_PER_DAY as f64;

/// Weeks in one second: 1 / 604_800
/// Minimum precision: f32 (24-bit mantissa needed; ~0.00000165)
pub const WEEKS_PER_SEC: f64 = 1.0 / SECS_PER_WEEK as f64;

/// Years in one second: 1 / 31_536_000
/// Minimum precision: f32 (24-bit mantissa needed)
pub const YEARS_PER_SEC: f64 = 1.0 / SECS_PER_YEAR as f64;

/// Leap years in one second: 1 / 31_622_400
/// Minimum precision: f32 (24-bit mantissa needed)
pub const LEAP_YEARS_PER_SEC: f64 = 1.0 / SECS_PER_LEAP_YEAR as f64;

/// Hours in one minute: 1 / 60
/// Minimum precision: f16 (11-bit mantissa sufficient; ~0.0167)
pub const HOURS_PER_MIN: f64 = 1.0 / MINS_PER_HOUR as f64;

/// Days in one minute: 1 / 1_440
/// Minimum precision: f32 (24-bit mantissa needed; ~0.000694)
pub const DAYS_PER_MIN: f64 = 1.0 / MINS_PER_DAY as f64;

/// Weeks in one minute: 1 / 10_080
/// Minimum precision: f32 (24-bit mantissa needed; ~0.0000992)
pub const WEEKS_PER_MIN: f64 = 1.0 / MINS_PER_WEEK as f64;

/// Years in one minute: 1 / 525_600
/// Minimum precision: f32 (24-bit mantissa needed)
pub const YEARS_PER_MIN: f64 = 1.0 / MINS_PER_YEAR as f64;

/// Leap years in one minute: 1 / 527_040
/// Minimum precision: f32 (24-bit mantissa needed)
pub const LEAP_YEARS_PER_MIN: f64 = 1.0 / MINS_PER_LEAP_YEAR as f64;

/// Days in one hour: 1 / 24
/// Minimum precision: f16 (11-bit mantissa sufficient; ~0.0417)
pub const DAYS_PER_HOUR: f64 = 1.0 / HOURS_PER_DAY as f64;

/// Weeks in one hour: 1 / 168
/// Minimum precision: f16 (11-bit mantissa sufficient; ~0.00595)
pub const WEEKS_PER_HOUR: f64 = 1.0 / HOURS_PER_WEEK as f64;

/// Years in one hour: 1 / 8_760
/// Minimum precision: f16 (11-bit mantissa sufficient)
pub const YEARS_PER_HOUR: f64 = 1.0 / HOURS_PER_YEAR as f64;

/// Leap years in one hour: 1 / 8_784
/// Minimum precision: f16 (11-bit mantissa sufficient)
pub const LEAP_YEARS_PER_HOUR: f64 = 1.0 / HOURS_PER_LEAP_YEAR as f64;

/// Weeks in one day: 1 / 7
/// Minimum precision: f16 (11-bit mantissa sufficient; ~0.143)
pub const WEEKS_PER_DAY: f64 = 1.0 / DAYS_PER_WEEK as f64;

/// Years in one day: 1 / 365
/// Minimum precision: f16 (11-bit mantissa sufficient)
pub const YEARS_PER_DAY: f64 = 1.0 / DAYS_PER_YEAR as f64;

/// Leap years in one day: 1 / 366
/// Minimum precision: f16 (11-bit mantissa sufficient)
pub const LEAP_YEARS_PER_DAY: f64 = 1.0 / DAYS_PER_LEAP_YEAR as f64;

/// Years in one week: 1 / 52
/// Minimum precision: f16 (11-bit mantissa sufficient)
pub const YEARS_PER_WEEK: f64 = 1.0 / WEEKS_PER_YEAR;

/// Leap years in one week: 1 / 52
/// Minimum precision: f16 (11-bit mantissa sufficient)
pub const LEAP_YEARS_PER_WEEK: f64 = 1.0 / WEEKS_PER_LEAP_YEAR;

/// Returns the amount of days within a month, accurately taking the year into account
///
/// If you want to force a leap year, use 0
/// If you want to force a non leap year, use 1
///
/// TODO: Create functions for all other time values
#[must_use]
pub const fn days_in_month(month: u8, year: u64) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if is_leap_year(year) {
            29
        } else {
            28
        }),
        _ => None,
    }
}
#[must_use]
/// Checks if a given year is a leap year according to the Gregorian calendar.
///
/// A year is a leap year if it is divisible by 4,
/// except for end-of-century years, which must also be divisible by 400.
pub const fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100))
        || year.is_multiple_of(400)
}

#[must_use]
/// Returns the number of days in a specific year
pub const fn days_in_year(year: u64) -> u64 {
    if std::intrinsics::unlikely(is_leap_year(year)) {
        DAYS_PER_LEAP_YEAR
    } else {
        DAYS_PER_YEAR
    }
}
#[must_use]
/// Returns the total number of nanoseconds in a specific year
pub const fn nanos_in_year(year: u64) -> u64 {
    days_in_year(year) * NANOS_PER_DAY
}
#[must_use]
/// Returns the total number of microseconds in a specific year
pub const fn micros_in_year(year: u64) -> u64 {
    days_in_year(year) * MICROS_PER_DAY
}
#[must_use]
/// Returns the total number of milliseconds in a specific year
pub const fn millis_in_year(year: u64) -> u64 {
    days_in_year(year) * MILLIS_PER_DAY
}

#[must_use]
/// Returns the total number of seconds in a specific year
pub const fn secs_in_year(year: u64) -> u64 {
    days_in_year(year) * SECS_PER_DAY
}

#[must_use]
/// Returns the total number of hours in a specific year
pub const fn hours_in_year(year: u64) -> u64 {
    days_in_year(year) * HOURS_PER_DAY
}
