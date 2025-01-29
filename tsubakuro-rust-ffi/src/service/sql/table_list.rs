use std::ffi::c_char;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_require_non_null, ffi_cchar_dispose, ffi_str_to_cchar, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
};

pub(crate) struct TsurugiFfiTableList {
    table_list: TableList,
    table_names: Option<Vec<*mut c_char>>,
}

impl TsurugiFfiTableList {
    pub(crate) fn new(table_list: TableList) -> TsurugiFfiTableList {
        TsurugiFfiTableList {
            table_list,
            table_names: None,
        }
    }
}

impl std::ops::Deref for TsurugiFfiTableList {
    type Target = TableList;

    fn deref(&self) -> &Self::Target {
        &self.table_list
    }
}

impl std::ops::DerefMut for TsurugiFfiTableList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table_list
    }
}

pub type TsurugiFfiTableListHandle = *mut TsurugiFfiTableList;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_list_get_table_names_size(
    context: TsurugiFfiContextHandle,
    table_list: TsurugiFfiTableListHandle,
    size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_list_get_table_names_size()";
    trace!("{FUNCTION_NAME} start. table_list={:?}", table_list);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, size_out);
    unsafe {
        *size_out = 0;
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, table_list);

    let table_list = unsafe { &*table_list };
    let table_names = table_list.table_names();

    unsafe {
        *size_out = table_names.len() as u32;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_list_get_table_names_value(
    context: TsurugiFfiContextHandle,
    table_list: TsurugiFfiTableListHandle,
    index: u32,
    table_name_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_list_get_table_names_value()";
    trace!("{FUNCTION_NAME} start. table_list={:?}", table_list);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_name_out);
    unsafe {
        *table_name_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, table_list);

    let table_list = unsafe { &mut *table_list };
    let table_names = table_list.table_names();

    let index = index as usize;
    if index >= table_names.len() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "index", "out of bounds");
    }

    // TODO mutex.lock
    if table_list.table_names.is_none() {
        let mut vec = Vec::with_capacity(table_names.len());
        for tname in table_names {
            let s = tname.to_string();
            let s = ffi_str_to_cchar!(context, s);
            vec.push(s);
        }
        table_list.table_names = Some(vec);
    }

    let table_name = table_list.table_names.as_ref().unwrap()[index];

    unsafe {
        *table_name_out = table_name;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_list_dispose(table_list: TsurugiFfiTableListHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_list_dispose()";
    trace!("{FUNCTION_NAME} start. table_list={:?}", table_list);

    if table_list.is_null() {
        trace!("{FUNCTION_NAME} end. arg[table_list] is null");
        return;
    }

    unsafe {
        let table_list = Box::from_raw(table_list);

        if let Some(table_names) = table_list.table_names {
            for table_name in table_names {
                ffi_cchar_dispose!(table_name);
            }
        }
    }

    trace!("{FUNCTION_NAME} end");
}
