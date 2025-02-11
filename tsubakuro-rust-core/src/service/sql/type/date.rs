#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TgDate {
    /// date (number of days offset of epoch 1970-01-01).
    pub epoch_days: i64,
}

impl TgDate {
    pub fn new(epoch_days: i64) -> TgDate {
        TgDate { epoch_days }
    }
}
