use crate::jogasaki::proto::sql::common::LargeObjectProvider;

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

/// BLOB for [SqlQueryResult].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TgBlobReference {
    provider: LargeObjectProvider,
    object_id: i64,
}

impl TgBlobReference {
    pub(crate) fn new(provider: LargeObjectProvider, object_id: i64) -> TgBlobReference {
        TgBlobReference {
            provider,
            object_id,
        }
    }

    pub(crate) fn _provider(&self) -> LargeObjectProvider {
        self.provider
    }

    pub(crate) fn _object_id(&self) -> i64 {
        self.object_id
    }
}
