mod cchar;

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
