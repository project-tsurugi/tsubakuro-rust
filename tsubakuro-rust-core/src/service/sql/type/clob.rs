use crate::{
    jogasaki::proto::sql::common::LargeObjectProvider, service::lob::lob_client::RemoteLob,
};

use super::large_object::TgLargeObjectReference;

/// CLOB.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TgClob {
    /// CLOB with client path.
    Path(String),
    /// CLOB with bytes.
    Contents(Vec<u8>),
    /// CLOB with uploaded lob.
    /// since 0.10.0
    #[allow(private_interfaces)]
    RemoteLob(RemoteLob),
}

impl TgClob {
    /// Creates a new instance.
    #[deprecated(since = "0.10.0", note = "Use SqlClient::upload_clob_file instead")]
    pub fn new(path: &str) -> TgClob {
        TgClob::Path(path.to_string())
    }
}

impl From<&str> for TgClob {
    // #[deprecated(since = "0.10.0", note = "Use SqlClient::upload_clob instead")]
    fn from(value: &str) -> Self {
        TgClob::Contents(value.as_bytes().to_vec())
    }
}

impl From<String> for TgClob {
    // #[deprecated(since = "0.10.0", note = "Use SqlClient::upload_clob instead")]
    fn from(value: String) -> Self {
        TgClob::Contents(value.into_bytes())
    }
}

/// CLOB for [SqlQueryResult](crate::prelude::SqlQueryResult).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TgClobReference {
    provider: LargeObjectProvider,
    object_id: u64,
    reference_tag: u64,
}

impl TgClobReference {
    pub(crate) fn new(
        provider: LargeObjectProvider,
        object_id: u64,
        reference_tag: u64,
    ) -> TgClobReference {
        TgClobReference {
            provider,
            object_id,
            reference_tag,
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
    fn reference_tag(&self) -> u64 {
        self.reference_tag
    }
}
