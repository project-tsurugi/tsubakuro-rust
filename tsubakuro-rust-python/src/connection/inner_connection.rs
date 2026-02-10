use log::{debug, trace};
use pyo3::prelude::*;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tsubakuro_rust_core::prelude::{
    CommitOption as CoreCommitOption, Session, ShutdownType as CoreShutdownType, SqlClient,
    TgError, Transaction, TransactionOption as CoreTransactionOption,
};

use crate::{
    commit_option::CommitOption, config::Config, error::to_pyerr, shutdown_option::ShutdownOption,
    transaction_option::TransactionOption,
};

pub(crate) struct InnerConnection {
    config: Arc<Mutex<Config>>,
    runtime: tokio::runtime::Runtime,
    session: Arc<Session>,
    sql_client: SqlClient,
    transaction_option: Arc<Mutex<CoreTransactionOption>>,
    commit_option: Arc<Mutex<CoreCommitOption>>,
    transaction: Mutex<Option<Arc<Transaction>>>,
    closed: AtomicBool,
}

impl InnerConnection {
    pub(super) fn new(
        config: Config,
        runtime: tokio::runtime::Runtime,
        session: Arc<Session>,
        sql_client: SqlClient,
    ) -> Self {
        let transaction_option = config.core_transaction_option();
        let commit_option = config.core_commit_option();
        Self {
            config: Arc::new(Mutex::new(config)),
            runtime,
            session,
            sql_client,
            transaction_option: Arc::new(Mutex::new(transaction_option)),
            commit_option: Arc::new(Mutex::new(commit_option)),
            transaction: Mutex::new(None),
            closed: AtomicBool::new(false),
        }
    }

    pub(crate) fn runtime(&self) -> &tokio::runtime::Runtime {
        &self.runtime
    }

    pub(crate) fn sql_client(&self) -> &SqlClient {
        &self.sql_client
    }

    pub(crate) fn default_timeout(&self) -> Duration {
        let config = self.config.lock().unwrap();
        config.default_timeout()
    }

    pub(crate) fn set_transaction_option(&self, option: TransactionOption) {
        let mut transaction_option = self.transaction_option.lock().unwrap();
        *transaction_option = option.to_core_transaction_option();

        let mut config = self.config.lock().unwrap();
        config.transaction_option = Some(option);
    }

    pub(crate) fn get_transaction(&self) -> Result<Arc<Transaction>, TgError> {
        const FUNCTION_NAME: &str = "get_transaction()";

        let mut transaction = self.transaction.lock().unwrap();
        if transaction.is_none() {
            let runtime = self.runtime();
            let sql_client = self.sql_client();
            let option = self.transaction_option.lock().unwrap();
            let timeout = self.begin_timeout();

            trace!("{FUNCTION_NAME}: create transaction start. {:?}", *option);
            let tx = runtime.block_on(sql_client.start_transaction_for(&option, timeout))?;
            trace!("{FUNCTION_NAME}: create transaction end");

            *transaction = Some(Arc::new(tx));
        }

        Ok(transaction.as_ref().unwrap().clone())
    }

    fn begin_timeout(&self) -> Duration {
        let config = self.config.lock().unwrap();
        if let Some(option) = &config.transaction_option {
            if let Some(timeout) = option.begin_timeout() {
                return timeout;
            }
        }
        config.default_timeout()
    }

    pub(crate) fn set_commit_option(&self, option: CommitOption) {
        let mut commit_option = self.commit_option.lock().unwrap();
        *commit_option = option.to_core_commit_option();

        let mut config = self.config.lock().unwrap();
        config.commit_option = Some(option);
    }

    pub(super) fn commit(
        &self,
        option: Option<CoreCommitOption>,
        timeout: Option<Duration>,
    ) -> PyResult<()> {
        const FUNCTION_NAME: &str = "commit()";

        let mut transaction = self.transaction.lock().unwrap();
        if let Some(tx) = &*transaction {
            let runtime = self.runtime();
            let sql_client = self.sql_client();
            let commit_option = option.unwrap_or(*self.commit_option.lock().unwrap());
            let timeout = self.commit_timeout(timeout);

            trace!("{FUNCTION_NAME}: commit start. {:?}", commit_option);
            runtime
                .block_on(sql_client.commit_for(tx, &commit_option, timeout))
                .map_err(to_pyerr)?;
            trace!("{FUNCTION_NAME}: commit end");

            *transaction = None;
        }

        Ok(())
    }

    fn commit_timeout(&self, timeout: Option<Duration>) -> Duration {
        timeout.unwrap_or_else(|| self.default_timeout())
    }

    pub(super) fn rollback(&self) -> PyResult<()> {
        const FUNCTION_NAME: &str = "rollback()";

        let mut transaction = self.transaction.lock().unwrap();
        if let Some(tx) = &*transaction {
            let runtime = self.runtime();
            let sql_client = self.sql_client();
            let timeout = self.default_timeout();

            trace!("{FUNCTION_NAME}: rollback start");
            runtime
                .block_on(sql_client.rollback_for(tx, timeout))
                .map_err(to_pyerr)?;
            trace!("{FUNCTION_NAME}: rollback end");

            *transaction = None;
        }

        Ok(())
    }

    pub(crate) fn set_shutdown_option(&self, option: ShutdownOption) {
        let mut config = self.config.lock().unwrap();
        config.shutdown_option = Some(option);
    }

    pub(crate) fn close(&self) -> PyResult<()> {
        const FUNCTION_NAME: &str = "InnerConnection.close()";

        if !self
            .closed
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            // already closed
            return Ok(());
        }

        let runtime = self.runtime();
        let session = &self.session;

        let result = runtime
            .block_on(async {
                {
                    let transaction = self.transaction.lock().unwrap();
                    if let Some(tx) = &*transaction {
                        trace!("{FUNCTION_NAME}: transaction close start");
                        if let Err(e) = tx.close().await {
                            debug!("{FUNCTION_NAME}: transaction close error: {:?}", e);
                        }
                        trace!("{FUNCTION_NAME}: transaction close end");
                    }
                }

                let (shutdown_type, timeout) = {
                    let config = self.config.lock().unwrap();
                    if let Some(option) = config.shutdown_option() {
                        let timeout = option
                            .shutdown_timeout()
                            .unwrap_or_else(|| config.default_timeout());
                        (option.core_shutdown_type(), timeout)
                    } else {
                        (CoreShutdownType::Graceful, config.default_timeout())
                    }
                };
                if shutdown_type != CoreShutdownType::NotSet {
                    trace!(
                        "{FUNCTION_NAME}: session shutdown start. shutdown_type={:?}",
                        shutdown_type
                    );
                    if let Err(e) = session.shutdown_for(shutdown_type, timeout).await {
                        debug!("{FUNCTION_NAME}: session shutdown error: {:?}", e);
                    }
                    trace!("{FUNCTION_NAME}: session shutdown end");
                }

                trace!("{FUNCTION_NAME}: session close start");
                let result = session.close().await;
                trace!("{FUNCTION_NAME}: session close end");

                result
            })
            .map_err(to_pyerr);

        let mut transaction = self.transaction.lock().unwrap();
        *transaction = None;
        result
    }
}
