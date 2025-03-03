//! atom type.

use tsubakuro_rust_core::prelude::AtomType;

// copy from AtomType
/// Atom type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TsurugiFfiAtomType {
    /// unspecified type.
    TypeUnspecified = 0,
    /// boolean type.
    Boolean = 1,
    /// 32-bit signed integer.
    Int4 = 4,
    /// 64-bit signed integer.
    Int8 = 5,
    /// 32-bit floating point number.
    Float4 = 6,
    /// 64-bit floating point number.
    Float8 = 7,
    /// multi precision decimal number.
    Decimal = 8,
    /// character sequence.
    Character = 9,
    /// octet sequence.
    Octet = 11,
    /// bit sequence.
    Bit = 13,
    /// date.
    Date = 15,
    /// time of day.
    TimeOfDay = 16,
    /// time point.
    TimePoint = 17,
    /// date-time interval.
    DatetimeInterval = 18,
    /// time of day with time zone.
    TimeOfDayWithTimeZone = 19,
    /// time point with time zone.
    TimePointWithTimeZone = 20,
    /// character large objects.
    Clob = 21,
    /// binary large objects.
    Blob = 22,
    /// unknown type.
    Unknown = 31,
    /// unrecognized.
    Unrecognized = -1,
}

macro_rules! impl_from_for_enum {
    ($from:ident, $to:ident { $($variant:ident),*$(,)? }) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                match value {
                    $(
                        $from::$variant => Self::$variant,
                    )*
                }
            }
        }

        impl From<$to> for $from {
            fn from(value: $to) -> Self {
                match value {
                    $(
                        $to::$variant => $from::$variant,
                    )*
                    _ => $from::Unknown,
                }
            }
        }
    };
}

impl_from_for_enum! {AtomType, TsurugiFfiAtomType {
    TypeUnspecified,
    Boolean,
    Int4,
    Int8,
    Float4,
    Float8,
    Decimal,
    Character,
    Octet,
    Bit,
    Date,
    TimeOfDay,
    TimePoint,
    DatetimeInterval,
    TimeOfDayWithTimeZone,
    TimePointWithTimeZone,
    Clob,
    Blob,
    Unknown,
}}
