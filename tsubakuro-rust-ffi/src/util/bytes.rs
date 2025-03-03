#[doc(hidden)]
#[macro_export]
macro_rules! bytes_to_vec_u8 {
    ($value:expr, $size:expr) => {
        unsafe { std::slice::from_raw_parts($value, $size as usize) }.to_vec()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! vec_u8_to_field {
    ($field:expr, $vec:expr) => {{
        let size = $vec.len();
        $field = Some($vec);

        if size > 0 {
            ($field.as_ref().unwrap().as_ptr(), size as u64)
        } else {
            (std::ptr::null(), 0u64)
        }
    }};
}
