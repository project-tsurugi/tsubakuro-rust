//! Session type.

use tsubakuro_rust_core::prelude::*;

/// Large object transfer type.
///
/// since 0.10.0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TsurugiFfiLobTransferType {
    /// Indicates the default transfer policy.
    Default = 0,

    /// Does not use transfer type.
    NotUse = 1,

    /// Privileged transfer type.
    Privileged = 2,

    /// Blob Relay transfer type.
    Relay = 3,
}

impl From<TsurugiFfiLobTransferType> for LobTransferType {
    fn from(value: TsurugiFfiLobTransferType) -> Self {
        match value {
            TsurugiFfiLobTransferType::Default => Self::Default,
            TsurugiFfiLobTransferType::NotUse => Self::NotUse,
            TsurugiFfiLobTransferType::Privileged => Self::Privileged,
            TsurugiFfiLobTransferType::Relay => Self::Relay,
        }
    }
}

impl From<LobTransferType> for TsurugiFfiLobTransferType {
    fn from(value: LobTransferType) -> Self {
        match value {
            LobTransferType::Default => Self::Default,
            LobTransferType::NotUse => Self::NotUse,
            LobTransferType::Privileged => Self::Privileged,
            LobTransferType::Relay => Self::Relay,
        }
    }
}

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
