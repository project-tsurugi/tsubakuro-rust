use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf, TgClob};

use crate::{error::to_pyerr, type_code::ParameterContext};

/// CLOB type.
///
/// Examples:
///     ```python
///     value = cursor.upload_clob("example text")
///     value = cursor.upload_clob(None)
///     ```
///
/// since 0.10.0
#[gen_stub_pyclass]
#[pyclass(module = "tsurugi_dbapi.type_code")]
#[derive(Debug)]
pub struct Clob {
    inner: Option<InnerClob>,
}

#[derive(Debug)]
pub(crate) enum InnerClob {
    Value(String),
    Clob(TgClob),
}

#[gen_stub_pymethods]
#[pymethods]
impl Clob {
    /// Create a new `Clob`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<String>) -> Self {
        Clob {
            inner: value.map(InnerClob::Value),
        }
    }

    pub fn __repr__(&self) -> String {
        match &self.inner {
            Some(InnerClob::Value(v)) => format!("Clob({:?})", v),
            Some(InnerClob::Clob(clob)) => format!("Clob({:?})", clob),
            None => "Clob(None)".to_string(),
        }
    }
}

impl Clob {
    pub(crate) fn none() -> Self {
        Clob { inner: None }
    }

    pub(crate) fn from_clob(clob: TgClob) -> Self {
        Clob {
            inner: Some(InnerClob::Clob(clob)),
        }
    }

    pub(crate) fn create_parameter(
        context: &ParameterContext,
        name: &str,
        value: &Bound<PyAny>,
    ) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Clob>>() {
            match &v.inner {
                Some(InnerClob::Value(value)) => {
                    let clob = Self::upload_clob(context, value)?;
                    Ok(SqlParameter::of(name, Some(clob)))
                }
                Some(InnerClob::Clob(clob)) => Ok(SqlParameter::of(name, Some(clob.clone()))),
                None => Ok(SqlParameter::of(name, None as Option<TgClob>)),
            }
        } else {
            let v: Option<String> = value.extract()?;
            if let Some(value) = v {
                let clob = Self::upload_clob(context, &value)?;
                Ok(SqlParameter::of(name, Some(clob)))
            } else {
                Ok(SqlParameter::of(name, None as Option<TgClob>))
            }
        }
    }

    fn upload_clob(context: &ParameterContext, value: &str) -> PyResult<TgClob> {
        let runtime = context.runtime();
        let sql_client = context.sql_client();

        let clob = runtime
            .block_on(sql_client.upload_clob(value))
            .map_err(to_pyerr)?;
        Ok(clob)
    }
}
