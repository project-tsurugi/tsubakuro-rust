use std::collections::HashMap;

use chrono::Timelike;
use pyo3::{exceptions::PyValueError, prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{AtomType, SqlParameter, SqlParameterOf, SqlPlaceholder};

use crate::error::ProgrammingError;

/// BOOLEAN type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Bool {
    #[pyo3(get)]
    value: Option<bool>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Bool {
    /// Create a new `Bool`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<bool>) -> Self {
        Bool { value }
    }

    pub fn __bool__(&self) -> bool {
        self.value.unwrap_or(false)
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value {
            format!("Bool({})", v)
        } else {
            "Bool(None)".to_string()
        }
    }
}

/// INT type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Int32 {
    #[pyo3(get)]
    value: Option<i32>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Int32 {
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<i32>) -> PyResult<Self> {
        Ok(Int32 { value })
    }

    pub fn __int__(&self) -> Option<i32> {
        self.value
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value {
            format!("Int32({})", v)
        } else {
            "Int32(None)".to_string()
        }
    }
}

/// BIGINT type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Int64 {
    #[pyo3(get)]
    value: Option<i64>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Int64 {
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<i64>) -> PyResult<Self> {
        Ok(Int64 { value })
    }

    pub fn __int__(&self) -> Option<i64> {
        self.value
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value {
            format!("Int64({})", v)
        } else {
            "Int64(None)".to_string()
        }
    }
}

/// REAL type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Float32 {
    #[pyo3(get)]
    value: Option<f32>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Float32 {
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<f32>) -> PyResult<Self> {
        Ok(Float32 { value })
    }

    pub fn __float__(&self) -> Option<f32> {
        self.value
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value {
            format!("Float32({})", v)
        } else {
            "Float32(None)".to_string()
        }
    }
}

/// DOUBLE type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Float64 {
    #[pyo3(get)]
    value: Option<f64>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Float64 {
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<f64>) -> PyResult<Self> {
        Ok(Float64 { value })
    }

    pub fn __float__(&self) -> Option<f64> {
        self.value
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value {
            format!("Float64({})", v)
        } else {
            "Float64(None)".to_string()
        }
    }
}

/// DECIMAL type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Decimal {
    #[pyo3(get)]
    value: Option<rust_decimal::Decimal>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Decimal {
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<rust_decimal::Decimal>) -> Self {
        Decimal { value }
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Decimal({})", v)
        } else {
            "Decimal(None)".to_string()
        }
    }
}

/// CHAR, VARCHAR type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Str {
    #[pyo3(get)]
    value: Option<String>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Str {
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<String>) -> Self {
        Str { value }
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Str({})", v)
        } else {
            "Str(None)".to_string()
        }
    }
}

/// BINARY, VARBINARY type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Bytes {
    #[pyo3(get)]
    value: Option<Vec<u8>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Bytes {
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<Vec<u8>>) -> Self {
        Bytes { value }
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Bytes({:?})", v)
        } else {
            "Bytes(None)".to_string()
        }
    }
}

/// DATE type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Date {
    #[pyo3(get)]
    value: Option<chrono::NaiveDate>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Date {
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<chrono::NaiveDate>) -> PyResult<Self> {
        Ok(Date { value })
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Date({})", v)
        } else {
            "Date(None)".to_string()
        }
    }
}

/// TIME type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Time {
    #[pyo3(get)]
    value: Option<chrono::NaiveTime>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Time {
    #[new]
    #[pyo3(signature = (value=None, nanosecond=None))]
    pub fn new(value: Option<chrono::NaiveTime>, nanosecond: Option<u32>) -> PyResult<Self> {
        if let Some(v) = value {
            let v = if let Some(ns) = nanosecond {
                v.with_nanosecond(ns)
                    .ok_or_else(|| PyValueError::new_err("invalid nanosecond value"))?
            } else {
                v
            };
            Ok(Time { value: Some(v) })
        } else {
            Ok(Time { value: None })
        }
    }

    #[getter]
    pub fn nanosecond(&self) -> Option<u32> {
        self.value.map(|v| v.nanosecond())
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Time({})", v)
        } else {
            "Time(None)".to_string()
        }
    }
}

/// TIMESTAMP type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Datetime {
    #[pyo3(get)]
    value: Option<chrono::NaiveDateTime>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Datetime {
    #[new]
    #[pyo3(signature = (value=None, nanosecond=None))]
    pub fn new(value: Option<chrono::NaiveDateTime>, nanosecond: Option<u32>) -> PyResult<Self> {
        if let Some(v) = value {
            let v = if let Some(ns) = nanosecond {
                v.with_nanosecond(ns)
                    .ok_or_else(|| PyValueError::new_err("invalid nanosecond value"))?
            } else {
                v
            };
            Ok(Datetime { value: Some(v) })
        } else {
            Ok(Datetime { value: None })
        }
    }

    #[getter]
    pub fn nanosecond(&self) -> Option<u32> {
        self.value.map(|v| v.nanosecond())
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Datetime({})", v)
        } else {
            "Datetime(None)".to_string()
        }
    }
}

/// TIME WITH TIME ZONE type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct OffsetTime {
    value: Option<(chrono::NaiveTime, chrono::FixedOffset)>,
}

#[gen_stub_pymethods]
#[pymethods]
impl OffsetTime {
    #[new]
    #[pyo3(signature = (value=None, nanosecond=None))]
    pub fn new(value: Option<Bound<PyTime>>, nanosecond: Option<u32>) -> PyResult<Self> {
        if let Some(v) = value {
            let time: chrono::NaiveTime = v.extract()?;
            let time = if let Some(ns) = nanosecond {
                time.with_nanosecond(ns)
                    .ok_or_else(|| PyValueError::new_err("invalid nanosecond value"))?
            } else {
                time
            };
            let tzinfo = v.get_tzinfo();
            let offset: chrono::FixedOffset = if let Some(tzinfo) = tzinfo {
                tzinfo.extract()?
            } else {
                chrono::FixedOffset::east_opt(0).unwrap()
            };
            Ok(OffsetTime {
                value: Some((time, offset)),
            })
        } else {
            Ok(OffsetTime { value: None })
        }
    }

    #[getter]
    pub fn value<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyTime>>> {
        if let Some((time, offset)) = &self.value {
            let hour = time.hour() as u8;
            let minute = time.minute() as u8;
            let second = time.second() as u8;
            let microsecond = time.nanosecond() / 1000;
            let tzinfo = offset.into_pyobject(py)?;
            let time = PyTime::new(py, hour, minute, second, microsecond, Some(&tzinfo))?;
            Ok(Some(time))
        } else {
            Ok(None)
        }
    }

    #[getter]
    pub fn nanosecond(&self) -> Option<u32> {
        self.value.map(|(time, _)| time.nanosecond())
    }

    pub fn __repr__(&self) -> String {
        if let Some((time, offset)) = &self.value {
            format!("OffsetTime({} {})", time, offset)
        } else {
            "OffsetTime(None)".to_string()
        }
    }
}

/// TIMESTAMP WITH TIME ZONE type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct OffsetDatetime {
    #[pyo3(get)]
    value: Option<chrono::DateTime<chrono::FixedOffset>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl OffsetDatetime {
    #[new]
    #[pyo3(signature = (value=None, nanosecond=None))]
    pub fn new(
        value: Option<chrono::DateTime<chrono::FixedOffset>>,
        nanosecond: Option<u32>,
    ) -> PyResult<Self> {
        if let Some(v) = value {
            let v = if let Some(ns) = nanosecond {
                v.with_nanosecond(ns)
                    .ok_or_else(|| PyValueError::new_err("invalid nanosecond value"))?
            } else {
                v
            };
            Ok(OffsetDatetime { value: Some(v) })
        } else {
            Ok(OffsetDatetime { value: None })
        }
    }

    #[getter]
    pub fn nanosecond(&self) -> Option<u32> {
        self.value.map(|v| v.nanosecond())
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("OffsetDatetime({})", v)
        } else {
            "OffsetDatetime(None)".to_string()
        }
    }
}

pub(crate) fn atom_type_to_type_code(atom_type: Option<AtomType>) -> &'static str {
    match atom_type {
        Some(t) => match t {
            AtomType::Boolean => "Bool",
            AtomType::Int4 => "Int32",
            AtomType::Int8 => "Int64",
            AtomType::Float4 => "Float32",
            AtomType::Float8 => "Float64",
            AtomType::Decimal => "Decimal",
            AtomType::Character => "Str",
            AtomType::Octet => "Bytes",
            AtomType::Date => "Date",
            AtomType::TimeOfDay => "Time",
            AtomType::TimePoint => "Datetime",
            AtomType::TimeOfDayWithTimeZone => "OffsetTime",
            AtomType::TimePointWithTimeZone => "OffsetDatetime",
            _ => "Unknown",
        },
        None => "Unknown",
    }
}

pub(crate) fn to_parameters(
    seq_of_parameters: Bound<PyAny>,
) -> PyResult<(
    HashMap<String, AtomType>,
    Vec<SqlPlaceholder>,
    Vec<Vec<SqlParameter>>,
)> {
    let mut types = HashMap::new();
    let mut placeholders = Vec::new();
    let mut parameters_list = Vec::new();

    let mut first = true;
    for row in seq_of_parameters.try_iter()? {
        let row = row?;

        let parameters = if let Ok(row) = row.extract() {
            to_parameter_named(row, first, &mut types, &mut placeholders, true)?
        } else {
            to_parameter_qmark(row, first, &mut types, &mut placeholders, true)?
        };

        parameters_list.push(parameters);

        first = false;
    }

    Ok((types, placeholders, parameters_list))
}

pub(crate) fn to_placeholders(
    row: Bound<PyAny>,
) -> PyResult<(HashMap<String, AtomType>, Vec<SqlPlaceholder>)> {
    let mut types = HashMap::new();
    let mut placeholders = Vec::new();
    if let Ok(row) = row.extract() {
        to_parameter_named(row, true, &mut types, &mut placeholders, false)?
    } else {
        to_parameter_qmark(row, true, &mut types, &mut placeholders, false)?
    };
    Ok((types, placeholders))
}

fn to_parameter_qmark(
    row: Bound<PyAny>,
    first: bool,
    types: &mut HashMap<String, AtomType>,
    placeholders: &mut Vec<SqlPlaceholder>,
    with_parameter: bool,
) -> PyResult<Vec<SqlParameter>> {
    let mut parameters = Vec::new();

    let mut i = 1;
    for value in row.try_iter()? {
        let value = value?;

        let placeholder_name = format!("{}", i);
        let atom_type = if first {
            to_atom_type(&value)?
        } else {
            *types.get(&placeholder_name).ok_or_else(|| {
                ProgrammingError::new_err(format!(
                    "parameter type not found. placeholder_number={}",
                    i
                ))
            })?
        };

        if first {
            let placeholder = SqlPlaceholder::of_atom_type(&placeholder_name, atom_type);
            placeholders.push(placeholder);
        }

        if with_parameter {
            let parameter = create_parameter(&placeholder_name, &value, atom_type)?;
            parameters.push(parameter);
        }

        if first {
            types.insert(placeholder_name, atom_type);
        }

        i += 1;
    }

    Ok(parameters)
}

fn to_parameter_named(
    row: Bound<PyDict>,
    first: bool,
    types: &mut HashMap<String, AtomType>,
    placeholders: &mut Vec<SqlPlaceholder>,
    with_parameter: bool,
) -> PyResult<Vec<SqlParameter>> {
    let mut parameters = Vec::new();

    for (key, value) in row.iter() {
        let placeholder_name: String = key.extract()?;
        let atom_type = if first {
            to_atom_type(&value)?
        } else {
            *types.get(&placeholder_name).ok_or_else(|| {
                ProgrammingError::new_err(format!(
                    "parameter type not found. placeholder_name={}",
                    placeholder_name
                ))
            })?
        };

        if first {
            let placeholder = SqlPlaceholder::of_atom_type(&placeholder_name, atom_type);
            placeholders.push(placeholder);
        }

        if with_parameter {
            let parameter = create_parameter(&placeholder_name, &value, atom_type)?;
            parameters.push(parameter);
        }

        if first {
            types.insert(placeholder_name, atom_type);
        }
    }

    Ok(parameters)
}

pub(crate) fn to_parameters_only(
    seq_of_parameters: Bound<PyAny>,
    types: &HashMap<String, AtomType>,
) -> PyResult<Vec<Vec<SqlParameter>>> {
    let mut parameters_list = Vec::new();

    for row in seq_of_parameters.try_iter()? {
        let row = row?;

        let parameters = if let Ok(row) = row.extract() {
            to_parameter_only_named(row, types)?
        } else {
            to_parameter_only_qmark(row, types)?
        };

        parameters_list.push(parameters);
    }

    Ok(parameters_list)
}

fn to_parameter_only_qmark(
    row: Bound<PyAny>,
    types: &HashMap<String, AtomType>,
) -> PyResult<Vec<SqlParameter>> {
    let mut parameters = Vec::new();

    let mut i = 1;
    for value in row.try_iter()? {
        let value = value?;

        let placeholder_name = format!("{}", i);
        let atom_type = types
            .get(&placeholder_name)
            .ok_or_else(|| ProgrammingError::new_err("parameter type not found"))?;
        let parameter = create_parameter(&placeholder_name, &value, *atom_type)?;

        parameters.push(parameter);

        i += 1;
    }

    Ok(parameters)
}

fn to_parameter_only_named(
    row: Bound<PyDict>,
    types: &HashMap<String, AtomType>,
) -> PyResult<Vec<SqlParameter>> {
    let mut parameters = Vec::new();

    for (key, value) in row.iter() {
        let placeholder_name: String = key.extract()?;
        let atom_type = types
            .get(&placeholder_name)
            .ok_or_else(|| ProgrammingError::new_err("parameter type not found"))?;

        let parameter = create_parameter(&placeholder_name, &value, *atom_type)?;
        parameters.push(parameter);
    }

    Ok(parameters)
}

fn to_atom_type(item: &Bound<PyAny>) -> PyResult<AtomType> {
    if item.is_none() {
        return Ok(AtomType::Unknown);
    }
    if item.is_instance_of::<PyDateTime>() {
        if let Ok(tzinfo) = item.getattr("tzinfo") {
            if tzinfo.is_none() {
                return Ok(AtomType::TimePoint);
            } else {
                return Ok(AtomType::TimePointWithTimeZone);
            }
        }
    }
    if item.is_instance_of::<PyTime>() {
        if let Ok(tzinfo) = item.getattr("tzinfo") {
            if tzinfo.is_none() {
                return Ok(AtomType::TimeOfDay);
            } else {
                return Ok(AtomType::TimeOfDayWithTimeZone);
            }
        }
    }

    let type_name: String = if let Ok(item_type) = item.extract::<Bound<PyType>>() {
        item_type.name()?.extract()?
    } else {
        item.get_type().name()?.extract()?
    };
    to_atom_type_from_name(&type_name)
}

fn to_atom_type_from_name(type_name: &str) -> PyResult<AtomType> {
    match type_name {
        "NoneType" => Ok(AtomType::Unknown),
        "bool"  // Python bool
        | "Bool" // Tsurugi Warper
            => Ok(AtomType::Boolean),
        "Int32" // Tsurugi Warper
        | "int32" // numpy.int32
            => Ok(AtomType::Int4),
        "int" // Python int
        | "Int64"  // Tsurugi Warper
        | "int64" // numpy.int64
            => Ok(AtomType::Int8),
        "Float32" // Tsurugi Warper
        | "float32" // numpy.float32
            => Ok(AtomType::Float4),
        "float"  // Python float
        | "Float64" // Tsurugi Warper
        | "float64" // numpy.float64
            => Ok(AtomType::Float8),
        "Decimal" // Python decimal.Decimal, Tsurugi Warper
            => Ok(AtomType::Decimal),
        "str" // Python str
        | "Str" // Tsurugi Warper
        | "str_" // numpy.str_
            => Ok(AtomType::Character),
        "bytes" // Python bytes
        | "Bytes" // Tsurugi Warper
        | "bytes_" // numpy.bytes_
            => Ok(AtomType::Octet),
        "date" // Python datetime.date
        | "Date" // Tsurugi Warper
            => Ok(AtomType::Date),
        "time" // Python datetime.time
        | "Time" // Tsurugi Warper
            => Ok(AtomType::TimeOfDay),
        "Datetime" // Tsurugi Warper
        | "datetime64" // numpy.datetime64
            => Ok(AtomType::TimePoint),
        "OffsetTime" // Tsurugi Warper
            => Ok(AtomType::TimeOfDayWithTimeZone),
        "datetime" // Python datetime.datetime
        | "OffsetDatetime" // Tsurugi Warper
            => Ok(AtomType::TimePointWithTimeZone),
        _ => Err(ProgrammingError::new_err(format!(
            "to_atom_type_from_name(): Unsupported type_name: {}",
            type_name
        ))),
    }
}

fn create_parameter(
    name: &str,
    value: &Bound<PyAny>,
    atom_type: AtomType,
) -> PyResult<SqlParameter> {
    match atom_type {
        AtomType::Boolean => {
            let v: Option<bool> = if let Ok(v) = value.extract::<PyRef<Bool>>() {
                v.value
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Int4 => {
            let v: Option<i32> = if let Ok(v) = value.extract::<PyRef<Int32>>() {
                v.value
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Int8 => {
            let v: Option<i64> = if let Ok(v) = value.extract::<PyRef<Int64>>() {
                v.value
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Float4 => {
            let v: Option<f32> = if let Ok(v) = value.extract::<PyRef<Float32>>() {
                v.value
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Float8 => {
            let v: Option<f64> = if let Ok(v) = value.extract::<PyRef<Float64>>() {
                v.value
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Decimal => {
            let v: Option<rust_decimal::Decimal> = if let Ok(v) = value.extract::<PyRef<Decimal>>()
            {
                v.value.clone()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Character => {
            let v: Option<String> = if let Ok(v) = value.extract::<PyRef<Str>>() {
                v.value.clone()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Octet => {
            let v: Option<Vec<u8>> = if let Ok(v) = value.extract::<PyRef<Bytes>>() {
                v.value.clone()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Date => {
            let v: Option<chrono::NaiveDate> = if let Ok(v) = value.extract::<PyRef<Date>>() {
                v.value
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::TimeOfDay => {
            let v: Option<chrono::NaiveTime> = if let Ok(v) = value.extract::<PyRef<Time>>() {
                v.value
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::TimePoint => {
            let v: Option<chrono::NaiveDateTime> = convert_datetime(value)?;
            Ok(SqlParameter::of(name, v))
        }
        AtomType::TimeOfDayWithTimeZone => {
            let v: Option<(chrono::NaiveTime, chrono::FixedOffset)> = convert_offset_time(value)?;
            Ok(SqlParameter::of(name, v))
        }
        AtomType::TimePointWithTimeZone => {
            let v: Option<chrono::DateTime<chrono::FixedOffset>> =
                if let Ok(v) = value.extract::<PyRef<OffsetDatetime>>() {
                    v.value
                } else {
                    value.extract()?
                };
            Ok(SqlParameter::of(name, v))
        }
        _ => Err(ProgrammingError::new_err(format!(
            "create_parameter: unsupported AtomType: {:?}",
            atom_type
        ))),
    }
}

fn convert_datetime(value: &Bound<PyAny>) -> PyResult<Option<chrono::NaiveDateTime>> {
    if value.is_none() {
        return Ok(None);
    }

    if let Ok(v) = value.extract::<PyRef<Datetime>>() {
        return Ok(v.value);
    }
    if let Ok(v) = value.call_method1("astype", ("datetime64[ns]",)) {
        let v = v.call_method1("astype", ("int64",))?;
        let epoch: i64 = v.extract()?;
        let dt = chrono::DateTime::<chrono::Utc>::from_timestamp_nanos(epoch);
        return Ok(Some(dt.naive_utc()));
    }

    Ok(value.extract()?)
}

fn convert_offset_time(
    value: &Bound<PyAny>,
) -> PyResult<Option<(chrono::NaiveTime, chrono::FixedOffset)>> {
    if value.is_none() {
        return Ok(None);
    }

    if let Ok(v) = value.extract::<PyRef<OffsetTime>>() {
        return Ok(v.value);
    }

    if value.is_instance_of::<PyTime>() {
        let time: chrono::NaiveTime = value.extract()?;
        let tzinfo = value.getattr("tzinfo")?;
        let offset: chrono::FixedOffset = if tzinfo.is_none() {
            chrono::FixedOffset::east_opt(0).unwrap()
        } else {
            tzinfo.extract()?
        };
        return Ok(Some((time, offset)));
    }

    Ok(value.extract()?)
}
