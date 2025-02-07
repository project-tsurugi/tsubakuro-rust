use std::ffi::CString;

use crate::{TsurugiFfiStringArrayHandle, TsurugiFfiStringHandle};

#[macro_export]
macro_rules! ffi_arg_cchar_to_str {
    ($context:expr, $function_name:expr, $arg_index:expr, $arg:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let s = unsafe { std::ffi::CStr::from_ptr($arg) };
        match s.to_str() {
            Ok(value) => value,
            Err(e) => {
                return $crate::rc_ffi_arg_error!(
                    $context,
                    $function_name,
                    $arg_index,
                    stringify!($arg),
                    &e.to_string()
                )
            }
        }
    }};
}

#[macro_export]
macro_rules! ffi_str_to_cstring {
    ($context:expr, $value:expr) => {{
        match std::ffi::CString::new($value) {
            Ok(value) => value,
            Err(e) => {
                let rc = $crate::return_code::TSURUGI_FFI_RC_FFI_NUL_ERROR;
                let message = format!("CString::new() error. {:?}", e);
                let error = $crate::error::TsurugiFfiError::FfiError(rc, message);
                $crate::context::TsurugiFfiContext::set_error($context, rc, error);

                return rc;
            }
        }
    }};
}

#[macro_export]
macro_rules! ffi_str_to_cchar {
    ($context:expr, $value:expr) => {{
        match std::ffi::CString::new($value) {
            Ok(value) => value.into_raw(),
            Err(e) => {
                let rc = $crate::return_code::TSURUGI_FFI_RC_FFI_NUL_ERROR;
                let message = format!("CString::new() error. {:?}", e);
                let error = $crate::error::TsurugiFfiError::FfiError(rc, message);
                $crate::context::TsurugiFfiContext::set_error($context, rc, error);

                return rc;
            }
        }
    }};
}

#[macro_export]
macro_rules! cstring_to_cchar {
    ($value:expr) => {
        match &$value {
            Some(value) => value.as_ptr(),
            None => std::ptr::null(),
        }
    };
}

#[macro_export]
macro_rules! cchar_field_set {
    ($context:expr, $field:expr, $value:expr) => {{
        $field = Some($crate::ffi_str_to_cstring!($context, $value));
    }};
}

#[macro_export]
macro_rules! cchar_field_clear {
    ($field:expr) => {{
        $field = None;
    }};
}

#[derive(Debug)]
pub(crate) struct TsurugiFfiCStringArray {
    vec: Vec<CString>,
    ptr_vec: Vec<TsurugiFfiStringHandle>,
}

impl TsurugiFfiCStringArray {
    pub(crate) fn new(capacity: usize) -> TsurugiFfiCStringArray {
        TsurugiFfiCStringArray {
            vec: Vec::with_capacity(capacity),
            ptr_vec: Vec::with_capacity(capacity),
        }
    }

    pub(crate) fn push(&mut self, s: CString) {
        self.ptr_vec.push(s.as_ptr());
        self.vec.push(s);
    }

    pub(crate) fn as_ptr(&self) -> TsurugiFfiStringArrayHandle {
        self.ptr_vec.as_ptr()
    }
}

#[macro_export]
macro_rules! cstring_array_field_set_if_none {
    ($context:expr, $field:expr, $values:expr) => {{
        if $field.is_none() {
            let mut array = $crate::util::cchar::TsurugiFfiCStringArray::new($values.len());
            for value in $values {
                let s = value.to_string();
                let s = $crate::ffi_str_to_cstring!($context, s);
                array.push(s);
            }
            $field = Some(array);
        }
    }};
}

#[macro_export]
macro_rules! cstring_array_field_clear {
    ($field:expr) => {{
        $field = None;
    }};
}

#[macro_export]
macro_rules! cstring_array_field_to_ptr {
    ($field:expr) => {
        match &$field {
            Some(value) => value.as_ptr(),
            None => std::ptr::null(),
        }
    };
}
