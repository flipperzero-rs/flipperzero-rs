//! Locale handling.

use core::ffi::CStr;

use crate::datetime::DateTime;
use crate::furi::string::FuriString;

use flipperzero_sys as sys;

pub use sys::LocaleMeasurementUnits as MeasurementUnits;
pub use sys::LocaleMeasurementUnitsImperial as MeasurementUnitsImperial;
pub use sys::LocaleMeasurementUnitsMetric as MeasurementUnitsMetric;

pub use sys::LocaleTimeFormat as TimeFormat;
pub use sys::LocaleTimeFormat12h as TimeFormat12h;
pub use sys::LocaleTimeFormat24h as TimeFormat24h;

pub use sys::LocaleDateFormat as DateFormat;
pub use sys::LocaleDateFormatDMY as DateFormatDMY;
pub use sys::LocaleDateFormatMDY as DateFormatMDY;
pub use sys::LocaleDateFormatYMD as DateFormatYMD;

/// Get locale measurement units.
pub fn measurement_unit() -> MeasurementUnits {
    unsafe { sys::locale_get_measurement_unit() }
}

/// Set locale measurement units.
pub fn set_measurement_unit(format: MeasurementUnits) {
    unsafe { sys::locale_set_measurement_unit(format) }
}

/// Convert Fahrenheit to Celsius.
pub fn fahrenheit_to_celsius(temp_f: f32) -> f32 {
    unsafe { sys::locale_fahrenheit_to_celsius(temp_f) }
}

/// Convert Celsius to Fahrenheit.
pub fn celsius_to_fahrenheit(temp_c: f32) -> f32 {
    unsafe { sys::locale_celsius_to_fahrenheit(temp_c) }
}

/// Get locale time format.
pub fn time_format() -> TimeFormat {
    unsafe { sys::locale_get_time_format() }
}

/// Set locale time format.
pub fn set_time_format(format: TimeFormat) {
    unsafe { sys::locale_set_time_format(format) }
}

/// Format time to furi string.
pub fn format_time(datetime: &DateTime, show_seconds: bool) -> FuriString {
    format_time_ex(datetime, time_format(), show_seconds)
}

/// Format time to furi string.
pub fn format_time_ex(datetime: &DateTime, format: TimeFormat, show_seconds: bool) -> FuriString {
    let mut string = FuriString::new();
    unsafe {
        sys::locale_format_time(
            string.as_mut_ptr(),
            datetime as *const _,
            format,
            show_seconds,
        )
    }

    string
}

/// Get locale date format.
pub fn date_format() -> DateFormat {
    unsafe { sys::locale_get_date_format() }
}

/// Set locale date format.
pub fn set_date_format(format: DateFormat) {
    unsafe { sys::locale_set_date_format(format) }
}

/// Format date to furi string.
pub fn format_date(datetime: &DateTime) -> FuriString {
    format_date_ex(datetime, date_format(), c"-")
}

/// Format date to furi string.
pub fn format_date_ex(datetime: &DateTime, format: DateFormat, separator: &CStr) -> FuriString {
    let mut string = FuriString::new();
    unsafe {
        sys::locale_format_date(
            string.as_mut_ptr(),
            datetime as *const _,
            format,
            separator.as_ptr(),
        )
    }

    string
}
