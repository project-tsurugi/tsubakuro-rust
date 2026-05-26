use crate::{
    jogasaki::proto::sql::common::LargeObjectProvider, service::lob::lob_client::RemoteLob,
};

use super::large_object::TgLargeObjectReference;

/// BLOB.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TgBlob {
    pub(crate) inner: InnerBlob,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InnerBlob {
    /// BLOB with client path.
    Path(String),
    /// BLOB with bytes.
    Contents(Vec<u8>),
    /// BLOB with uploaded lob.
    RemoteLob(RemoteLob),
}

impl TgBlob {
    /// Creates a new instance.
    #[deprecated(since = "0.10.0", note = "Use SqlClient::upload_blob_file instead")]
    pub fn new(path: &str) -> TgBlob {
        TgBlob {
            inner: InnerBlob::Path(path.to_string()),
        }
    }

    pub(crate) fn from_remote_lob(remote_lob: RemoteLob) -> TgBlob {
        TgBlob {
            inner: InnerBlob::RemoteLob(remote_lob),
        }
    }
}

impl From<Vec<u8>> for TgBlob {
    // #[deprecated(since = "0.10.0", note = "Use SqlClient::upload_blob instead")]
    fn from(value: Vec<u8>) -> Self {
        TgBlob {
            inner: InnerBlob::Contents(value),
        }
    }
}

/// BLOB for [SqlQueryResult](crate::prelude::SqlQueryResult).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TgBlobReference {
    provider: LargeObjectProvider,
    object_id: u64,
    reference_tag: u64,
}

impl TgBlobReference {
    pub(crate) fn new(
        provider: LargeObjectProvider,
        object_id: u64,
        reference_tag: u64,
    ) -> TgBlobReference {
        TgBlobReference {
            provider,
            object_id,
            reference_tag,
        }
    }
}

impl TgLargeObjectReference for TgBlobReference {
    fn provider(&self) -> LargeObjectProvider {
        self.provider
    }

    fn object_id(&self) -> u64 {
        self.object_id
    }

    fn reference_tag(&self) -> u64 {
        self.reference_tag
    }
}
