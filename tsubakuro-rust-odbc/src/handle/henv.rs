use std::sync::{Arc, Mutex};

use log::debug;

use crate::{
    ctype::SqlReturn,
    handle::{
        diag::{TsurugiOdbcDiagCollection, TsurugiOdbcError},
        end_tran::CompletionType,
        hdbc::TsurugiOdbcDbc,
    },
};

pub type HEnv = *const TsurugiOdbcEnv;

const SQL_OV_ODBC3: u32 = 3; // ODBC 3.0

#[derive(Debug)]
pub struct TsurugiOdbcEnv {
    dbcs: Mutex<Vec<Arc<TsurugiOdbcDbc>>>,
    diags: Arc<TsurugiOdbcDiagCollection>,
}

impl TsurugiOdbcEnv {
    fn new() -> TsurugiOdbcEnv {
        TsurugiOdbcEnv {
            dbcs: Mutex::new(Vec::new()),
            diags: Arc::new(TsurugiOdbcDiagCollection::new()),
        }
    }

    pub(crate) fn add_dbc(&self, dbc: Arc<TsurugiOdbcDbc>) {
        let mut dbcs = self.dbcs.lock().unwrap();
        dbcs.push(dbc);
    }

    pub(crate) fn remove_dbc(&self, dbc: &TsurugiOdbcDbc) {
        let id = dbc.id();

        let mut dbcs = self.dbcs.lock().unwrap();
        dbcs.retain(|c| c.id() != id);
    }
}

pub(crate) fn alloc_handle_env() -> Result<HEnv, SqlReturn> {
    let env = Arc::new(TsurugiOdbcEnv::new());
    let henv = Arc::into_raw(env);
    debug!(
        "alloc_handle_env(): created new TsurugiOdbcEnv at {:?}",
        henv
    );
    Ok(henv)
}

impl TsurugiOdbcEnv {
    pub(crate) fn from(henv: HEnv) -> Arc<TsurugiOdbcEnv> {
        let env = unsafe { Arc::from_raw(henv) };
        let ret = env.clone();
        let _ = Arc::into_raw(env);
        ret
    }
}

#[macro_export]
macro_rules! check_env {
    ($henv:ident) => {{
        if $henv.is_null() {
            log::debug!("{FUNCTION_NAME} error. {} is null", stringify!($henv));
            let rc = $crate::ctype::SqlReturn::SQL_INVALID_HANDLE;
            log::trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return rc;
        }
        let env = $crate::handle::henv::TsurugiOdbcEnv::from($henv);
        env.clear_diag();
        env
    }};
}

impl TsurugiOdbcEnv {
    pub(crate) fn set_odbc_version(&self, value: u32) -> SqlReturn {
        if value == SQL_OV_ODBC3 {
            debug!("set_odbc_version(): OdbcVersion={}", value);
            SqlReturn::SQL_SUCCESS
        } else {
            debug!("set_odbc_version(): unsupported OdbcVersion {}", value);
            self.add_diag(
                TsurugiOdbcError::InvalidAttribute,
                format!("unsupported OdbcVersion {}", value),
            );
            SqlReturn::SQL_ERROR
        }
    }

    pub(crate) fn odbc_version(&self) -> u32 {
        let value = SQL_OV_ODBC3;
        debug!("odbc_version(): returning {}", value);
        value
    }
}

pub(crate) fn end_tran_env(henv: HEnv, completion_type: CompletionType) -> SqlReturn {
    const FUNCTION_NAME: &str = "end_tran_env()";

    let env = check_env!(henv);
    let dbcs = env.dbcs.lock().unwrap();

    let mut rc = SqlReturn::SQL_SUCCESS;
    for dbc in dbcs.iter() {
        let rc1 = dbc.end_tran_if_exists_transaction(completion_type, &env.diags);
        rc = rc.or(rc1);
    }

    rc
}

impl TsurugiOdbcEnv {
    pub(crate) fn clear_diag(&self) {
        self.diags.clear();
    }

    pub(crate) fn add_diag(&self, error: TsurugiOdbcError, message: impl Into<String>) {
        self.diags.add_diag(error, message);
    }

    pub(crate) fn get_diag_collection(&self) -> Arc<TsurugiOdbcDiagCollection> {
        self.diags.clone()
    }
}

pub(crate) fn free_handle_env(henv: HEnv) -> SqlReturn {
    debug!("free_handle_env(): henv={:?}", henv);
    unsafe {
        let _ = Arc::from_raw(henv);
    }
    SqlReturn::SQL_SUCCESS
}
