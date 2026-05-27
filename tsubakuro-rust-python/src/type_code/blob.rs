use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf, TgBlob};

use crate::{error::to_pyerr, type_code::ParameterContext};

/// BLOB type.
///
/// Examples:
///     ```python
///     value = cursor.upload_blob(b"0x01\x02\x03")
///     value = cursor.upload_blob(None)
///     ```
///
/// since 0.10.0
#[gen_stub_pyclass]
#[pyclass(module = "tsurugi_dbapi.type_code")]
#[derive(Debug)]
pub struct Blob {
    inner: Option<InnerBlob>,
}

#[derive(Debug)]
pub(crate) enum InnerBlob {
    Value(Vec<u8>),
    Blob(TgBlob),
}

#[gen_stub_pymethods]
#[pymethods]
impl Blob {
    /// Create a new `Blob`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<Vec<u8>>) -> Self {
        Blob {
            inner: value.map(InnerBlob::Value),
        }
    }

    pub fn __repr__(&self) -> String {
        match &self.inner {
            Some(InnerBlob::Value(v)) => format!("Blob({:?})", v),
            Some(InnerBlob::Blob(blob)) => format!("Blob({:?})", blob),
            None => "Blob(None)".to_string(),
        }
    }
}

impl Blob {
    pub(crate) fn none() -> Self {
        Blob { inner: None }
    }

    pub(crate) fn from_blob(blob: TgBlob) -> Self {
        Blob {
            inner: Some(InnerBlob::Blob(blob)),
        }
    }

    pub(crate) fn create_parameter(
        context: &ParameterContext,
        name: &str,
        value: &Bound<PyAny>,
    ) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Blob>>() {
            match &v.inner {
                Some(InnerBlob::Value(bytes)) => {
                    let blob = Self::upload_blob(context, bytes)?;
                    Ok(SqlParameter::of(name, Some(blob)))
                }
                Some(InnerBlob::Blob(blob)) => Ok(SqlParameter::of(name, Some(blob.clone()))),
                None => Ok(SqlParameter::of(name, None as Option<TgBlob>)),
            }
        } else {
            let v: Option<Vec<u8>> = value.extract()?;
            if let Some(bytes) = v {
                let blob = Self::upload_blob(context, &bytes)?;
                Ok(SqlParameter::of(name, Some(blob)))
            } else {
                Ok(SqlParameter::of(name, None as Option<TgBlob>))
            }
        }
    }

    fn upload_blob(context: &ParameterContext, value: &[u8]) -> PyResult<TgBlob> {
        let runtime = context.runtime();
        let sql_client = context.sql_client();

        let blob = runtime
            .block_on(sql_client.upload_blob(value))
            .map_err(to_pyerr)?;
        Ok(blob)
    }
}
