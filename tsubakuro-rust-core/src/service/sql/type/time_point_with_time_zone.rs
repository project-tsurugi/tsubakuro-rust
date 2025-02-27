pub use crate::jogasaki::proto::sql::common::TimePointWithTimeZone as TgTimePointWithTimeZone;

impl TgTimePointWithTimeZone {
    /// Creates a new instance.
    pub fn new(epoch_seconds: i64, nanos: u32, time_zone_offset: i32) -> TgTimePointWithTimeZone {
        TgTimePointWithTimeZone {
            offset_seconds: epoch_seconds,
            nano_adjustment: nanos,
            time_zone_offset,
        }
    }
}
