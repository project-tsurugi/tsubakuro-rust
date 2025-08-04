use crate::jogasaki::proto::sql::common::LargeObjectProvider;

use super::large_object::TgLargeObjectReference;

/// BLOB.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TgBlob {
    /// BLOB with path.
    Path(String),
    /// BLOB with bytes.
    Contents(Vec<u8>),
}

impl TgBlob {
    pub fn new(path: &str) -> TgBlob {
        TgBlob::Path(path.to_string())
    }
}

impl From<Vec<u8>> for TgBlob {
    fn from(value: Vec<u8>) -> Self {
        TgBlob::Contents(value)
    }
}

/// BLOB for [SqlQueryResult](crate::prelude::SqlQueryResult).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TgBlobReference {
    provider: LargeObjectProvider,
    object_id: u64,
}

impl TgBlobReference {
    pub(crate) fn new(provider: LargeObjectProvider, object_id: u64) -> TgBlobReference {
        TgBlobReference {
            provider,
            object_id,
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
}
