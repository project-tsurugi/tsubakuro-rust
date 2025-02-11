#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TgTimeOfDay {
    /// time of day (nano-seconds since 00:00:00).
    pub offset_nanoseconds: u64,
}

impl TgTimeOfDay {
    pub fn new(nanoseconds_of_day: u64) -> TgTimeOfDay {
        TgTimeOfDay {
            offset_nanoseconds: nanoseconds_of_day,
        }
    }

    pub fn from(hour: u8, min: u8, sec: u8, nanoseconds: u32) -> TgTimeOfDay {
        let value = (((hour as u64) * 60 + min as u64) * 60 + sec as u64) * 1_000_000_000
            + nanoseconds as u64;
        TgTimeOfDay::new(value)
    }
}

impl std::fmt::Display for TgTimeOfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nanos = self.offset_nanoseconds % 1_000_000_000;
        let value = self.offset_nanoseconds / 1_000_000_000;
        let sec = value % 60;
        let value = value / 60;
        let min = value % 60;
        let hour = value / 60;
        write!(f, "{hour:02}:{min:02}:{sec:02}.{nanos:09}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_time_of_day() {
        let value = TgTimeOfDay::from(0, 0, 0, 0);
        assert_eq!("00:00:00.000000000", value.to_string());

        let value = TgTimeOfDay::from(1, 2, 3, 12345678);
        assert_eq!("01:02:03.012345678", value.to_string());

        let value = TgTimeOfDay::from(23, 59, 59, 999_999_999);
        assert_eq!("23:59:59.999999999", value.to_string());
    }
}
