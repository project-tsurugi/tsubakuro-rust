mod bigdecimal0;
mod bigdecimal1;
mod binary;
mod blob;
mod boolean;
mod char;
mod chrono_date_time;
mod chrono_date_time_utc;
mod chrono_naive_date;
mod chrono_naive_date_time;
mod chrono_naive_time;
mod chrono_naive_time_with_offset;
mod clob;
mod date;
mod decimal0;
mod decimal1;
mod float4;
mod float8;
mod int4;
mod int8;
mod rust_decimal0;
mod rust_decimal1;
mod time_date;
mod time_of_day;
mod time_of_day_with_time_zone;
mod time_offset_date_time;
mod time_point;
mod time_point_with_time_zone;
mod time_primitive_date_time;
mod time_time;
mod time_time_with_offset;
mod varbinary;
mod varchar;

#[cfg(test)]
fn epoch_days(year: i32, month: u8, day: u8) -> i64 {
    match (year, month, day) {
        (-1, 1, 1) => -719893,
        (0, 1, 1) => -719528,
        (1, 1, 1) => -719162,
        (1969, 12, 31) => -1,
        (1970, 1, 1) => 0,
        (9999, 12, 31) => 2932896,
        (2025, 1, 16) => 20104,
        (2025, 2, 7) => 20126,
        _ => panic!("unsupported {year:04}-{month:02}-{day:02}"),
    }
}

#[cfg(test)]
fn epoch_days_to_string(value: i64) -> &'static str {
    match value {
        -719893 => "-0001-01-01",
        -719528 => "0000-01-01",
        -719162 => "0001-01-01",
        -1 => "1969-12-31",
        0 => "1970-01-01",
        20104 => "2025-01-16",
        20126 => "2025-02-07",
        2932896 => "9999-12-31",
        _ => panic!("unsupported {}", value),
    }
}

#[cfg(test)]
fn seconds_of_day(hour: u8, min: u8, sec: u8) -> u32 {
    ((hour as u32) * 60 + (min as u32)) * 60 + (sec as u32)
}

#[cfg(test)]
fn seconds_of_day_to_string(value: u32, nanos: u32) -> String {
    let (hour, min, sec) = seconds_of_day_to_hms(value);
    format!("{hour:02}:{min:02}:{sec:02}.{nanos:09}")
}

#[cfg(test)]
fn seconds_of_day_to_hms(value: u32) -> (u8, u8, u8) {
    let sec = value % 60;
    let value = value / 60;
    let min = value % 60;
    let hour = value / 60;
    (hour as u8, min as u8, sec as u8)
}
