use crate::{error::TgError, io_error, service::sql::r#type::large_object::TgLargeObjectReference};

pub(crate) mod downloader;
pub(crate) mod lob_client;
pub(crate) mod lob_transfer_info;
pub(crate) mod privileged;
pub(crate) mod relay;
pub(crate) mod uploader;

fn storage_id(lob: &dyn TgLargeObjectReference) -> Result<u64, TgError> {
    use crate::jogasaki::proto::sql::common::LargeObjectProvider::*;
    match lob.provider() {
        Datastore => Ok(1), // LIMESTONE_BLOB_STORE
        v => Err(io_error!(format!(
            "Unsupported LargeObjectProvider: {:?}",
            v
        ))),
    }
}
