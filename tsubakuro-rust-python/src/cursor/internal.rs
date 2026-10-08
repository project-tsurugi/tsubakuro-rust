use log::{debug, trace};
use pyo3::{prelude::*, types::*};
use std::time::Duration;
use tsubakuro_rust_core::prelude::{AtomType, SqlQueryResult};

use crate::{
    cursor::{
        query_result::{convert_row_to_tuple, next_row1, QueryResultContext, QueryValue},
        Cursor, RowNumber,
    },
    error::{to_pyerr, ProgrammingError},
    type_code::{blob::Blob, clob::Clob},
};

impl Cursor {
    pub(super) fn upload_blob_internal(
        &self,
        value: Vec<u8>,
        timeout: Option<u64>,
    ) -> PyResult<Blob> {
        let connection = &self.connection;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = timeout
            .map(Duration::from_secs)
            .unwrap_or_else(|| connection.lob_upload_timeout());
        let result = runtime
            .block_on(sql_client.upload_blob_for(&value, timeout))
            .map(Blob::from_blob)
            .map_err(to_pyerr);
        result
    }

    pub(super) fn upload_clob_internal(&self, value: &str, timeout: Option<u64>) -> PyResult<Clob> {
        let connection = &self.connection;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = timeout
            .map(Duration::from_secs)
            .unwrap_or_else(|| connection.lob_upload_timeout());
        let result = runtime
            .block_on(sql_client.upload_clob_for(value, timeout))
            .map(Clob::from_clob)
            .map_err(to_pyerr);
        result
    }

    pub(super) fn fetchone_internal(&mut self) -> PyResult<Option<Vec<QueryValue>>> {
        let qr = if let Some(qr) = &mut self.query_result {
            qr
        } else {
            return Err(ProgrammingError::new_err("No query result available"));
        };

        let connection = &self.connection;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let transaction = connection.find_transaction();
        let context =
            QueryResultContext::new(sql_client, transaction, connection.lob_download_timeout());
        runtime.block_on(next_row1(
            &context,
            qr,
            &self.query_types,
            &mut self.row_number,
        ))
    }

    pub(super) fn fetchmany_internal(
        &mut self,
        size: Option<usize>,
    ) -> PyResult<Vec<Vec<QueryValue>>> {
        let qr = if let Some(qr) = &mut self.query_result {
            qr
        } else {
            return Err(ProgrammingError::new_err("No query result available"));
        };

        let size = size.unwrap_or(self.arraysize);

        let connection = &self.connection;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let transaction = connection.find_transaction();
        let context =
            QueryResultContext::new(sql_client, transaction, connection.lob_download_timeout());
        runtime.block_on(Self::next_rows(
            &context,
            qr,
            &self.query_types,
            &mut self.row_number,
            size,
        ))
    }

    async fn next_rows(
        context: &QueryResultContext<'_>,
        qr: &mut SqlQueryResult,
        types: &Vec<AtomType>,
        row_number: &mut Option<RowNumber>,
        size: usize,
    ) -> PyResult<Vec<Vec<QueryValue>>> {
        let mut rows = Vec::with_capacity(size);
        for _ in 0..size {
            if let Some(row) = next_row1(context, qr, types, row_number).await? {
                rows.push(row);
            } else {
                break;
            }
        }
        Ok(rows)
    }

    pub(super) fn fetchall_internal(&mut self) -> PyResult<Vec<Vec<QueryValue>>> {
        let qr = if let Some(qr) = &mut self.query_result {
            qr
        } else {
            return Err(ProgrammingError::new_err("No query result available"));
        };

        let connection = &self.connection;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let transaction = connection.find_transaction();
        let context =
            QueryResultContext::new(sql_client, transaction, connection.lob_download_timeout());
        runtime.block_on(Self::all_rows(
            &context,
            qr,
            &self.query_types,
            &mut self.row_number,
        ))
    }

    async fn all_rows(
        context: &QueryResultContext<'_>,
        qr: &mut SqlQueryResult,
        types: &Vec<AtomType>,
        row_number: &mut Option<RowNumber>,
    ) -> PyResult<Vec<Vec<QueryValue>>> {
        let mut rows = Vec::new();
        loop {
            if let Some(row) = next_row1(context, qr, types, row_number).await? {
                rows.push(row);
            } else {
                break;
            }
        }
        Ok(rows)
    }

    pub(super) fn convert_rows_to_tuples<'py>(
        &self,
        py: Python<'py>,
        rows: Vec<Vec<QueryValue>>,
    ) -> PyResult<Vec<Bound<'py, PyTuple>>> {
        let mut tuples = Vec::with_capacity(rows.len());
        for row in rows {
            let tuple = convert_row_to_tuple(py, row)?;
            tuples.push(tuple);
        }
        Ok(tuples)
    }

    pub(super) fn clear_internal(&mut self) -> PyResult<()> {
        let err = if !self.ps_map.is_empty() || self.query_result.is_some() {
            let connection = &self.connection;
            let runtime = connection.runtime();
            runtime.block_on(async {
                let mut err = None;

                if let Some(qr) = self.query_result.as_mut() {
                    if let Err(e) = qr.close().await {
                        debug!("Cursor query_result close error: {:?}", e);
                        if connection.has_transaction() {
                            err = Some(e);
                        }
                    }
                }

                for (ps, _) in self.ps_map.values_mut() {
                    if let Err(e) = ps.close().await {
                        debug!("Cursor prepared_statement close error: {:?}", e);
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                err
            })
        } else {
            None
        };

        self.ps_map.clear();
        self.query_result = None;
        self.query_types.clear();
        self.row_number = None;
        self.rowcount = -1;

        if let Some(e) = err {
            return Err(to_pyerr(e));
        }
        Ok(())
    }

    pub(super) fn close_internal(&mut self) -> PyResult<()> {
        self.closed = true;
        self.clear_internal()
    }

    pub(super) fn check_closed(&self, function_name: &str) -> PyResult<()> {
        if self.closed {
            trace!("{}: Cursor is already closed", function_name);
            return Err(ProgrammingError::new_err("Cursor is already closed"));
        }
        Ok(())
    }
}
