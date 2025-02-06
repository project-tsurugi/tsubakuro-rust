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

pub type TsurugiFfiSqlParameterHandle = *mut TsurugiFfiSqlParameter;

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
    let parameter = SqlParameter::of_decimal(name, (unscaled_value, exponent));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

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
    let parameter = SqlParameter::of_decimal_i128(name, (unscaled_value, exponent));

    let parameter = Box::new(TsurugiFfiSqlParameter::new(parameter));

    let handle = Box::into_raw(parameter);
    unsafe {
        *parameter_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. parameter={:?}", rc, handle);
    rc
}

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
        match parameter.name() {
            Some(name) => {
                let name = name.clone();
                cchar_field_set!(context, parameter.name, name);
            }
            None => {}
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
