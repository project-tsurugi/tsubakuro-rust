use crate::ctype::SqlUSmallInt;

#[repr(C)]
#[derive(Debug)]
pub struct SqlTimeStruct {
    hour: SqlUSmallInt,
    minute: SqlUSmallInt,
    second: SqlUSmallInt,
}

impl std::fmt::Display for SqlTimeStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

impl SqlTimeStruct {
    pub(crate) fn new(
        hour: SqlUSmallInt,
        minute: SqlUSmallInt,
        second: SqlUSmallInt,
    ) -> SqlTimeStruct {
        SqlTimeStruct {
            hour,
            minute,
            second,
        }
    }
}

impl TryFrom<&SqlTimeStruct> for time::Time {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &SqlTimeStruct) -> Result<Self, Self::Error> {
        time::Time::from_hms(value.hour as u8, value.minute as u8, value.second as u8)
            .map_err(|e| e.into())
    }
}

impl From<time::Time> for SqlTimeStruct {
    fn from(value: time::Time) -> Self {
        SqlTimeStruct::new(
            value.hour() as SqlUSmallInt,
            value.minute() as SqlUSmallInt,
            value.second() as SqlUSmallInt,
        )
    }
}

impl From<time::PrimitiveDateTime> for SqlTimeStruct {
    fn from(value: time::PrimitiveDateTime) -> Self {
        SqlTimeStruct::new(
            value.hour() as SqlUSmallInt,
            value.minute() as SqlUSmallInt,
            value.second() as SqlUSmallInt,
        )
    }
}
