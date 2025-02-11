pub use crate::jogasaki::proto::sql::common::TimeOfDayWithTimeZone as TgTimeOfDayWithTimeZone;

impl TgTimeOfDayWithTimeZone {
    pub fn new(nanoseconds_of_day: u64, time_zone_offset: i32) -> TgTimeOfDayWithTimeZone {
        TgTimeOfDayWithTimeZone {
            offset_nanoseconds: nanoseconds_of_day,
            time_zone_offset,
        }
    }

    pub fn from(
        hour: u8,
        min: u8,
        sec: u8,
        nanoseconds: u32,
        time_zone_offset: i32,
    ) -> TgTimeOfDayWithTimeZone {
        let value = (((hour as u64) * 60 + min as u64) * 60 + sec as u64) * 1_000_000_000
            + nanoseconds as u64;
        TgTimeOfDayWithTimeZone::new(value, time_zone_offset)
    }
}

impl std::fmt::Display for TgTimeOfDayWithTimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nanos = self.offset_nanoseconds % 1_000_000_000;
        let value = self.offset_nanoseconds / 1_000_000_000;
        let sec = value % 60;
        let value = value / 60;
        let min = value % 60;
        let hour = value / 60;

        let offset_sign = if self.time_zone_offset >= 0 { "+" } else { "-" };
        let value = self.time_zone_offset.abs();
        let offset_min = value % 60;
        let offset_hour = value / 60;

        write!(
            f,
            "{hour:02}:{min:02}:{sec:02}.{nanos:09}{offset_sign}{offset_hour:02}:{offset_min:02}"
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_time_of_day() {
        let value = TgTimeOfDayWithTimeZone::from(0, 0, 0, 0, 0);
        assert_eq!("00:00:00.000000000+00:00", value.to_string());

        let value = TgTimeOfDayWithTimeZone::from(1, 2, 3, 12345678, 9 * 60);
        assert_eq!("01:02:03.012345678+09:00", value.to_string());
        let value = TgTimeOfDayWithTimeZone::from(1, 2, 3, 12345678, -9 * 60);
        assert_eq!("01:02:03.012345678-09:00", value.to_string());

        let value = TgTimeOfDayWithTimeZone::from(23, 59, 59, 999_999_999, 0);
        assert_eq!("23:59:59.999999999+00:00", value.to_string());
    }
}
