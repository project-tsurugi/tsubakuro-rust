use crate::ctype::{SqlInteger, SqlSmallInt, SqlUSmallInt};

#[repr(C)]
#[derive(Debug)]
pub struct SqlTimestampStruct {
    year: SqlSmallInt,
    month: SqlUSmallInt,
    day: SqlUSmallInt,
    hour: SqlUSmallInt,
    minute: SqlUSmallInt,
    second: SqlUSmallInt,
    fraction: SqlInteger, // nanosecond
}

impl std::fmt::Display for SqlTimestampStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, self.fraction
        )
    }
}

impl SqlTimestampStruct {
    pub(crate) fn new(
        year: SqlSmallInt,
        month: SqlUSmallInt,
        day: SqlUSmallInt,
        hour: SqlUSmallInt,
        minute: SqlUSmallInt,
        second: SqlUSmallInt,
        fraction: SqlInteger,
    ) -> SqlTimestampStruct {
        SqlTimestampStruct {
            year,
            month,
            day,
            hour,
            minute,
            second,
            fraction,
        }
    }
}

impl TryFrom<&SqlTimestampStruct> for time::Date {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &SqlTimestampStruct) -> Result<Self, Self::Error> {
        time::Date::from_calendar_date(
            value.year as i32,
            time::Month::try_from(value.month as u8)?,
            value.day as u8,
        )
        .map_err(|e| e.into())
    }
}

impl TryFrom<&SqlTimestampStruct> for time::Time {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &SqlTimestampStruct) -> Result<Self, Self::Error> {
        time::Time::from_hms_nano(
            value.hour as u8,
            value.minute as u8,
            value.second as u8,
            value.fraction as u32,
        )
        .map_err(|e| e.into())
    }
}

impl TryFrom<&SqlTimestampStruct> for time::PrimitiveDateTime {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &SqlTimestampStruct) -> Result<Self, Self::Error> {
        let date = value.try_into()?;
        let time = value.try_into()?;
        Ok(time::PrimitiveDateTime::new(date, time))
    }
}

impl From<time::Date> for SqlTimestampStruct {
    fn from(value: time::Date) -> Self {
        let (year, month, day) = value.to_calendar_date();
        SqlTimestampStruct::new(
            year as SqlSmallInt,
            month as SqlUSmallInt,
            day as SqlUSmallInt,
            0,
            0,
            0,
            0,
        )
    }
}

impl From<time::PrimitiveDateTime> for SqlTimestampStruct {
    fn from(value: time::PrimitiveDateTime) -> Self {
        let (year, month, day) = value.to_calendar_date();
        SqlTimestampStruct::new(
            year as SqlSmallInt,
            month as SqlUSmallInt,
            day as SqlUSmallInt,
            value.hour() as SqlUSmallInt,
            value.minute() as SqlUSmallInt,
            value.second() as SqlUSmallInt,
            value.nanosecond() as SqlInteger,
        )
    }
}

impl From<time::OffsetDateTime> for SqlTimestampStruct {
    fn from(value: time::OffsetDateTime) -> Self {
        let (year, month, day) = value.to_calendar_date();
        SqlTimestampStruct::new(
            year as SqlSmallInt,
            month as SqlUSmallInt,
            day as SqlUSmallInt,
            value.hour() as SqlUSmallInt,
            value.minute() as SqlUSmallInt,
            value.second() as SqlUSmallInt,
            value.nanosecond() as SqlInteger,
        )
    }
}
