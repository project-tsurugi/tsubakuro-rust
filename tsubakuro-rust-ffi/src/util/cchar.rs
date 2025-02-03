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
macro_rules! ffi_cchar_dispose {
    ($value:expr) => {
        let _ = std::ffi::CString::from_raw($value);
    };
}

#[macro_export]
macro_rules! cchar_field_set {
    ($context:expr, $field:expr, $value:expr) => {{
        $crate::cchar_field_clear!($field);

        $field = $crate::ffi_str_to_cchar!($context, $value);
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
            $crate::ffi_cchar_dispose!($field);
        }
    };
}

#[macro_export]
macro_rules! vec_cchar_field_set_if_none {
    ($context:expr, $field:expr, $values:expr) => {{
        if $field.is_none() {
            let mut vec = Vec::with_capacity($values.len());
            for value in $values {
                let s = value.to_string();
                let s = $crate::ffi_str_to_cchar!($context, s);
                vec.push(s);
            }
            $field = Some(vec);
        }
    }};
}

#[macro_export]
macro_rules! vec_cchar_field_clear {
    ($field:expr) => {
        if $field.is_some() {
            let ss = $field.take().unwrap();
            for s in ss {
                $crate::ffi_cchar_dispose!(s);
            }
        }
    };
}

#[macro_export]
macro_rules! vec_cchar_field_dispose {
    ($field:expr) => {
        if let Some(ss) = $field {
            for s in ss {
                $crate::ffi_cchar_dispose!(s);
            }
        }
    };
}
