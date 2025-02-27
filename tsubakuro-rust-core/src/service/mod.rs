use std::sync::Arc;

use crate::prelude::Session;

pub mod core;
pub(crate) mod endpoint;
pub mod sql;

/// client of service on tsurugidb.
pub trait ServiceClient {
    /// Creates a new instance.
    ///
    /// see [Session::make_client]
    fn new(session: Arc<Session>) -> Self;
}
