//! Sql parameter.

use std::ffi::CString;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    bytes_to_vec_u8, cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_cchar_to_str, ffi_arg_out_initialize, ffi_arg_require_non_null,
    return_code::{rc_ok, TsurugiFfiRc},
    TsurugiFfiByteArrayHandle, TsurugiFfiStringHandle,
};

#[derive(Debug)]
pub(crate) struct TsurugiFfiSqlParameter {
    parameter: SqlParameter,
    name: Option<CString>,
}

impl TsurugiFfiSqlParameter {
    fn new(parameter: SqlParameter) -> TsurugiFfiSqlParameter {
        TsurugiFfiSqlParameter {
            parameter,
            name: None,
        }
    }

    pub(crate) fn raw_clone(&self) -> SqlParameter {
        self.parameter.clone()
    }
}

impl std::ops::Deref for TsurugiFfiSqlParameter {
    type Target = SqlParameter;

    fn deref(&self) -> &Self::Target {
        &self.parameter
    }
}

impl std::ops::DerefMut for TsurugiFfiSqlParameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parameter
    }
}

/// Sql parameter.
pub type TsurugiFfiSqlParameterHandle = *mut TsurugiFfiSqlParameter;

/// SqlParameter: Creates a null parameter.
///
/// See [`SqlParameter::null`].
///
/// # Parameters
/// - `name` - parameter name.
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_null(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_null()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, parameter_out={:?}",
        context,
        name,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let parameter = SqlParameter::null(name);

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of int4 (int).
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `value` - parameter value.
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_int4(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    value: i32,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_int4()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, value={:?}, parameter_out={:?}",
        context,
        name,
        value,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let parameter = SqlParameter::of(name, value);

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of int8 (bigint).
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `value` - parameter value.
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_int8(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    value: i64,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_int8()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, value={:?}, parameter_out={:?}",
        context,
        name,
        value,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let parameter = SqlParameter::of(name, value);

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of float4 (real).
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `value` - parameter value.
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_float4(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    value: f32,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_float4()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, value={:?}, parameter_out={:?}",
        context,
        name,
        value,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let parameter = SqlParameter::of(name, value);

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of float8 (double).
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `value` - parameter value.
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_float8(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    value: f64,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_float8()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, value={:?}, parameter_out={:?}",
        context,
        name,
        value,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let parameter = SqlParameter::of(name, value);

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of decimal.
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `unscaled_value` - unscaled value of decimal.
/// - `unscaled_value_size` - `unscaled_value` size \[byte\].
/// - `exponent` - exponent of decimal.
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_decimal(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    unscaled_value: TsurugiFfiByteArrayHandle,
    unscaled_value_size: u32,
    exponent: i32,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_decimal()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, unscaled_value={:?}, unscaled_value_size={:?}, exponent={:?}, parameter_out={:?}",
        context,
        name,
        unscaled_value,
        unscaled_value_size,
        exponent,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, unscaled_value);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let unscaled_value = bytes_to_vec_u8!(unscaled_value, unscaled_value_size);
    let parameter = SqlParameter::of(name, TgDecimal::new(unscaled_value, exponent));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of decimal.
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `unscaled_value_high` - unscaled value of decimal (high 64 bit).
/// - `unscaled_value_low` - unscaled value of decimal (low 64 bit).
/// - `exponent` - exponent of decimal.
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_decimal_i128(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    unscaled_value_high: i64,
    unscaled_value_low: u64,
    exponent: i32,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_decimal_i128()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, unscaled_value_high={:?}, unscaled_value_low={:?}, exponent={:?}, parameter_out={:?}",
        context,
        name,
        unscaled_value_high,
        unscaled_value_low,
        exponent,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let unscaled_value = ((unscaled_value_high as i128) << 64) | (unscaled_value_low as i128);
    let parameter = SqlParameter::of(name, TgDecimalI128::new(unscaled_value, exponent));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of character (char/varchar).
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `value` - parameter value.
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_character(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    value: TsurugiFfiStringHandle,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_character()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, value={:?}, parameter_out={:?}",
        context,
        name,
        value,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let value = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, value);
    let parameter = SqlParameter::of(name, value);

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of octet (binary/varbinary).
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `value` - parameter value.
/// - `value_size` - `value` size \[byte\].
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_octet(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    value: TsurugiFfiByteArrayHandle,
    value_size: u64,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_octet()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, value={:?}, value_size={:?}, parameter_out={:?}",
        context,
        name,
        value,
        value_size,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let value = bytes_to_vec_u8!(value, value_size);
    let parameter = SqlParameter::of(name, value);

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of date.
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `epoch_days` - parameter value (number of days offset of epoch 1970-01-01).
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_date(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    epoch_days: i64,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_date()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, epoch_days={:?}, parameter_out={:?}",
        context,
        name,
        epoch_days,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let parameter = SqlParameter::of(name, TgDate::new(epoch_days));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of time of day (time).
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `nanoseconds_of_day` - parameter value (nanoseconds since 00:00:00).
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_time_of_day(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    nanoseconds_of_day: u64,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_time_of_day()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, nanoseconds_of_day={:?}, parameter_out={:?}",
        context,
        name,
        nanoseconds_of_day,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let parameter = SqlParameter::of(name, TgTimeOfDay::new(nanoseconds_of_day));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of time point (timestamp).
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `epoch_seconds` - parameter value (number of seconds offset of epoch 1970-01-01).
/// - `nanos` - parameter value (nanoseconds adjustment \[0, 10^9-1\]).
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_time_point(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    epoch_seconds: i64,
    nanos: u32,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_time_point()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, epoch_seconds={:?}, nanos={:?}, parameter_out={:?}",
        context,
        name,
        epoch_seconds,
        nanos,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let parameter = SqlParameter::of(name, TgTimePoint::new(epoch_seconds, nanos));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of time of day with time zone (time with time zone).
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `nanoseconds_of_day` - parameter value (nanoseconds since 00:00:00).
/// - `time_zone_offset` - parameter value (timezone offset in minute).
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_time_of_day_with_time_zone(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    nanoseconds_of_day: u64,
    time_zone_offset: i32,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_time_of_day_with_time_zone()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, nanoseconds_of_day={:?}, time_zone_offset={:?}, parameter_out={:?}",
        context,
        name,
        nanoseconds_of_day,
        time_zone_offset,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let parameter = SqlParameter::of(
        name,
        TgTimeOfDayWithTimeZone::new(nanoseconds_of_day, time_zone_offset),
    );

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of time point with time zone (timestamp with time zone).
///
/// See [`SqlParameter::of`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `epoch_seconds` - parameter value (number of seconds offset of epoch 1970-01-01).
/// - `nanos` - parameter value (nanoseconds adjustment \[0, 10^9-1\]).
/// - `time_zone_offset` - parameter value (timezone offset in minute).
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_time_point_with_time_zone(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    epoch_seconds: i64,
    nanos: u32,
    time_zone_offset: i32,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_time_point_with_time_zone()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, epoch_seconds={:?}, nanos={:?}, time_zone_offset={:?}, parameter_out={:?}",
        context,
        name,
        epoch_seconds,
        nanos,
        time_zone_offset,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let parameter = SqlParameter::of(
        name,
        TgTimePointWithTimeZone::new(epoch_seconds, nanos, time_zone_offset),
    );

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of blob.
///
/// See [`TgBlob::new`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `path` - parameter value (path of file).
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_blob(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    path: TsurugiFfiStringHandle,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_blob()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, path={:?}, parameter_out={:?}",
        context,
        name,
        path,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, path);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let path = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, path);
    let parameter = SqlParameter::of(name, TgBlob::new(path));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of blob.
///
/// See [`TgBlob::from`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `value` - parameter value.
/// - `value_size` - `value` size \[byte\].
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_blob_contents(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    value: TsurugiFfiByteArrayHandle,
    value_size: u64,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_blob()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, value={:?}, value_size={:?}, parameter_out={:?}",
        context,
        name,
        value,
        value_size,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let value = bytes_to_vec_u8!(value, value_size);
    let parameter = SqlParameter::of(name, TgBlob::from(value));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of clob.
///
/// See [`TgClob::new`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `path` - parameter value (path of file).
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_clob(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    path: TsurugiFfiStringHandle,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_clob()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, path={:?}, parameter_out={:?}",
        context,
        name,
        path,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, path);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let path = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, path);
    let parameter = SqlParameter::of(name, TgClob::new(path));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Creates a parameter of clob.
///
/// See [`TgClob::from`].
///
/// # Parameters
/// - `name` - parameter name.
/// - `value` - parameter value.
///
/// # Returns
/// - `parameter_out` - parameter. To dispose, call `tsurugi_ffi_sql_parameter_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_of_clob_contents(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    value: TsurugiFfiStringHandle,
    parameter_out: *mut TsurugiFfiSqlParameterHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_of_clob()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, value={:?}, parameter_out={:?}",
        context,
        name,
        value,
        parameter_out
    );

    ffi_arg_out_initialize!(parameter_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, parameter_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let value = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, value);
    let parameter = SqlParameter::of(name, TgClob::from(value));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

/// SqlParameter: Get name.
///
/// See [`SqlParameter::name`].
///
/// # Receiver
/// - `parameter` - Sql parameter.
///
/// # Returns
/// - `name_out` - parameter name.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_get_name(
    context: TsurugiFfiContextHandle,
    parameter: TsurugiFfiSqlParameterHandle,
    name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_get_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, parameter={:?}, name_out={:?}",
        context,
        parameter,
        name_out
    );

    ffi_arg_out_initialize!(name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, parameter);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, name_out);

    let parameter = unsafe { &mut *parameter };

    if parameter.name.is_none() {
        if let Some(name) = parameter.name() {
            let name = name.clone();
            cchar_field_set!(context, parameter.name, name);
        }
    }

    let ptr = cstring_to_cchar!(parameter.name);
    unsafe {
        *name_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (name={:?})", rc, ptr);
    rc
}

/// SqlParameter: Dispose.
///
/// # Receiver
/// - `parameter` - Sql parameter.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_parameter_dispose(parameter: TsurugiFfiSqlParameterHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_parameter_dispose()";
    trace!("{FUNCTION_NAME} start. parameter={:?}", parameter);

    if parameter.is_null() {
        trace!("{FUNCTION_NAME} end. arg[parameter] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(parameter);
    }

    trace!("{FUNCTION_NAME} end");
}
