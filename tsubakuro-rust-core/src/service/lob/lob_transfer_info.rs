use crate::{
    session::lob_transfer_type::LobTransferType,
    tateyama::proto::endpoint::response::{
        handshake::success::BlobTransfer as ProtoBlobTransfer, BlobRelayServiceInfo,
    },
};

/// Large object transfer info.
///
/// since 0.10.0
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum LobTransferInfo {
    /// Does not use transfer type.
    NotUse,

    /// Privileged transfer type.
    Privileged,

    /// Blob Relay transfer type.
    Relay(BlobRelayServiceInfo),
}

impl From<Option<ProtoBlobTransfer>> for LobTransferInfo {
    fn from(value: Option<ProtoBlobTransfer>) -> Self {
        use ProtoBlobTransfer::*;
        match value {
            Some(PrivilegedMode(_)) => LobTransferInfo::Privileged,
            Some(BlobRelayServiceInfo(info)) => LobTransferInfo::Relay(info),
            None => LobTransferInfo::NotUse,
        }
    }
}

impl From<LobTransferInfo> for LobTransferType {
    fn from(value: LobTransferInfo) -> Self {
        match value {
            LobTransferInfo::NotUse => LobTransferType::NotUse,
            LobTransferInfo::Privileged => LobTransferType::Privileged,
            LobTransferInfo::Relay(_) => LobTransferType::Relay,
        }
    }
}
