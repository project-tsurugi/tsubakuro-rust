use crate::ctype::{SqlSmallInt, SqlUSmallInt};

#[repr(C)]
#[derive(Debug)]
pub struct SqlDateStruct {
    year: SqlSmallInt,
    month: SqlUSmallInt,
    day: SqlUSmallInt,
}

impl std::fmt::Display for SqlDateStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl SqlDateStruct {
    pub(crate) fn new(year: SqlSmallInt, month: SqlUSmallInt, day: SqlUSmallInt) -> SqlDateStruct {
        SqlDateStruct { year, month, day }
    }
}

impl TryFrom<&SqlDateStruct> for time::Date {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &SqlDateStruct) -> Result<Self, Self::Error> {
        time::Date::from_calendar_date(
            value.year as i32,
            time::Month::try_from(value.month as u8)?,
            value.day as u8,
        )
        .map_err(|e| e.into())
    }
}

impl TryFrom<&SqlDateStruct> for time::PrimitiveDateTime {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &SqlDateStruct) -> Result<Self, Self::Error> {
        let date = value.try_into()?;
        let time = time::Time::from_hms(0, 0, 0)?;
        Ok(time::PrimitiveDateTime::new(date, time))
    }
}

impl From<time::Date> for SqlDateStruct {
    fn from(value: time::Date) -> Self {
        let (year, month, day) = value.to_calendar_date();
        SqlDateStruct::new(
            year as SqlSmallInt,
            month as SqlUSmallInt,
            day as SqlUSmallInt,
        )
    }
}

impl From<time::PrimitiveDateTime> for SqlDateStruct {
    fn from(value: time::PrimitiveDateTime) -> Self {
        let (year, month, day) = value.to_calendar_date();
        SqlDateStruct::new(
            year as SqlSmallInt,
            month as SqlUSmallInt,
            day as SqlUSmallInt,
        )
    }
}
