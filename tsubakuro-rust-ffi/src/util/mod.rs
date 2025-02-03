mod cchar;

#[macro_export]
macro_rules! ffi_arg_out_initialize {
    ($arg:expr, $value:expr) => {
        if (!$arg.is_null()) {
            unsafe {
                *$arg = $value;
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_arg_require_non_null {
    ($context:expr, $function_name:expr, $arg_index:expr, $arg:expr) => {{
        if ($arg.is_null()) {
            return $crate::rc_ffi_arg_error!(
                $context,
                $function_name,
                $arg_index,
                stringify!($arg),
                "is null"
            );
        }
    }};
}

#[macro_export]
macro_rules! ffi_exec_core {
    ($context:expr, $function_name:expr, $executor:expr) => {
        match ($executor) {
            Ok(value) => value,
            Err(e) => return $crate::rc_core_error!($context, $function_name, e),
        }
    };
}

#[macro_export]
macro_rules! ffi_exec_core_async {
    ($context:expr, $function_name:expr, $runtime:expr, $executor:expr) => {
        match $runtime.block_on($executor) {
            Ok(value) => value,
            Err(e) => return $crate::rc_core_error!($context, $function_name, e),
        }
    };
}
