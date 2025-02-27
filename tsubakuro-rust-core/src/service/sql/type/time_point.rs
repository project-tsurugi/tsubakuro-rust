pub use crate::jogasaki::proto::sql::common::TimePoint as TgTimePoint;

impl TgTimePoint {
    /// Creates a new instance.
    pub fn new(epoch_seconds: i64, nanos: u32) -> TgTimePoint {
        TgTimePoint {
            offset_seconds: epoch_seconds,
            nano_adjustment: nanos,
        }
    }
}
