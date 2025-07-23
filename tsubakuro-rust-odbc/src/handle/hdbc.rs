use std::sync::{
    atomic::{AtomicBool, AtomicU64},
    Arc, Mutex, Weak,
};

use log::{debug, warn};
use tokio::runtime::Runtime;
use tsubakuro_rust_core::prelude::*;

use crate::{
    check_sql_client, check_sql_client_or_err,
    ctype::SqlReturn,
    handle::{
        diag::{TsurugiOdbcDiagCollection, TsurugiOdbcError},
        end_tran::CompletionType,
        henv::{HEnv, TsurugiOdbcEnv},
    },
};

static DBC_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub type HDbc = *const TsurugiOdbcDbc;

pub struct TsurugiOdbcDbc {
    dbc_id: u64,
    env: Weak<TsurugiOdbcEnv>,
    runtime: Arc<Runtime>,
    connection_timeout: AtomicU64, // seconds
    session: Mutex<Option<Arc<Session>>>,
    sql_client: Mutex<Option<Arc<SqlClient>>>,
    auto_commit: AtomicBool,
    transaction: Mutex<Option<Arc<Transaction>>>,
    diags: Arc<TsurugiOdbcDiagCollection>,
}

impl std::fmt::Debug for TsurugiOdbcDbc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsurugiOdbcDbc")
            .field("dbc_id", &self.dbc_id)
            .field("session", &self.session)
            .field("auto_commit", &self.auto_commit)
            .field("diag", &self.diags)
            .finish()
    }
}

impl std::fmt::Display for TsurugiOdbcDbc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TsurugiOdbcDbc{{dbc_id={}}}", self.dbc_id)
    }
}

impl TsurugiOdbcDbc {
    fn new(env: Arc<TsurugiOdbcEnv>) -> TsurugiOdbcDbc {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        TsurugiOdbcDbc {
            dbc_id: DBC_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            env: Arc::downgrade(&env),
            runtime: Arc::new(runtime),
            connection_timeout: AtomicU64::new(0),
            session: Mutex::new(None),
            sql_client: Mutex::new(None),
            auto_commit: AtomicBool::new(true),
            transaction: Mutex::new(None),
            diags: Arc::new(TsurugiOdbcDiagCollection::new()),
        }
    }

    pub(crate) fn id(&self) -> u64 {
        self.dbc_id
    }

    pub(crate) fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }

    pub(crate) fn set_connection_timeout(&self, connection_timeout: u64) -> SqlReturn {
        self.connection_timeout
            .store(connection_timeout, std::sync::atomic::Ordering::SeqCst);
        SqlReturn::SQL_SUCCESS
    }

    pub(crate) fn connection_timeout(&self) -> u64 {
        self.connection_timeout
            .load(std::sync::atomic::Ordering::SeqCst)
    }

    pub(crate) fn set_session(&self, session: Arc<Session>) {
        let sql_client: SqlClient = session.make_client();

        let mut self_session = self.session.lock().unwrap();
        *self_session = Some(session);

        let mut self_sql_client = self.sql_client.lock().unwrap();
        *self_sql_client = Some(Arc::new(sql_client));
    }

    pub(crate) fn session(&self) -> Option<Arc<Session>> {
        let session = self.session.lock().unwrap();
        session.clone()
    }

    pub(crate) fn sql_client(&self) -> Option<Arc<SqlClient>> {
        let sql_client = self.sql_client.lock().unwrap();
        sql_client.clone()
    }

    pub(crate) fn clear_session(&self) -> Option<Arc<Session>> {
        let mut session = self.session.lock().unwrap();
        let session = session.take();

        let mut sql_client = self.sql_client.lock().unwrap();
        *sql_client = None;

        session
    }
}

pub(crate) fn alloc_handle_dbc(henv: HEnv) -> Result<HDbc, SqlReturn> {
    const FUNCTION_NAME: &str = "alloc_handle_dbc()";

    if henv.is_null() {
        debug!("{FUNCTION_NAME} error. henv is null");
        return Err(SqlReturn::SQL_INVALID_HANDLE);
    }
    let env = TsurugiOdbcEnv::from(henv);

    let dbc = Arc::new(TsurugiOdbcDbc::new(env.clone()));
    env.add_dbc(dbc.clone());
    let dbc_string = dbc.to_string();
    let hdbc = Arc::into_raw(dbc);
    debug!("{FUNCTION_NAME}: created {} at {:?}", dbc_string, hdbc);

    Ok(hdbc)
}

impl TsurugiOdbcDbc {
    pub(crate) fn from(hdbc: HDbc) -> Arc<TsurugiOdbcDbc> {
        let dbc = unsafe { Arc::from_raw(hdbc) };
        let ret = dbc.clone();
        let _ = Arc::into_raw(dbc);
        ret
    }
}

#[macro_export]
macro_rules! check_dbc {
    ($hdbc:ident) => {{
        if $hdbc.is_null() {
            log::debug!("{FUNCTION_NAME} error. {} is null", stringify!($hdbc));
            let rc = $crate::ctype::SqlReturn::SQL_INVALID_HANDLE;
            log::trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return rc;
        }
        let dbc = $crate::handle::hdbc::TsurugiOdbcDbc::from($hdbc);
        dbc.clear_diag();
        dbc
    }};
}

impl TsurugiOdbcDbc {
    pub(crate) fn set_auto_commit(&self, auto_commit: bool) -> SqlReturn {
        const FUNCTION_NAME: &str = "set_auto_commit()";

        let transaction = self.transaction.lock().unwrap();
        if transaction.is_some() {
            debug!("{self}.{FUNCTION_NAME} error. transaction exists");
            self.add_diag(
                TsurugiOdbcError::SetAutoCommitError,
                "Cannot change auto_commit because transaction is running",
            );
            return SqlReturn::SQL_ERROR;
        }

        self.auto_commit
            .store(auto_commit, std::sync::atomic::Ordering::SeqCst);

        SqlReturn::SQL_SUCCESS
    }

    pub(crate) fn auto_commit(&self) -> bool {
        self.auto_commit.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn transaction_option(&self) -> TransactionOption {
        TransactionOption::from(TransactionType::Short)
    }

    pub(crate) fn transaction(&self) -> Result<Arc<Transaction>, SqlReturn> {
        const FUNCTION_NAME: &str = "transaction()";

        let mut self_transaction = self.transaction.lock().unwrap();
        if self_transaction.is_none() {
            let sql_client = check_sql_client_or_err!(self);
            let runtime = self.runtime();

            let transaction_option = self.transaction_option();
            let transaction =
                match runtime.block_on(sql_client.start_transaction(&transaction_option)) {
                    Ok(transaction) => {
                        debug!("{self}.{FUNCTION_NAME}: start_transaction() succeeded");
                        transaction
                    }
                    Err(e) => {
                        warn!("{self}.{FUNCTION_NAME}: start_transaction() error. {:?}", e);
                        self.add_diag(
                            TsurugiOdbcError::TransactionStartError,
                            format!("start transaction error. {}", e),
                        );
                        return Err(SqlReturn::SQL_ERROR);
                    }
                };
            *self_transaction = Some(Arc::new(transaction));
        }

        Ok(self_transaction.as_ref().unwrap().clone())
    }

    fn commit_option(&self) -> CommitOption {
        CommitOption::default()
    }

    pub(crate) fn commit(&self, diags: &Arc<TsurugiOdbcDiagCollection>) -> SqlReturn {
        const FUNCTION_NAME: &str = "commit()";

        let mut self_transaction = self.transaction.lock().unwrap();
        if let Some(transaction) = &*self_transaction {
            let rc = self.commit_transaction(FUNCTION_NAME, transaction, diags);
            let rc1 = self.close_transaction(FUNCTION_NAME, transaction, diags);

            *self_transaction = None;
            rc.or(rc1)
        } else {
            debug!("{self}.{FUNCTION_NAME}: Ttransaction not found");
            diags.add_diag(
                TsurugiOdbcError::TransactionNotFound,
                "Transaction not found",
            );
            SqlReturn::SQL_ERROR
        }
    }

    fn commit_transaction(
        &self,
        function_name: &str,
        transaction: &Transaction,
        diags: &Arc<TsurugiOdbcDiagCollection>,
    ) -> SqlReturn {
        let sql_client = check_sql_client!(self);
        let runtime = self.runtime();

        let commit_option = &self.commit_option();
        match runtime.block_on(sql_client.commit(transaction, commit_option)) {
            Ok(_) => {
                debug!("{self}.{function_name}: transaction.commit() succeeded");
                SqlReturn::SQL_SUCCESS
            }
            Err(e) => {
                warn!(
                    "{self}.{function_name}: transaction.commit() error. {:?}",
                    e
                );
                diags.add_diag(
                    TsurugiOdbcError::TransactionCommitError,
                    format!("commit error. {}", e),
                );
                SqlReturn::SQL_ERROR
            }
        }
    }

    pub(crate) fn rollback(
        &self,
        error_if_not_found: bool,
        diags: &Arc<TsurugiOdbcDiagCollection>,
    ) -> SqlReturn {
        const FUNCTION_NAME: &str = "rollback()";

        let mut self_transaction = self.transaction.lock().unwrap();
        if let Some(transaction) = &*self_transaction {
            let rc = self.rollback_transaction(FUNCTION_NAME, transaction, diags);
            let rc1 = self.close_transaction(FUNCTION_NAME, transaction, diags);

            *self_transaction = None;
            rc.or(rc1)
        } else {
            #[allow(clippy::collapsible_else_if)]
            if error_if_not_found {
                debug!("{self}.{FUNCTION_NAME}: Ttransaction not found");
                diags.add_diag(
                    TsurugiOdbcError::TransactionNotFound,
                    "Transaction not found",
                );
                SqlReturn::SQL_ERROR
            } else {
                SqlReturn::SQL_SUCCESS
            }
        }
    }

    fn rollback_transaction(
        &self,
        function_name: &str,
        transaction: &Transaction,
        diags: &Arc<TsurugiOdbcDiagCollection>,
    ) -> SqlReturn {
        let sql_client = check_sql_client!(self);
        let runtime = self.runtime();

        match runtime.block_on(sql_client.rollback(transaction)) {
            Ok(_) => {
                debug!("{self}.{function_name}: transaction.rollback() succeeded");
                SqlReturn::SQL_SUCCESS
            }
            Err(e) => {
                warn!(
                    "{self}.{function_name}: transaction.rollback() error. {:?}",
                    e
                );
                diags.add_diag(
                    TsurugiOdbcError::TransactionRollbackError,
                    format!("rollback error. {}", e),
                );
                SqlReturn::SQL_ERROR
            }
        }
    }

    fn close_transaction(
        &self,
        function_name: &str,
        transaction: &Transaction,
        diags: &Arc<TsurugiOdbcDiagCollection>,
    ) -> SqlReturn {
        let runtime = self.runtime();
        match runtime.block_on(transaction.close()) {
            Ok(_) => {
                debug!("{self}.{function_name}: transaction.close() succeeded");
                SqlReturn::SQL_SUCCESS
            }
            Err(e) => {
                debug!("{self}.{function_name}: transaction.close() error. {:?}", e);
                diags.add_diag(
                    TsurugiOdbcError::TransactionCloseError,
                    format!("transaction close error. {}", e),
                );
                SqlReturn::SQL_SUCCESS_WITH_INFO
            }
        }
    }

    pub(crate) fn end_tran_if_exists_transaction(
        &self,
        completion_type: CompletionType,
        diags: &Arc<TsurugiOdbcDiagCollection>,
    ) -> SqlReturn {
        const FUNCTION_NAME: &str = "end_tran_if_exists_transaction()";

        let mut self_transaction = self.transaction.lock().unwrap();
        if let Some(transaction) = &*self_transaction {
            let rc = match completion_type {
                CompletionType::SQL_COMMIT => {
                    self.commit_transaction(FUNCTION_NAME, transaction, diags)
                }
                CompletionType::SQL_ROLLBACK => {
                    self.rollback_transaction(FUNCTION_NAME, transaction, diags)
                }
            };
            let rc1 = self.close_transaction(FUNCTION_NAME, transaction, diags);

            *self_transaction = None;
            rc.or(rc1)
        } else {
            SqlReturn::SQL_SUCCESS
        }
    }
}

pub(crate) fn end_tran_dbc(hdbc: HDbc, completion_type: CompletionType) -> SqlReturn {
    const FUNCTION_NAME: &str = "end_tran_dbc()";

    let dbc = check_dbc!(hdbc);
    if dbc.auto_commit() {
        debug!("{dbc}.{FUNCTION_NAME} error. It is an auto-commit");
        dbc.add_diag(
            TsurugiOdbcError::EndTranError,
            "Cannot execute end-transaction because it is an auto-commit",
        );
        return SqlReturn::SQL_ERROR;
    }

    match completion_type {
        CompletionType::SQL_COMMIT => dbc.commit(&dbc.diags),
        CompletionType::SQL_ROLLBACK => dbc.rollback(true, &dbc.diags),
    }
}

impl TsurugiOdbcDbc {
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

pub(crate) fn free_handle_dbc(hdbc: HDbc) -> SqlReturn {
    const FUNCTION_NAME: &str = "free_handle_dbc()";
    unsafe {
        let dbc = Arc::from_raw(hdbc);
        debug!("{dbc}.{FUNCTION_NAME}: hdbc={:?}", hdbc);

        let rc = {
            let transaction = dbc.transaction.lock().unwrap();
            if let Some(transaction) = &*transaction {
                dbc.close_transaction(FUNCTION_NAME, transaction, &dbc.diags)
            } else {
                SqlReturn::SQL_SUCCESS
            }
        };

        if let Some(env) = dbc.env.upgrade() {
            env.remove_dbc(&dbc);
        }

        rc
    }
}
