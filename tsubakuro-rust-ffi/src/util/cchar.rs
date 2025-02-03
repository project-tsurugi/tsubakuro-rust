#[macro_export]
macro_rules! ffi_arg_cchar_to_str {
    ($context:expr, $function_name:expr, $arg_index:expr, $arg:expr) => {{
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

#[macro_export]
macro_rules! vec_cchar_field_set_if_none {
    ($context:expr, $field:expr, $ptr_field:expr, $values:expr) => {{
        if $field.is_none() {
            let mut vec = Vec::with_capacity($values.len());
            let mut ptr_vec = Vec::with_capacity($values.len());
            for value in $values {
                let s = value.to_string();
                let s = $crate::ffi_str_to_cstring!($context, s);
                ptr_vec.push(s.as_ptr());
                vec.push(s);
            }
            $field = Some(vec);
            $ptr_field = Some(ptr_vec);
        }
    }};
}

#[macro_export]
macro_rules! vec_cchar_field_clear {
    ($field:expr, $ptr_field:expr) => {{
        $ptr_field = None;
        $field = None;
    }};
}

#[macro_export]
macro_rules! vec_cchar_field_to_ptr {
    ($ptr_field:expr) => {
        match &$ptr_field {
            Some(value) => value.as_ptr(),
            None => std::ptr::null(),
        }
    };
}
