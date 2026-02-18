use pyo3::{prelude::*, types::PyTuple};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::SqlColumn;

use crate::type_code::atom_type_to_type_code;

/// Column metadata.
///
/// Attributes:
///     name (str): Column name. (read only)
///     description (Optional[str]): Column description. (read only)
///     type_code (str): Type code. (read only)
///     atom_type_code (int): AtomType code. -1 if unknown. (read only)
///     sql_type (str): SQL type. (read only)
///     sql_type_name (Optional[str]): SQL type name. (read only)
///     length (Optional[int]): Length for string types. (read only)
///     precision (Optional[int]): Precision for numeric types. (read only)
///     scale (Optional[int]): Scale for numeric types. (read only)
///     nullable (Optional[bool]): Nullable flag. (read only)
#[gen_stub_pyclass]
#[pyclass]
pub struct Column {
    inner: SqlColumn,
}

impl Column {
    pub(crate) fn new(column: SqlColumn) -> Self {
        Column { inner: column }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Column {
    /// Column name.
    #[getter]
    pub fn name(&self) -> &str {
        self.inner.name()
    }

    /// Column description.
    #[getter]
    pub fn description(&self) -> Option<&String> {
        self.inner.description()
    }

    /// type_code.
    #[getter]
    pub fn type_code(&self) -> &'static str {
        atom_type_to_type_code(self.inner.atom_type())
    }

    /// AtomType code.
    #[getter]
    pub fn atom_type_code(&self) -> i32 {
        if let Some(atom_type) = self.inner.atom_type() {
            atom_type.into()
        } else {
            -1
        }
    }

    /// SQL type.
    #[getter]
    pub fn sql_type(&self) -> String {
        if let Some(t) = self.inner.sql_type() {
            t
        } else {
            "Unknown".to_string()
        }
    }

    /// SQL type name.
    #[getter]
    pub fn sql_type_name(&self) -> Option<&str> {
        self.inner.sql_type_name()
    }

    /// Length.
    #[getter]
    pub fn length(&self) -> Option<u32> {
        match self.inner.length() {
            Some((length, false)) => Some(length),
            _ => None,
        }
    }

    /// Precision.
    #[getter]
    pub fn precision(&self) -> Option<u32> {
        match self.inner.precision() {
            Some((precision, false)) => Some(precision),
            _ => None,
        }
    }

    /// Scale.
    #[getter]
    pub fn scale(&self) -> Option<u32> {
        match self.inner.scale() {
            Some((scale, false)) => Some(scale),
            _ => None,
        }
    }

    /// Nullable.
    #[getter]
    pub fn nullable(&self) -> Option<bool> {
        self.inner.nullable()
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Column(name='{}', type='{}')",
            self.name(),
            self.sql_type()
        ))
    }
}

pub(crate) fn columns_description<'py>(
    py: Python<'py>,
    columns: &Vec<SqlColumn>,
) -> PyResult<Bound<'py, PyTuple>> {
    let mut vec = Vec::with_capacity(columns.len());
    for column in columns {
        let name = column.name();
        let type_code = atom_type_to_type_code(column.atom_type());
        let length = match column.length() {
            Some((length, false)) => Some(length),
            _ => None,
        };
        let precision = match column.precision() {
            Some((precision, false)) => Some(precision),
            _ => None,
        };
        let scale = match column.scale() {
            Some((scale, false)) => Some(scale),
            _ => None,
        };
        let nullable = column.nullable();

        let tuple = PyTuple::new(
            py,
            vec![
                name.into_pyobject(py)?.into_any(),      // name
                type_code.into_pyobject(py)?.into_any(), // type_code
                py.None().into_pyobject(py)?,            // display_size
                length.into_pyobject(py)?,               // internal_size
                precision.into_pyobject(py)?,            // precision
                scale.into_pyobject(py)?,                // scale
                nullable.into_pyobject(py)?,             // null_ok
            ],
        )?;
        vec.push(tuple);
    }
    Ok(PyTuple::new(py, vec)?)
}
