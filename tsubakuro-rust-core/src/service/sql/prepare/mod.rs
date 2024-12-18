use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use log::debug;

use crate::{
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::response::Response as SqlResponseType,
    prelude::{convert_sql_response, sql_result_only_success_processor, Session},
    service::ServiceClient,
    sql_service_error,
};

use super::{SqlClient, WireResponse};

pub mod parameter;
pub mod placeholder;

#[derive(Debug)]
pub struct SqlPreparedStatement {
    session: Arc<Session>,
    prepare_handle: u64,
    has_result_records: bool,
    close_timeout: Duration,
    closed: AtomicBool,
}

impl SqlPreparedStatement {
    fn new(
        session: Arc<Session>,
        prepare_handle: u64,
        has_result_records: bool,
        close_timeout: Duration,
    ) -> SqlPreparedStatement {
        SqlPreparedStatement {
            session,
            prepare_handle,
            has_result_records,
            close_timeout,
            closed: AtomicBool::new(false),
        }
    }

    pub(crate) fn prepare_handle(&self) -> u64 {
        self.prepare_handle
    }

    pub fn has_result_records(&self) -> bool {
        self.has_result_records
    }

    pub fn set_close_timeout(&mut self, timeout: Duration) {
        self.close_timeout = timeout;
    }

    pub fn close_timeout(&self) -> Duration {
        self.close_timeout
    }

    pub async fn close(&self) -> Result<(), TgError> {
        self.close_for(self.close_timeout).await
    }

    pub async fn close_for(&self, timeout: Duration) -> Result<(), TgError> {
        if let Ok(_) = self.closed.compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
        ) {
            let sql_client = SqlClient::new(self.session.clone());
            sql_client
                .dispose_prepare(self.prepare_handle, self.has_result_records, timeout)
                .await?;
        }
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl Drop for SqlPreparedStatement {
    fn drop(&mut self) {
        if self.is_closed() {
            return;
        }

        std::thread::scope(|scope| {
            scope.spawn(move || {
                let runtime = {
                    match tokio::runtime::Runtime::new() {
                        Ok(runtime) => runtime,
                        Err(e) => {
                            debug!("SqlPreparedStatement.drop() error. {}", e);
                            return;
                        }
                    }
                };
                runtime.block_on(async {
                    if let Err(e) = self.close().await {
                        debug!("SqlPreparedStatement.drop() error. {}", e);
                    }
                })
            });
        });
    }
}

pub(crate) fn prepare_processor(
    session: Arc<Session>,
    response: WireResponse,
    close_timeout: Duration,
) -> Result<SqlPreparedStatement, TgError> {
    const FUNCTION_NAME: &str = "prepare_processor()";

    let sql_response = convert_sql_response(FUNCTION_NAME, &response)?;
    let message = sql_response.ok_or(invalid_response_error!(
        FUNCTION_NAME,
        format!("response {:?} is not ResponseSessionPayload", response),
    ))?;
    match message.response {
        Some(SqlResponseType::Prepare(prepare)) => match prepare.result {
            Some(
                crate::jogasaki::proto::sql::response::prepare::Result::PreparedStatementHandle(ps),
            ) => Ok(SqlPreparedStatement::new(
                session,
                ps.handle,
                ps.has_result_records,
                close_timeout,
            )),
            Some(crate::jogasaki::proto::sql::response::prepare::Result::Error(error)) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
            _ => Err(invalid_response_error!(
                FUNCTION_NAME,
                format!("response.prepare {:?} result is None", prepare),
            )),
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not Prepare", message.response),
        )),
    }
}

pub(crate) fn prepare_dispose_processor(response: WireResponse) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "prepare_dispose_processor()";

    sql_result_only_success_processor(FUNCTION_NAME, response)
}
