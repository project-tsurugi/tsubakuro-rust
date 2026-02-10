use pyo3::{prelude::*, types::PyTuple};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::TableMetadata as SqlTableMetadata;

use crate::column::{columns_description, Column};

/// Table metadata.
#[gen_stub_pyclass]
#[pyclass]
pub struct TableMetadata {
    inner: SqlTableMetadata,
}

impl TableMetadata {
    pub(crate) fn new(table_metadata: SqlTableMetadata) -> Self {
        TableMetadata {
            inner: table_metadata,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl TableMetadata {
    /// Database name.
    #[getter]
    pub fn database_name(&self) -> &str {
        self.inner.database_name()
    }

    /// Schema name.
    #[getter]
    pub fn schema_name(&self) -> &str {
        self.inner.schema_name()
    }

    /// Table name.
    #[getter]
    pub fn table_name(&self) -> &str {
        self.inner.table_name()
    }

    /// Table description.
    #[getter]
    pub fn table_description(&self) -> Option<&String> {
        self.inner.description()
    }

    /// Columns metadata.
    #[getter]
    pub fn columns(&self) -> Vec<Column> {
        self.inner
            .columns()
            .iter()
            .map(|col| Column::new(col.clone()))
            .collect()
    }

    /// Columns description.
    #[getter]
    pub fn description<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyTuple>>> {
        let columns = self.inner.columns();
        columns_description(py, columns)
    }

    /// Primary keys.
    #[getter]
    pub fn primary_keys(&self) -> &Vec<String> {
        self.inner.primary_keys()
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "TableMetadata(database_name='{}', schema_name='{}', table_name='{}')",
            self.database_name(),
            self.schema_name(),
            self.table_name()
        ))
    }
}
