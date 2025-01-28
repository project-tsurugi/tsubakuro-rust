use std::ffi::c_char;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_dispose, cchar_field_set,
    context::TsurugiFfiContextHandle,
    ffi_arg_cchar_to_str, ffi_arg_require_non_null, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
    service::sql::atom_type::TsurugiFfiAtomType,
};

#[derive(Debug)]
pub(crate) struct TsurugiFfiSqlPlaceholder {
    placeholder: SqlPlaceholder,
    name: *mut c_char,
}

impl std::ops::Deref for TsurugiFfiSqlPlaceholder {
    type Target = SqlPlaceholder;

    fn deref(&self) -> &Self::Target {
        &self.placeholder
    }
}

impl std::ops::DerefMut for TsurugiFfiSqlPlaceholder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.placeholder
    }
}

pub type TsurugiFfiSqlPlaceholderHandle = *mut TsurugiFfiSqlPlaceholder;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_placeholder_of_atom_type(
    context: TsurugiFfiContextHandle,
    name: *const c_char,
    atom_type: TsurugiFfiAtomType,
    placeholder_out: *mut TsurugiFfiSqlPlaceholderHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_placeholder_of_atom_type()";
    trace!("{FUNCTION_NAME} start");

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    if !TsurugiFfiAtomType::is_valid(atom_type as i32) {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "atom_type", "is invalid");
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, placeholder_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let placeholder = SqlPlaceholder::of_atom_type(name, atom_type.into());

    let placeholder = Box::new(TsurugiFfiSqlPlaceholder {
        placeholder,
        name: std::ptr::null_mut(),
    });

    let handle = Box::into_raw(placeholder);
    unsafe {
        *placeholder_out = handle;
    }

    trace!("{FUNCTION_NAME} end. placeholder={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_placeholder_get_name(
    context: TsurugiFfiContextHandle,
    placeholder: TsurugiFfiSqlPlaceholderHandle,
    name_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_placeholder_get_name()";
    trace!("{FUNCTION_NAME} start. placeholder={:?}", placeholder);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, placeholder);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, name_out);

    let placeholder = unsafe { &mut *placeholder };

    if placeholder.name.is_null() {
        match placeholder.name() {
            Some(name) => unsafe {
                let name = name.clone();
                cchar_field_set!(context, placeholder.name, name);
            },
            None => {}
        }
    }

    unsafe {
        *name_out = placeholder.name;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_placeholder_get_atom_type(
    context: TsurugiFfiContextHandle,
    placeholder: TsurugiFfiSqlPlaceholderHandle,
    atom_type_out: *mut TsurugiFfiAtomType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_placeholder_get_atom_type()";
    trace!("{FUNCTION_NAME} start. placeholder={:?}", placeholder);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, placeholder);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, atom_type_out);

    let placeholder = unsafe { &*placeholder };

    let atom_type = match placeholder.atom_type() {
        Some(value) => value.into(),
        None => TsurugiFfiAtomType::Unrecognized,
    };

    unsafe {
        *atom_type_out = atom_type;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_placeholder_dispose(placeholder: TsurugiFfiSqlPlaceholderHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_placeholder_dispose()";
    trace!("{FUNCTION_NAME} start. placeholder={:?}", placeholder);

    if placeholder.is_null() {
        trace!("{FUNCTION_NAME} end. arg[placeholder] is null");
        return;
    }

    unsafe {
        let placeholder = Box::from_raw(placeholder);

        cchar_field_dispose!(placeholder.name);
    }

    trace!("{FUNCTION_NAME} end");
}
