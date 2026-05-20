use crate::tateyama::proto::endpoint::request::{
    BlobTransferMedium, BlobTransferType as ProtoLobTransferType,
};

/// Large object transfer type.
///
/// since 0.10.0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LobTransferType {
    /// Indicates the default transfer policy.
    Default,

    /// Does not use transfer type.
    NotUse,

    /// Privileged transfer type.
    Privileged,

    /// Blob Relay transfer type.
    Relay,
}

impl LobTransferType {
    pub(crate) fn to_proto(&self) -> Vec<BlobTransferMedium> {
        use ProtoLobTransferType::*;
        match self {
            LobTransferType::Default => vec![Relay.into(), DoesNotUse.into()],
            LobTransferType::NotUse => vec![DoesNotUse.into()],
            LobTransferType::Privileged => vec![Privileged.into()],
            LobTransferType::Relay => vec![Relay.into()],
        }
    }
}

impl From<ProtoLobTransferType> for BlobTransferMedium {
    fn from(value: ProtoLobTransferType) -> Self {
        BlobTransferMedium {
            blob_transfer_type: value as i32,
        }
    }
}
