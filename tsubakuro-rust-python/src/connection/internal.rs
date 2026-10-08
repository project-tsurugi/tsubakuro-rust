use log::{debug, trace};
use pyo3::{prelude::*, types::*};
use std::sync::Arc;
use tsubakuro_rust_core::prelude::{name::TName, Session, SqlClient, TgError};

use crate::{
    commit_option::CommitOption,
    config::Config,
    connection::{inner_connection::InnerConnection, Connection},
    error::{to_pyerr, ProgrammingError},
    table_metadata::TableMetadata,
};

impl Connection {
    pub(crate) fn create_config(
        args: &Bound<PyTuple>,
        kwargs: Option<Bound<PyDict>>,
    ) -> PyResult<Config> {
        let config = Config::new(args, kwargs)?;
        Ok(config)
    }

    pub(crate) fn connect(config: Config) -> PyResult<Connection> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let session = runtime.block_on(Self::session_connect(&config))?;
        let sql_client: SqlClient = session.make_client();

        let connection = InnerConnection::new(config, runtime, session, sql_client);

        Ok(Connection::new(Arc::new(connection)))
    }

    async fn session_connect(config: &Config) -> PyResult<Arc<Session>> {
        let connection_option = config.connection_option()?;
        let timeout = config.connect_timeout();

        let session = Session::connect_for(&connection_option, timeout)
            .await
            .map_err(to_pyerr)?;
        Ok(session)
    }

    pub(super) fn list_tables_internal(&self) -> PyResult<Vec<String>> {
        const FUNCTION_NAME: &str = "Connection.list_tables()";
        trace!("{FUNCTION_NAME} start");

        let connection = &self.inner;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = connection.default_timeout();

        match runtime.block_on(sql_client.list_tables_for(timeout)) {
            Ok(table_list) => {
                let table_names = table_list
                    .table_names()
                    .iter()
                    .map(TName::to_string)
                    .collect();
                trace!("{FUNCTION_NAME} end");
                Ok(table_names)
            }
            Err(e) => {
                debug!("{FUNCTION_NAME} error: {:?}", e);
                Err(to_pyerr(e))
            }
        }
    }

    pub(super) fn get_table_metadata_internal(&self, table_name: &str) -> PyResult<TableMetadata> {
        const FUNCTION_NAME: &str = "Connection.get_table_metadata()";
        trace!("{FUNCTION_NAME} start. table_name={}", table_name);

        let result = self.get_table_metadata_main(table_name).map_err(to_pyerr);

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        };
        result
    }

    pub(super) fn find_table_metadata_internal(
        &self,
        table_name: &str,
    ) -> PyResult<Option<TableMetadata>> {
        const FUNCTION_NAME: &str = "Connection.find_table_metadata()";
        trace!("{FUNCTION_NAME} start. table_name={}", table_name);

        match self.get_table_metadata_main(table_name) {
            Ok(metadata) => {
                trace!("{FUNCTION_NAME} end");
                Ok(Some(metadata))
            }
            Err(e) => {
                let code = e.diagnostic_code();
                if let Some(code) = code {
                    if code.name() == "TARGET_NOT_FOUND_EXCEPTION" {
                        trace!("{FUNCTION_NAME} end: table not found");
                        return Ok(None);
                    }
                }

                debug!("{FUNCTION_NAME} error: {:?}", e);
                Err(to_pyerr(e))
            }
        }
    }

    fn get_table_metadata_main(&self, table_name: &str) -> Result<TableMetadata, TgError> {
        let connection = &self.inner;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = connection.default_timeout();

        let table_metadata =
            runtime.block_on(sql_client.get_table_metadata_for(table_name, timeout))?;

        Ok(TableMetadata::new(table_metadata))
    }

    pub(super) fn commit_internal(&mut self, option: Option<CommitOption>) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Connection.commit()";
        trace!("{FUNCTION_NAME} start. option={:?}", option);

        let timeout = option.as_ref().and_then(CommitOption::commit_timeout);
        let connection = &self.inner;
        let result = if let Some(option) = option {
            let option = option.to_core_commit_option();
            connection.commit(Some(option), timeout)
        } else {
            connection.commit(None, timeout)
        };

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        };
        result
    }

    pub(super) fn rollback_internal(&mut self) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Connection.rollback()";
        trace!("{FUNCTION_NAME} start");

        let connection = &self.inner;
        let result = connection.rollback();

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        };
        result
    }

    pub(super) fn close_internal(&mut self) -> PyResult<()> {
        let connection = &self.inner;
        connection.close()
    }

    pub(super) fn check_closed(&self, function_name: &str) -> PyResult<()> {
        if self.closed() {
            trace!("{}: Connection is already closed", function_name);
            return Err(ProgrammingError::new_err("Connection is already closed"));
        }
        Ok(())
    }
}
