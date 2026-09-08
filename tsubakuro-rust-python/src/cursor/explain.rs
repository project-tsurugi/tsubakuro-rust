use std::collections::HashMap;

use log::trace;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass;
use tsubakuro_rust_core::prelude::{
    AtomType, SqlExplainResult, SqlPlaceholder, SqlPreparedStatement,
};

use crate::{
    column::Column,
    cursor::Cursor,
    error::to_pyerr,
    type_code::{to_parameters, to_parameters_only, ParameterContext},
};

/// Explain result.
///
/// Attributes:
///     format_id (str): The content format ID. (read only)
///     format_version (int): The content format version. (read only)
///     contents (str): The explain result contents. (read only)
///     columns (List[Column]): The result set column information, or empty if it does not provided. (read only)
///
/// since 0.11.0
#[gen_stub_pyclass]
#[pyclass(module = "tsurugi_dbapi")]
#[derive(Debug)]
pub struct ExplainResult {
    /// the content format ID.
    #[pyo3(get)]
    format_id: String,
    /// the content format version.
    #[pyo3(get)]
    format_version: u64,
    /// the explain result contents.
    #[pyo3(get)]
    contents: String,
    /// the result set column information, or empty if it does not provided.
    #[pyo3(get)]
    columns: Vec<Column>,
}

impl From<SqlExplainResult> for ExplainResult {
    fn from(value: SqlExplainResult) -> Self {
        let columns = value
            .columns()
            .iter()
            .map(|c| Column::new(c.clone()))
            .collect();
        ExplainResult {
            format_id: value.format_id().to_string(),
            format_version: value.format_version(),
            contents: value.contents().to_string(),
            columns,
        }
    }
}

#[pymethods]
impl ExplainResult {
    pub fn __repr__(&self) -> String {
        format!(
            "ExplainResult(format_id='{}', format_version={}, contents='{}', columns={:?})",
            self.format_id, self.format_version, self.contents, self.columns
        )
    }
}

impl Cursor {
    pub(crate) fn explain_direct(&mut self, sql: &str) -> PyResult<ExplainResult> {
        const FUNCTION_NAME: &str = "explain_direct()";

        let connection = &self.connection;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = connection.default_timeout();

        trace!("{FUNCTION_NAME}: explain start");
        let explain = runtime
            .block_on(sql_client.explain_for(sql, timeout))
            .map_err(to_pyerr)?;
        trace!("{FUNCTION_NAME}: explain end");

        Ok(ExplainResult::from(explain))
    }

    pub(crate) fn explain_with_parameters(
        &mut self,
        sql: &str,
        seq_of_parameters: Bound<PyAny>,
    ) -> PyResult<ExplainResult> {
        const FUNCTION_NAME: &str = "explain_with_parameters()";

        enum PsInfo<'a> {
            First(HashMap<String, AtomType>, Vec<SqlPlaceholder>),
            Ps(&'a mut SqlPreparedStatement),
        }

        let connection = &self.connection;
        let context = ParameterContext::new(
            connection.runtime(),
            connection.sql_client(),
            connection.lob_upload_timeout(),
        );
        let (info, parameters_list) = if let Some((ps, types)) = self.ps_map.get_mut(sql) {
            let parameters_list = to_parameters_only(&context, seq_of_parameters, &types)?;
            (PsInfo::Ps(ps), parameters_list)
        } else {
            let (types, placeholders, parameters_list) =
                to_parameters(&context, seq_of_parameters)?;
            (PsInfo::First(types, placeholders), parameters_list)
        };
        if parameters_list.is_empty() {
            return self.explain_direct(sql);
        }

        let connection = &self.connection;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = connection.default_timeout();

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

        trace!("{FUNCTION_NAME}: explain start");
        let parameters = parameters_list.into_iter().next().unwrap();
        let explain = runtime
            .block_on(sql_client.prepared_explain_for(&ps, parameters, timeout))
            .map_err(to_pyerr)?;
        trace!("{FUNCTION_NAME}: explain end");

        Ok(ExplainResult::from(explain))
    }
}
