//! [tsubakuro-rust-core](https://github.com/project-tsurugi/tsubakuro-rust/tree/master/tsubakuro-rust-core) is the core library to access Tsurugi DB written in Rust.
//!
//! # Overview
//! The procedure for executing SQL in tsubakuro-rust-core is generally as follows.
//!
//! 1. Create a [`Session`](crate::prelude::Session) (to connect to the Tsurugi DB server).
//!    1. Create a [`ConnectionOption`](crate::prelude::ConnectionOption).
//!    2. Set the ConnectionOption to the endpoint url (e.g. `tcp://localhost:12345`), etc.
//!    3. Invoke [`Session::connect()`](crate::prelude::Session::connect).
//! 2. Make a [`SqlClient`](crate::prelude::SqlClient) from Session.
//!    1. Invoke [`Session::make_client()`](crate::prelude::Session::make_client).
//! 3. When using prepared statement, create a [`SqlPreparedStatement`](crate::prelude::SqlPreparedStatement) from SqlClient.
//!    1. Invoke [`SqlClient::prepare()`](crate::prelude::SqlClient::prepare).
//! 4. Start a transaction (create a [`Transaction`](crate::prelude::Transaction) from SqlClient).
//!    1. Create a [`TransactionOption`](crate::prelude::TransactionOption).
//!    2. Set the TransactionOption to [`TransactionType`](crate::prelude::TransactionType), etc.
//!    3. Invoke [`SqlClient::start_transaction()`](crate::prelude::SqlClient::start_transaction).
//! 5. Execute SQL using SqlClient, Transaction (and SqlPreparedStatement).
//!    - [`SqlClient::execute()`](crate::prelude::SqlClient::execute), [`prepared_execute()`](crate::prelude::SqlClient::prepared_execute)
//!    - [`SqlClient::query()`](crate::prelude::SqlClient::query), [`prepared_query()`](crate::prelude::SqlClient::prepared_query)
//! 6. Commit transaction.
//!    1. Create a [`CommitOption`](crate::prelude::CommitOption).
//!    2. Set the CommitOption to [`CommitType`](crate::prelude::CommitType), etc.
//!    3. Invoke [`SqlClient::commit()`](crate::prelude::SqlClient::commit).
//! 7. Close transaction.
//!    1. Invoke [`Transaction::close()`](crate::prelude::Transaction::close).
//! 8. Close prepared statement if created.
//!    1. Invoke [`SqlPreparedStatement::close()`](crate::prelude::SqlPreparedStatement::close).
//! 9. Close session.
//!    1. Invoke [`Session::close()`](crate::prelude::Session::close).
//!
//! # Examples
//!
//! See <https://github.com/project-tsurugi/tsubakuro-rust/tree/master/tsubakuro-rust-core#example>.
//!
#![allow(clippy::result_large_err)]

#[doc(hidden)]
pub mod error;
#[doc(hidden)]
pub mod job;
pub mod prelude;
#[doc(hidden)]
pub mod service;
#[doc(hidden)]
pub mod session;
#[doc(hidden)]
pub mod transaction;
pub(crate) mod util;

#[allow(clippy::enum_variant_names, clippy::module_inception)]
pub(crate) mod jogasaki {
    pub(crate) mod proto {
        pub(crate) mod sql {
            pub(crate) mod common {
                include!(concat!(env!("OUT_DIR"), "/jogasaki.proto.sql.common.rs"));
            }
            pub(crate) mod error {
                include!(concat!(env!("OUT_DIR"), "/jogasaki.proto.sql.error.rs"));
            }
            pub(crate) mod request {
                include!(concat!(env!("OUT_DIR"), "/jogasaki.proto.sql.request.rs"));
            }
            pub(crate) mod response {
                include!(concat!(env!("OUT_DIR"), "/jogasaki.proto.sql.response.rs"));
            }
        }
    }
}

#[allow(clippy::module_inception)]
pub(crate) mod tateyama {
    pub(crate) mod proto {
        pub(crate) mod core {
            pub(crate) mod request {
                include!(concat!(env!("OUT_DIR"), "/tateyama.proto.core.request.rs"));
            }
            pub(crate) mod response {
                include!(concat!(env!("OUT_DIR"), "/tateyama.proto.core.response.rs"));
            }
        }
        pub(crate) mod diagnostics {
            include!(concat!(env!("OUT_DIR"), "/tateyama.proto.diagnostics.rs"));
        }
        pub(crate) mod endpoint {
            pub(crate) mod request {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/tateyama.proto.endpoint.request.rs"
                ));
            }
            pub(crate) mod response {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/tateyama.proto.endpoint.response.rs"
                ));
            }
        }
        pub(crate) mod framework {
            pub(crate) mod common {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/tateyama.proto.framework.common.rs"
                ));
            }
            pub(crate) mod request {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/tateyama.proto.framework.request.rs"
                ));
            }
            pub(crate) mod response {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/tateyama.proto.framework.response.rs"
                ));
            }
        }
    }
}
