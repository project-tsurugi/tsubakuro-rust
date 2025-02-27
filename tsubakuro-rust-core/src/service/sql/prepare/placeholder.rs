use crate::jogasaki::proto::sql::common::AtomType;
use crate::jogasaki::proto::sql::request::placeholder::{Placement, TypeInfo};
use crate::jogasaki::proto::sql::request::Placeholder as SqlPlaceholder;
use crate::prelude::{
    TgBlob, TgClob, TgDate, TgDecimal, TgDecimalI128, TgTimeOfDay, TgTimeOfDayWithTimeZone,
    TgTimePoint, TgTimePointWithTimeZone,
};

impl SqlPlaceholder {
    fn new(name: &str, type_info: TypeInfo, dimension: u32) -> SqlPlaceholder {
        let placement = Placement::Name(name.to_string());

        SqlPlaceholder {
            dimension,
            placement: Some(placement),
            type_info: Some(type_info),
        }
    }

    /// get name.
    pub fn name(&self) -> Option<&String> {
        match self.placement {
            Some(Placement::Name(ref name)) => Some(name),
            _ => None,
        }
    }

    /// get AtomType.
    pub fn atom_type(&self) -> Option<AtomType> {
        match self.type_info {
            Some(TypeInfo::AtomType(atom_type)) => AtomType::try_from(atom_type).ok(),
            _ => None,
        }
    }
}

impl SqlPlaceholder {
    /// Creates a new instance.
    pub fn of_atom_type(name: &str, atom_type: AtomType) -> SqlPlaceholder {
        let type_info = TypeInfo::AtomType(atom_type.into());
        SqlPlaceholder::new(name, type_info, 0)
    }

    /// Creates a new instance.
    pub fn of<T: AtomTypeProvider>(name: &str) -> Self {
        let atom_type = T::atom_type();
        SqlPlaceholder::of_atom_type(name, atom_type)
    }
}

/// AtomType provider for [SqlPlaceholder].
pub trait AtomTypeProvider {
    /// get Atomtype.
    fn atom_type() -> AtomType;
}

impl AtomTypeProvider for bool {
    fn atom_type() -> AtomType {
        AtomType::Boolean
    }
}

impl AtomTypeProvider for i32 {
    fn atom_type() -> AtomType {
        AtomType::Int4
    }
}

impl AtomTypeProvider for i64 {
    fn atom_type() -> AtomType {
        AtomType::Int8
    }
}

impl AtomTypeProvider for f32 {
    fn atom_type() -> AtomType {
        AtomType::Float4
    }
}

impl AtomTypeProvider for f64 {
    fn atom_type() -> AtomType {
        AtomType::Float8
    }
}

impl AtomTypeProvider for TgDecimal {
    fn atom_type() -> AtomType {
        AtomType::Decimal
    }
}

impl AtomTypeProvider for TgDecimalI128 {
    fn atom_type() -> AtomType {
        AtomType::Decimal
    }
}

#[cfg(feature = "with_bigdecimal")]
impl AtomTypeProvider for bigdecimal::BigDecimal {
    fn atom_type() -> AtomType {
        AtomType::Decimal
    }
}

#[cfg(feature = "with_rust_decimal")]
impl AtomTypeProvider for rust_decimal::Decimal {
    fn atom_type() -> AtomType {
        AtomType::Decimal
    }
}

impl AtomTypeProvider for &str {
    fn atom_type() -> AtomType {
        AtomType::Character
    }
}

impl AtomTypeProvider for String {
    fn atom_type() -> AtomType {
        AtomType::Character
    }
}

impl AtomTypeProvider for &[u8] {
    fn atom_type() -> AtomType {
        AtomType::Octet
    }
}

impl AtomTypeProvider for Vec<u8> {
    fn atom_type() -> AtomType {
        AtomType::Octet
    }
}

impl AtomTypeProvider for TgDate {
    fn atom_type() -> AtomType {
        AtomType::Date
    }
}

impl AtomTypeProvider for TgTimeOfDay {
    fn atom_type() -> AtomType {
        AtomType::TimeOfDay
    }
}

impl AtomTypeProvider for TgTimePoint {
    fn atom_type() -> AtomType {
        AtomType::TimePoint
    }
}

impl AtomTypeProvider for TgTimeOfDayWithTimeZone {
    fn atom_type() -> AtomType {
        AtomType::TimeOfDayWithTimeZone
    }
}

impl AtomTypeProvider for TgTimePointWithTimeZone {
    fn atom_type() -> AtomType {
        AtomType::TimePointWithTimeZone
    }
}

#[cfg(feature = "with_chrono")]
impl AtomTypeProvider for chrono::NaiveDate {
    fn atom_type() -> AtomType {
        AtomType::Date
    }
}

#[cfg(feature = "with_chrono")]
impl AtomTypeProvider for chrono::NaiveTime {
    fn atom_type() -> AtomType {
        AtomType::TimeOfDay
    }
}

#[cfg(feature = "with_chrono")]
impl AtomTypeProvider for chrono::NaiveDateTime {
    fn atom_type() -> AtomType {
        AtomType::TimePoint
    }
}

#[cfg(feature = "with_chrono")]
impl AtomTypeProvider for (chrono::NaiveTime, chrono::FixedOffset) {
    fn atom_type() -> AtomType {
        AtomType::TimeOfDayWithTimeZone
    }
}

#[cfg(feature = "with_chrono")]
impl<Tz: chrono::TimeZone> AtomTypeProvider for chrono::DateTime<Tz> {
    fn atom_type() -> AtomType {
        AtomType::TimePointWithTimeZone
    }
}

#[cfg(feature = "with_time")]
impl AtomTypeProvider for time::Date {
    fn atom_type() -> AtomType {
        AtomType::Date
    }
}

#[cfg(feature = "with_time")]
impl AtomTypeProvider for time::Time {
    fn atom_type() -> AtomType {
        AtomType::TimeOfDay
    }
}

#[cfg(feature = "with_time")]
impl AtomTypeProvider for time::PrimitiveDateTime {
    fn atom_type() -> AtomType {
        AtomType::TimePoint
    }
}

#[cfg(feature = "with_time")]
impl AtomTypeProvider for (time::Time, time::UtcOffset) {
    fn atom_type() -> AtomType {
        AtomType::TimeOfDayWithTimeZone
    }
}

#[cfg(feature = "with_time")]
impl AtomTypeProvider for time::OffsetDateTime {
    fn atom_type() -> AtomType {
        AtomType::TimePointWithTimeZone
    }
}

impl AtomTypeProvider for TgBlob {
    fn atom_type() -> AtomType {
        AtomType::Blob
    }
}

impl AtomTypeProvider for TgClob {
    fn atom_type() -> AtomType {
        AtomType::Clob
    }
}

/// `placeholder` method for [SqlPlaceholder].
pub trait SqlPlaceholderBind {
    /// Creates a new instance.
    fn placeholder<A: AtomTypeProvider>(self) -> SqlPlaceholder;
}

impl SqlPlaceholderBind for &str {
    fn placeholder<A: AtomTypeProvider>(self) -> SqlPlaceholder {
        let atom_type = A::atom_type();
        SqlPlaceholder::of_atom_type(self, atom_type)
    }
}

impl SqlPlaceholderBind for String {
    fn placeholder<A: AtomTypeProvider>(self) -> SqlPlaceholder {
        let atom_type = A::atom_type();
        SqlPlaceholder::of_atom_type(&self, atom_type)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bool() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Boolean);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Boolean, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<bool>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<bool>();
        assert_eq!(target0, target);
    }

    #[test]
    fn i32() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Int4);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Int4, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<i32>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<i32>();
        assert_eq!(target0, target);
    }

    #[test]
    fn i64() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Int8);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Int8, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<i64>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<i64>();
        assert_eq!(target0, target);
    }

    #[test]
    fn f32() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Float4);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Float4, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<f32>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<f32>();
        assert_eq!(target0, target);
    }

    #[test]
    fn f64() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Float8);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Float8, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<f64>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<f64>();
        assert_eq!(target0, target);
    }

    #[test]
    fn decimal() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Decimal);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Decimal, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<TgDecimal>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<TgDecimal>();
        assert_eq!(target0, target);

        let target = SqlPlaceholder::of::<TgDecimalI128>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<TgDecimalI128>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_bigdecimal")]
    #[test]
    fn bigdecimal() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Decimal);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Decimal, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<bigdecimal::BigDecimal>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<bigdecimal::BigDecimal>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_rust_decimal")]
    #[test]
    fn rust_decimal() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Decimal);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Decimal, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<rust_decimal::Decimal>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<rust_decimal::Decimal>();
        assert_eq!(target0, target);
    }

    #[test]
    fn str() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Character);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Character, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<&str>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<&str>();
        assert_eq!(target0, target);
    }

    #[test]
    fn string() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Character);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Character, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<String>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<String>();
        assert_eq!(target0, target);
    }

    #[test]
    fn array_u8() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Octet);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Octet, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<&[u8]>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<&[u8]>();
        assert_eq!(target0, target);
    }

    #[test]
    fn vec_u8() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Octet);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Octet, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<Vec<u8>>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<Vec<u8>>();
        assert_eq!(target0, target);
    }

    #[test]
    fn date() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Date);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Date, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<TgDate>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<TgDate>();
        assert_eq!(target0, target);
    }

    #[test]
    fn time_of_day() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimeOfDay);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::TimeOfDay, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<TgTimeOfDay>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<TgTimeOfDay>();
        assert_eq!(target0, target);
    }

    #[test]
    fn time_point() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimePoint);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::TimePoint, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<TgTimePoint>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<TgTimePoint>();
        assert_eq!(target0, target);
    }

    #[test]
    fn time_of_day_with_time_zone() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimeOfDayWithTimeZone);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            AtomType::TimeOfDayWithTimeZone,
            target0.atom_type().unwrap()
        );

        let target = SqlPlaceholder::of::<TgTimeOfDayWithTimeZone>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<TgTimeOfDayWithTimeZone>();
        assert_eq!(target0, target);
    }

    #[test]
    fn time_point_with_time_zone() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimePointWithTimeZone);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            AtomType::TimePointWithTimeZone,
            target0.atom_type().unwrap()
        );

        let target = SqlPlaceholder::of::<TgTimePointWithTimeZone>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<TgTimePointWithTimeZone>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_date() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Date);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Date, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<chrono::NaiveDate>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<chrono::NaiveDate>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_time() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimeOfDay);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::TimeOfDay, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<chrono::NaiveTime>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<chrono::NaiveTime>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_date_time() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimePoint);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::TimePoint, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<chrono::NaiveDateTime>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<chrono::NaiveDateTime>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_time_with_offset() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimeOfDayWithTimeZone);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            AtomType::TimeOfDayWithTimeZone,
            target0.atom_type().unwrap()
        );

        let target = SqlPlaceholder::of::<(chrono::NaiveTime, chrono::FixedOffset)>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<(chrono::NaiveTime, chrono::FixedOffset)>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_date_time() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimePointWithTimeZone);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            AtomType::TimePointWithTimeZone,
            target0.atom_type().unwrap()
        );

        let target = SqlPlaceholder::of::<chrono::DateTime<chrono::FixedOffset>>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<chrono::DateTime<chrono::FixedOffset>>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_date() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Date);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Date, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<time::Date>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<time::Date>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_time() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimeOfDay);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::TimeOfDay, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<time::Time>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<time::Time>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_primitive_date_time() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimePoint);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::TimePoint, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<time::PrimitiveDateTime>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<time::PrimitiveDateTime>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_time_with_offset() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimeOfDayWithTimeZone);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            AtomType::TimeOfDayWithTimeZone,
            target0.atom_type().unwrap()
        );

        let target = SqlPlaceholder::of::<(time::Time, time::UtcOffset)>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<(time::Time, time::UtcOffset)>();
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_offset_date_time() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::TimePointWithTimeZone);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            AtomType::TimePointWithTimeZone,
            target0.atom_type().unwrap()
        );

        let target = SqlPlaceholder::of::<time::OffsetDateTime>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<time::OffsetDateTime>();
        assert_eq!(target0, target);
    }

    #[test]
    fn blob() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Blob);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Blob, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<TgBlob>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<TgBlob>();
        assert_eq!(target0, target);
    }

    #[test]
    fn clob() {
        let target0 = SqlPlaceholder::of_atom_type("test", AtomType::Clob);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Clob, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of::<TgClob>("test");
        assert_eq!(target0, target);

        let target = "test".placeholder::<TgClob>();
        assert_eq!(target0, target);
    }
}
