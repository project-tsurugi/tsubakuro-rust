use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf, TgDecimal};

/// DECIMAL type.
///
/// Attributes:
///     value (Optional[decimal.Decimal]): decimal value. (read only)
///
/// Examples:
///     ```python
///     import tsurugi_dbapi as tsurugi
///     import decimal
///
///     value = tsurugi.type_code.Decimal(decimal.Decimal("123.45"))
///     value = tsurugi.type_code.Decimal(None)
///     ```
#[gen_stub_pyclass]
#[pyclass(module = "tsurugi_dbapi.type_code")]
#[derive(Debug)]
pub struct Decimal {
    value: Option<TgDecimal>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Decimal {
    /// Create a new `Decimal`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<rust_decimal::Decimal>) -> Self {
        let value = value.map(TgDecimal::from);
        Decimal { value }
    }

    /// Create a `Decimal` from unscaled value and exponent.
    ///
    /// Args:
    ///     unscaled_value (bytes): unscaled value as big-endian byte array
    ///     exponent (int): exponent
    ///
    /// Returns:
    ///     Decimal: created `Decimal` object
    #[classmethod]
    pub fn raw(_cls: &Bound<PyType>, unscaled_value: Vec<u8>, exponent: i32) -> PyResult<Self> {
        let value = TgDecimal::new(unscaled_value, exponent);
        Ok(Decimal { value: Some(value) })
    }

    /// Value.
    #[getter]
    pub fn value(&self) -> Option<rust_decimal::Decimal> {
        self.value.as_ref().map(|v| v.into())
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value() {
            format!("Decimal({})", v)
        } else {
            "Decimal(None)".to_string()
        }
    }
}

impl Decimal {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Decimal>>() {
            Ok(SqlParameter::of(name, v.value.clone()))
        } else {
            let v: Option<rust_decimal::Decimal> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
