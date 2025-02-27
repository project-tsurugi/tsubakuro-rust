#![allow(clippy::result_large_err)]
pub mod error;
pub mod job;
pub mod prelude;
pub mod service;
pub mod session;
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
