use std::path::{Path, PathBuf};

use crate::jogasaki::proto::sql::common::LargeObjectProvider;

pub(crate) trait TgLargeObjectReference: Send + Sync {
    fn provider(&self) -> LargeObjectProvider;
    fn object_id(&self) -> u64;
    fn reference_tag(&self) -> u64;
}

/// Represents large object cache.
///
/// since 0.5.0
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TgLargeObjectCache {
    path: Option<PathBuf>,
}

impl TgLargeObjectCache {
    pub(crate) fn new(path: Option<PathBuf>) -> Self {
        TgLargeObjectCache { path }
    }

    /// Returns the path of the file that represents the large object, only if it exists.
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}
