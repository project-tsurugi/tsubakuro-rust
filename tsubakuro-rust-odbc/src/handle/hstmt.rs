use std::{
    cell::RefCell,
    rc::Rc,
    sync::{atomic::AtomicU64, Arc, Mutex},
};

use log::debug;
use tokio::runtime::Runtime;
use tsubakuro_rust_core::prelude::SqlClient;

use crate::{
    ctype::SqlReturn,
    handle::{
        diag::{TsurugiOdbcDiagCollection, TsurugiOdbcError},
        hdbc::{HDbc, TsurugiOdbcDbc},
    },
    stmt::{
        bind_col::TsurugiOdbcBindColumn, bind_parameter::TsurugiOdbcBindParameter,
        prepare::TsurugiOdbcPrepare, TsurugiOdbcStatementProcessor,
    },
};

static STMT_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub type HStmt = *const Mutex<TsurugiOdbcStmt>;

pub struct TsurugiOdbcStmt {
    stmt_id: u64,
    dbc: Arc<TsurugiOdbcDbc>,
    name: String,
    bind_columns: Vec<Option<TsurugiOdbcBindColumn>>,
    parameters: Vec<Option<TsurugiOdbcBindParameter>>,
    prepare: Option<Rc<RefCell<TsurugiOdbcPrepare>>>,
    processor: Option<Rc<RefCell<dyn TsurugiOdbcStatementProcessor>>>,
    auto_commit: bool,
    diags: Arc<TsurugiOdbcDiagCollection>,
}

impl std::fmt::Debug for TsurugiOdbcStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsurugiOdbcStmt")
            .field("stmt_id", &self.stmt_id)
            .field("diag", &self.diags)
            .finish()
    }
}

impl std::fmt::Display for TsurugiOdbcStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TsurugiOdbcStmt{{stmt_id={}, dbc_id={}, name={}}}",
            self.stmt_id,
            self.dbc.id(),
            self.name
        )
    }
}

impl TsurugiOdbcStmt {
    fn new(dbc: Arc<TsurugiOdbcDbc>) -> TsurugiOdbcStmt {
        TsurugiOdbcStmt {
            stmt_id: STMT_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            dbc,
            name: "None".to_string(),
            bind_columns: Vec::new(),
            parameters: Vec::new(),
            prepare: None,
            processor: None,
            auto_commit: false,
            diags: Arc::new(TsurugiOdbcDiagCollection::new()),
        }
    }

    pub(crate) fn dbc(&self) -> &Arc<TsurugiOdbcDbc> {
        &self.dbc
    }

    pub(crate) fn runtime(&self) -> &Arc<Runtime> {
        self.dbc.runtime()
    }

    pub(crate) fn sql_client(&self) -> Option<Arc<SqlClient>> {
        self.dbc.sql_client()
    }

    pub(crate) fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub(crate) fn set_bind_column(&mut self, bind_column: TsurugiOdbcBindColumn) {
        let index = bind_column.column_number() as usize - 1;
        while index >= self.bind_columns.len() {
            self.bind_columns.push(None);
        }

        self.bind_columns[index] = Some(bind_column);
    }

    pub(crate) fn has_bind_columns(&self) -> bool {
        !self.bind_columns.is_empty()
    }

    pub(crate) fn bind_columns(&self) -> &Vec<Option<TsurugiOdbcBindColumn>> {
        &self.bind_columns
    }

    pub(crate) fn set_parameter(&mut self, parameter: TsurugiOdbcBindParameter) {
        let index = parameter.parameter_number() as usize - 1;
        while index >= self.parameters.len() {
            self.parameters.push(None);
        }

        self.parameters[index] = Some(parameter);
    }

    pub(crate) fn parameters(&self) -> &Vec<Option<TsurugiOdbcBindParameter>> {
        &self.parameters
    }

    pub(crate) fn set_prepare(&mut self, prepare: TsurugiOdbcPrepare) {
        self.close_prepare();
        self.prepare = Some(Rc::new(RefCell::new(prepare)));
    }

    pub(crate) fn prepare(&self) -> Option<Rc<RefCell<TsurugiOdbcPrepare>>> {
        self.prepare.clone()
    }

    pub(crate) fn close_prepare(&mut self) -> SqlReturn {
        if let Some(prepare) = self.prepare.take() {
            prepare.borrow_mut().close(self)
        } else {
            SqlReturn::SQL_SUCCESS
        }
    }

    pub(crate) fn set_processor<T: TsurugiOdbcStatementProcessor + 'static>(
        &mut self,
        processor: T,
    ) {
        self.close_processor();
        self.processor = Some(Rc::new(RefCell::new(processor)));
    }

    pub(crate) fn processor(
        &self,
        function_name: &str,
    ) -> Result<Rc<RefCell<dyn TsurugiOdbcStatementProcessor>>, SqlReturn> {
        match &self.processor {
            Some(processor) => Ok(processor.clone()),
            None => {
                debug!("{self}.{function_name} error. processor not found");
                self.add_diag(
                    TsurugiOdbcError::StatementProcessorNotFound,
                    "Statement processor not found",
                );
                Err(SqlReturn::SQL_ERROR)
            }
        }
    }

    fn close_processor(&mut self) -> SqlReturn {
        if let Some(processor) = self.processor.take() {
            processor.borrow_mut().dispose(self)
        } else {
            SqlReturn::SQL_SUCCESS
        }
    }

    pub(crate) fn set_auto_commit_from_dbc(&mut self) {
        self.auto_commit = self.dbc().auto_commit();
    }

    pub(crate) fn commit_if_auto_commit(&self) -> SqlReturn {
        if self.auto_commit {
            self.dbc().commit(&self.diags)
        } else {
            SqlReturn::SQL_SUCCESS
        }
    }

    pub(crate) fn rollback_if_auto_commit(&self) -> SqlReturn {
        if self.auto_commit {
            self.dbc().rollback(false, &self.diags)
        } else {
            SqlReturn::SQL_SUCCESS
        }
    }
}

pub(crate) fn alloc_handle_stmt(hdbc: HDbc) -> Result<HStmt, SqlReturn> {
    const FUNCTION_NAME: &str = "alloc_handle_stmt()";

    if hdbc.is_null() {
        debug!("{FUNCTION_NAME} error. hdbc is null");
        return Err(SqlReturn::SQL_INVALID_HANDLE);
    }
    let dbc = TsurugiOdbcDbc::from(hdbc);

    let stmt = TsurugiOdbcStmt::new(dbc);
    let stmt_string = stmt.to_string();
    let stmt = Arc::new(Mutex::new(stmt));
    let hstmt = Arc::into_raw(stmt);
    debug!("{FUNCTION_NAME}: created {} at {:?}", stmt_string, hstmt);

    Ok(hstmt)
}

impl TsurugiOdbcStmt {
    pub(crate) fn from(hstmt: HStmt) -> Arc<Mutex<TsurugiOdbcStmt>> {
        let stmt = unsafe { Arc::from_raw(hstmt) };
        let ret = stmt.clone();
        let _ = Arc::into_raw(stmt);
        ret
    }
}

#[macro_export]
macro_rules! check_stmt {
    ($hstmt:ident) => {{
        if $hstmt.is_null() {
            log::debug!("{FUNCTION_NAME} error. {} is null", stringify!($hstmt));
            let rc = $crate::ctype::SqlReturn::SQL_INVALID_HANDLE;
            log::trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return rc;
        }
        $crate::handle::hstmt::TsurugiOdbcStmt::from($hstmt)
    }};
}

#[macro_export]
macro_rules! check_sql_client {
    ($stmt:ident) => {
        match $stmt.sql_client() {
            Some(sql_client) => sql_client,
            None => {
                log::debug!("{} not connected", $stmt);
                $stmt.add_diag(TsurugiOdbcError::NotConnected, "not connected");
                return SqlReturn::SQL_ERROR;
            }
        }
    };
}

#[macro_export]
macro_rules! check_sql_client_or_err {
    ($stmt:ident) => {
        match $stmt.sql_client() {
            Some(sql_client) => sql_client,
            None => {
                log::debug!("{} not connected", $stmt);
                $stmt.add_diag(TsurugiOdbcError::NotConnected, "not connected");
                return Err(SqlReturn::SQL_ERROR);
            }
        }
    };
}

impl TsurugiOdbcStmt {
    pub(crate) fn clear_diag(&self) {
        self.diags.clear();
    }

    pub(crate) fn add_diag(&self, error: TsurugiOdbcError, message: impl Into<String>) {
        self.diags.add_diag(error, message);
    }

    pub(crate) fn diag_collection(&self) -> Arc<TsurugiOdbcDiagCollection> {
        self.diags.clone()
    }
}

pub(crate) fn free_handle_stmt(hstmt: HStmt) -> SqlReturn {
    unsafe {
        let stmt = Arc::from_raw(hstmt);
        let mut stmt = stmt.lock().unwrap();
        debug!("{stmt}.free_handle_stmt(): hdbc={:?}", hstmt);

        let rc1 = stmt.close_processor();
        let rc2 = stmt.close_prepare();

        rc1.or(rc2)
    }
}
