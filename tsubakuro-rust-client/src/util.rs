use prost::alloc::string::String as ProstString;

pub(crate) fn string_to_prost_string(s: Option<&String>) -> ProstString {
    if let Some(s) = s {
        ProstString::from(s)
    } else {
        ProstString::new()
    }
}
