use std::sync::OnceLock;

mod ctype;
pub mod dbc;
pub mod env;
pub mod handle;
mod logger;
pub mod setup;
pub mod stmt;
mod util;

const ODBC_DRIVER_NAME: &str = "Tsurugi ODBC Driver";
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const TSURUGI_VERSION: &str = "1.5.0";

#[cfg(windows)]
const ODBC_DRIVER_FILE_NAME: &str = "tsubakuro_rust_odbc.dll";

#[cfg(unix)]
const ODBC_DRIVER_FILE_NAME: &str = "libtsubakuro_rust_odbc.so";

static ODBC_DRIVER_VERSION: OnceLock<String> = OnceLock::new();

fn odbc_driver_version() -> &'static String {
    ODBC_DRIVER_VERSION.get_or_init(|| {
        let ss: Vec<i32> = CRATE_VERSION
            .split('.')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        format!("{:02}.{:02}.{:04}", ss[0], ss[1], ss[2])
    })
}
