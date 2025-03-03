//! Sql placeholder.

use std::ffi::CString;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_cchar_to_str, ffi_arg_out_initialize, ffi_arg_require_non_null,
    return_code::{rc_ok, TsurugiFfiRc},
    service::sql::atom_type::TsurugiFfiAtomType,
    TsurugiFfiStringHandle,
};

#[derive(Debug)]
pub(crate) struct TsurugiFfiSqlPlaceholder {
    placeholder: SqlPlaceholder,
    name: Option<CString>,
}

impl TsurugiFfiSqlPlaceholder {
    pub(crate) fn raw_clone(&self) -> SqlPlaceholder {
        self.placeholder.clone()
    }
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

/// Sql placeholder.
pub type TsurugiFfiSqlPlaceholderHandle = *mut TsurugiFfiSqlPlaceholder;

/// SqlPlaceholder: Creates a placeholder.
///
/// See [`SqlPlaceholder::of_atom_type`].
///
/// # Parameters
/// - `name` - placeholder name.
/// - `atom_type` - parameter type.
///
/// # Returns
/// - `placeholder_out` - placeholder. To dispose, call `tsurugi_ffi_sql_placeholder_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_placeholder_of_atom_type(
    context: TsurugiFfiContextHandle,
    name: TsurugiFfiStringHandle,
    atom_type: TsurugiFfiAtomType,
    placeholder_out: *mut TsurugiFfiSqlPlaceholderHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_placeholder_of_atom_type()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, name={:?}, atom_type={:?}, placeholder_out={:?}",
        context,
        name,
        atom_type as i32,
        placeholder_out
    );

    ffi_arg_out_initialize!(placeholder_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, placeholder_out);

    let name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, name);
    let placeholder = SqlPlaceholder::of_atom_type(name, atom_type.into());

    let placeholder = Box::new(TsurugiFfiSqlPlaceholder {
        placeholder,
        name: None,
    });

    let handle = Box::into_raw(placeholder);
    unsafe {
        *placeholder_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. placeholder={:?}", rc, handle);
    rc
}

/// SqlPlaceholder: Get name.
///
/// See [`SqlPlaceholder::name`].
///
/// # Receiver
/// - `placeholder` - Sql placeholder.
///
/// # Returns
/// - `name_out` - placeholder name.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_placeholder_get_name(
    context: TsurugiFfiContextHandle,
    placeholder: TsurugiFfiSqlPlaceholderHandle,
    name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_placeholder_get_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, placeholder={:?}, name_out={:?}",
        context,
        placeholder,
        name_out
    );

    ffi_arg_out_initialize!(name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, placeholder);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, name_out);

    let placeholder = unsafe { &mut *placeholder };

    if placeholder.name.is_none() {
        if let Some(name) = placeholder.name() {
            let name = name.clone();
            cchar_field_set!(context, placeholder.name, name);
        }
    }

    let ptr = cstring_to_cchar!(placeholder.name);
    unsafe {
        *name_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (name={:?})", rc, ptr);
    rc
}

/// SqlPlaceholder: Get AtomType.
///
/// See [`SqlPlaceholder::atom_type`].
///
/// # Receiver
/// - `placeholder` - Sql placeholder.
///
/// # Returns
/// - `atom_type_out` - placeholder type.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_placeholder_get_atom_type(
    context: TsurugiFfiContextHandle,
    placeholder: TsurugiFfiSqlPlaceholderHandle,
    atom_type_out: *mut TsurugiFfiAtomType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_placeholder_get_atom_type()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, placeholder={:?}, atom_type_out={:?}",
        context,
        placeholder,
        atom_type_out
    );

    ffi_arg_out_initialize!(atom_type_out, TsurugiFfiAtomType::Unrecognized);
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

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (atom_type={:?})",
        rc,
        atom_type as i32
    );
    rc
}

/// SqlPlaceholder: Dispose.
///
/// # Receiver
/// - `placeholder` - Sql placeholder.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_placeholder_dispose(placeholder: TsurugiFfiSqlPlaceholderHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_placeholder_dispose()";
    trace!("{FUNCTION_NAME} start. placeholder={:?}", placeholder);

    if placeholder.is_null() {
        trace!("{FUNCTION_NAME} end. arg[placeholder] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(placeholder);
    }

    trace!("{FUNCTION_NAME} end");
}
