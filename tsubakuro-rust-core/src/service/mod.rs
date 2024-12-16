use std::sync::Arc;

use crate::prelude::Session;

pub mod core;
pub(crate) mod endpoint;
pub mod sql;

pub trait ServiceClient {
    fn new(session: Arc<Session>) -> Self;
}
