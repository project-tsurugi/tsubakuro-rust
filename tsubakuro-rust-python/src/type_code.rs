use std::collections::HashMap;

use pyo3::{prelude::*, types::*};
use tsubakuro_rust_core::prelude::{AtomType, SqlParameter, SqlParameterOf, SqlPlaceholder};

use crate::{
    error::ProgrammingError,
    type_code::{
        bool::Bool, bytes::Bytes, date::Date, datetime::Datetime, decimal::Decimal,
        float32::Float32, float64::Float64, int32::Int32, int64::Int64,
        offset_datetime::OffsetDatetime, offset_time::OffsetTime, str::Str, time::Time,
    },
};

pub mod bool;
pub mod bytes;
pub mod date;
pub mod datetime;
pub mod decimal;
pub mod float32;
pub mod float64;
pub mod int32;
pub mod int64;
pub mod offset_datetime;
pub mod offset_time;
pub mod str;
pub mod time;

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
                v.value()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Int4 => {
            let v: Option<i32> = if let Ok(v) = value.extract::<PyRef<Int32>>() {
                v.value()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Int8 => {
            let v: Option<i64> = if let Ok(v) = value.extract::<PyRef<Int64>>() {
                v.value()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Float4 => {
            let v: Option<f32> = if let Ok(v) = value.extract::<PyRef<Float32>>() {
                v.value()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Float8 => {
            let v: Option<f64> = if let Ok(v) = value.extract::<PyRef<Float64>>() {
                v.value()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Decimal => {
            let v: Option<rust_decimal::Decimal> = if let Ok(v) = value.extract::<PyRef<Decimal>>()
            {
                v.value()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Character => {
            let v: Option<String> = if let Ok(v) = value.extract::<PyRef<Str>>() {
                v.value().clone()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Octet => {
            let v: Option<Vec<u8>> = if let Ok(v) = value.extract::<PyRef<Bytes>>() {
                v.value().clone()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::Date => {
            let v: Option<chrono::NaiveDate> = if let Ok(v) = value.extract::<PyRef<Date>>() {
                v.value().clone()
            } else {
                value.extract()?
            };
            Ok(SqlParameter::of(name, v))
        }
        AtomType::TimeOfDay => {
            let v: Option<chrono::NaiveTime> = if let Ok(v) = value.extract::<PyRef<Time>>() {
                v.value().clone()
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
                    v.value().clone()
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
        return Ok(v.value().clone());
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
        return Ok(v.value().clone());
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
