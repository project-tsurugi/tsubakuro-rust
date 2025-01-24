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
macro_rules! cchar_field_set {
    ($context:expr, $field:expr, $string:expr) => {{
        $crate::cchar_field_clear!($field);

        match std::ffi::CString::new($string) {
            Ok(s) => {
                $field = s.into_raw();
            }
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
macro_rules! cchar_field_clear {
    ($field:expr) => {
        if !$field.is_null() {
            $crate::cchar_field_dispose!($field);

            $field = std::ptr::null_mut();
        }
    };
}

#[macro_export]
macro_rules! cchar_field_dispose {
    ($field:expr) => {
        if !$field.is_null() {
            let _ = std::ffi::CString::from_raw($field);
        }
    };
}
