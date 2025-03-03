use tsubakuro_rust_core::prelude::*;

/// Shutdown type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TsurugiFfiShutdownType {
    /// The default shutdown type.
    NotSet = 0,
    /// Waits for the ongoing requests and safely shutdown the session.
    Graceful = 1,
    /// Cancelling the ongoing requests and safely shutdown the session.
    Forceful = 2,
}

impl From<TsurugiFfiShutdownType> for ShutdownType {
    fn from(value: TsurugiFfiShutdownType) -> Self {
        match value {
            TsurugiFfiShutdownType::NotSet => Self::NotSet,
            TsurugiFfiShutdownType::Graceful => Self::Graceful,
            TsurugiFfiShutdownType::Forceful => Self::Forceful,
        }
    }
}
