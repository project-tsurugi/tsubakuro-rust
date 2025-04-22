use crate::jogasaki::proto::sql::common::LargeObjectProvider;

use super::large_object::TgLargeObjectReference;

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
    object_id: u64,
}

impl TgClobReference {
    pub(crate) fn new(provider: LargeObjectProvider, object_id: u64) -> TgClobReference {
        TgClobReference {
            provider,
            object_id,
        }
    }
}

impl TgLargeObjectReference for TgClobReference {
    fn provider(&self) -> LargeObjectProvider {
        self.provider
    }

    fn object_id(&self) -> u64 {
        self.object_id
    }
}
