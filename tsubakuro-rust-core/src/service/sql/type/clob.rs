use crate::jogasaki::proto::sql::common::LargeObjectProvider;

/// CLOB.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TgClob {
    /// CLOB with path.
    Path(String),
    /// CLOB with bytes.
    Contents(Vec<u8>),
}

impl TgClob {
    /// Creates a new instance.
    pub fn new(path: &str) -> TgClob {
        TgClob::Path(path.to_string())
    }
}

impl From<&str> for TgClob {
    fn from(value: &str) -> Self {
        TgClob::Contents(value.as_bytes().to_vec())
    }
}

impl From<String> for TgClob {
    fn from(value: String) -> Self {
        TgClob::Contents(value.into_bytes())
    }
}

/// CLOB for [SqlQueryResult](crate::prelude::SqlQueryResult).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TgClobReference {
    provider: LargeObjectProvider,
    object_id: i64,
}

impl TgClobReference {
    pub(crate) fn new(provider: LargeObjectProvider, object_id: i64) -> TgClobReference {
        TgClobReference {
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
