use std::collections::HashMap;

use log::{debug, trace};
use pyo3::prelude::*;
use tsubakuro_rust_core::prelude::{AtomType, SqlPlaceholder, SqlPreparedStatement, TgError};

use crate::{
    cursor::{Cursor, RowNumber},
    error::{to_pyerr, OperationalError, ProgrammingError},
    type_code::{to_parameters, to_parameters_only, to_placeholders},
};

impl Cursor {
    pub(crate) fn execute_direct(&mut self, sql: &str) -> PyResult<()> {
        const FUNCTION_NAME: &str = "execute_direct()";

        let connection = &self.connection;
        let tx = connection.get_transaction().map_err(to_pyerr)?;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = connection.default_timeout();

        if let Some(qr) = self.query_result.as_mut() {
            trace!("{FUNCTION_NAME}: previous query_result close start");
            if let Err(e) = runtime.block_on(qr.close()) {
                debug!("{FUNCTION_NAME}: previous query_result close error: {}", e);
            }
            trace!("{FUNCTION_NAME}: previous query_result close end");

            self.query_result = None;
            self.query_types.clear();
        }
        self.row_number = None;
        self.rowcount = -1;

        let (ps, _types) = if let Some(ps) = self.ps_map.get_mut(sql) {
            ps
        } else {
            trace!("{FUNCTION_NAME}: prepare statement start");
            let placeholders = Vec::new();
            let ps = runtime
                .block_on(sql_client.prepare_for(&sql, placeholders, timeout))
                .map_err(to_pyerr)?;
            trace!("{FUNCTION_NAME}: prepare statement end");

            self.ps_map.insert(sql.to_string(), (ps, HashMap::new()));
            self.ps_map.get_mut(sql).unwrap()
        };

        let parameters = Vec::new();
        if ps.has_result_records() {
            trace!("{FUNCTION_NAME}: query start");
            let qr = runtime
                .block_on(sql_client.prepared_query_for(&tx, &ps, parameters, timeout))
                .map_err(to_pyerr)?;
            trace!("{FUNCTION_NAME}: query end");

            let metadata = qr
                .get_metadata()
                .ok_or_else(|| OperationalError::new_err("failed to get query metadata"))?;
            self.query_types = metadata
                .columns()
                .iter()
                .map(|c| c.atom_type().unwrap())
                .collect();
            self.query_result = Some(qr);
            self.row_number = Some(RowNumber::new());
        } else {
            trace!("{FUNCTION_NAME}: execute start");
            let er = runtime
                .block_on(sql_client.prepared_execute_for(&tx, &ps, parameters, timeout))
                .map_err(to_pyerr)?;
            trace!("{FUNCTION_NAME}: execute end");

            self.rowcount = er.rows() as isize;
        }

        Ok(())
    }

    pub(crate) fn execute_with_parameters(
        &mut self,
        sql: &str,
        seq_of_parameters: Bound<PyAny>,
    ) -> PyResult<()> {
        const FUNCTION_NAME: &str = "execute_with_parameters()";

        enum PsInfo<'a> {
            First(HashMap<String, AtomType>, Vec<SqlPlaceholder>),
            Ps(&'a mut SqlPreparedStatement),
        }

        let (info, parameters_list) = if let Some((ps, types)) = self.ps_map.get_mut(sql) {
            let parameters_list = to_parameters_only(seq_of_parameters, &types)?;
            (PsInfo::Ps(ps), parameters_list)
        } else {
            let (types, placeholders, parameters_list) = to_parameters(seq_of_parameters)?;
            (PsInfo::First(types, placeholders), parameters_list)
        };
        if parameters_list.is_empty() {
            return Ok(());
        }

        let connection = &self.connection;
        let tx = connection.get_transaction().map_err(to_pyerr)?;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = connection.default_timeout();

        if let Some(qr) = self.query_result.as_mut() {
            trace!("{FUNCTION_NAME}: previous query_result close start");
            if let Err(e) = runtime.block_on(qr.close()) {
                debug!("{FUNCTION_NAME}: previous query_result close error: {}", e);
            }
            trace!("{FUNCTION_NAME}: previous query_result close end");

            self.query_result = None;
            self.query_types.clear();
        }
        self.row_number = None;
        self.rowcount = -1;

        let ps = match info {
            PsInfo::First(types, placeholders) => {
                trace!("{FUNCTION_NAME}: prepare statement start");
                let ps = runtime
                    .block_on(sql_client.prepare_for(&sql, placeholders, timeout))
                    .map_err(to_pyerr)?;
                trace!("{FUNCTION_NAME}: prepare statement end");

                self.ps_map.insert(sql.to_string(), (ps, types));
                let (ps, _) = self.ps_map.get_mut(sql).unwrap();
                ps
            }
            PsInfo::Ps(ps) => ps,
        };

        if ps.has_result_records() {
            if parameters_list.len() != 1 {
                return Err(ProgrammingError::new_err(
                    "executemany with multi query is not supported",
                ));
            }
            let parameters = parameters_list.into_iter().next().unwrap();

            trace!("{FUNCTION_NAME}: query start");
            let qr = runtime
                .block_on(sql_client.prepared_query_for(&tx, &ps, parameters, timeout))
                .map_err(to_pyerr)?;
            trace!("{FUNCTION_NAME}: query end");

            let metadata = qr
                .get_metadata()
                .ok_or_else(|| OperationalError::new_err("failed to get query metadata"))?;
            self.query_types = metadata
                .columns()
                .iter()
                .map(|c| c.atom_type().unwrap())
                .collect();
            self.query_result = Some(qr);
            self.row_number = Some(RowNumber::new());
        } else {
            self.rowcount = if self.executemany_async && parameters_list.len() > 1 {
                trace!("{FUNCTION_NAME}: execute async start");
                let result: Result<isize, TgError> = runtime.block_on(async {
                    let mut job_list = Vec::with_capacity(parameters_list.len());
                    for parameters in parameters_list {
                        let job = sql_client
                            .prepared_execute_async(&tx, &ps, parameters)
                            .await?;
                        job_list.push(job);
                    }

                    let mut count = 0;
                    for mut job in job_list {
                        let er = job.take_for(timeout).await?;
                        count += er.rows() as isize;
                    }
                    Ok(count)
                });
                trace!("{FUNCTION_NAME}: execute async end");

                result.map_err(to_pyerr)?
            } else {
                trace!("{FUNCTION_NAME}: execute start");
                let result: Result<isize, TgError> = runtime.block_on(async {
                    let mut count = 0;
                    for parameters in parameters_list {
                        let er = sql_client
                            .prepared_execute_for(&tx, &ps, parameters, timeout)
                            .await?;
                        count += er.rows() as isize;
                    }
                    Ok(count)
                });
                trace!("{FUNCTION_NAME}: execute end");

                result.map_err(to_pyerr)?
            };
        }

        Ok(())
    }

    pub(crate) fn prepare_placeholders(
        &mut self,
        sql: &str,
        parameters: Bound<PyAny>,
    ) -> PyResult<()> {
        const FUNCTION_NAME: &str = "prepare_placeholders()";

        let connection = &self.connection;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = connection.default_timeout();

        if let Some((ps, _types)) = self.ps_map.get(sql) {
            trace!("{FUNCTION_NAME}: previous prepared_statement close start");
            if let Err(e) = runtime.block_on(ps.close()) {
                debug!(
                    "{FUNCTION_NAME}: previous prepared_statement close error: {:?}",
                    e
                );
            }
            trace!("{FUNCTION_NAME}: previous prepared_statement close end");
            self.ps_map.remove(sql);
        }

        let (types, placeholders) = to_placeholders(parameters)?;

        trace!("{FUNCTION_NAME}: prepare statement start");
        let ps = runtime
            .block_on(sql_client.prepare_for(&sql, placeholders, timeout))
            .map_err(to_pyerr)?;
        trace!("{FUNCTION_NAME}: prepare statement end");

        self.ps_map.insert(sql.to_string(), (ps, types));

        Ok(())
    }
}
