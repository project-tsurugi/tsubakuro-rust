use std::ffi::CString;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_cchar_to_str, ffi_arg_out_initialize, ffi_arg_require_non_null,
    return_code::{rc_ok, TsurugiFfiRc},
    TsurugiFfiStringHandle,
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

    trace!("{FUNCTION_NAME} end. parameter={:?}", handle);
    rc_ok(context)
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

    trace!("{FUNCTION_NAME} end. parameter={:?}", handle);
    rc_ok(context)
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

    trace!("{FUNCTION_NAME} end. parameter={:?}", handle);
    rc_ok(context)
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

    trace!("{FUNCTION_NAME} end. parameter={:?}", handle);
    rc_ok(context)
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

    trace!("{FUNCTION_NAME} end. parameter={:?}", handle);
    rc_ok(context)
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

    trace!("{FUNCTION_NAME} end. parameter={:?}", handle);
    rc_ok(context)
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

    unsafe {
        *name_out = cstring_to_cchar!(parameter.name);
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
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
