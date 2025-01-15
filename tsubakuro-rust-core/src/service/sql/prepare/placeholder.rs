use crate::jogasaki::proto::sql::common::AtomType;
use crate::jogasaki::proto::sql::request::placeholder::{Placement, TypeInfo};
use crate::jogasaki::proto::sql::request::Placeholder as SqlPlaceholder;

impl SqlPlaceholder {
    fn new(name: &str, type_info: TypeInfo, dimension: u32) -> SqlPlaceholder {
        let placement = Placement::Name(name.to_string());

        SqlPlaceholder {
            dimension,
            placement: Some(placement),
            type_info: Some(type_info),
        }
    }

    pub fn name(&self) -> Option<&String> {
        match self.placement {
            Some(Placement::Name(ref name)) => Some(name),
            _ => None,
        }
    }

    pub fn atom_type(&self) -> Option<AtomType> {
        match self.type_info {
            Some(TypeInfo::AtomType(atom_type)) => AtomType::try_from(atom_type).ok(),
            _ => None,
        }
    }
}

impl SqlPlaceholder {
    pub fn of_atom_type(name: &str, atom_type: AtomType) -> SqlPlaceholder {
        let type_info = TypeInfo::AtomType(atom_type.into());
        SqlPlaceholder::new(name, type_info, 0)
    }

    pub fn of<T: AtomTypeProvider>(name: &str) -> Self {
        let atom_type = T::atom_type();
        SqlPlaceholder::of_atom_type(name, atom_type)
    }
}

pub trait AtomTypeProvider {
    fn atom_type() -> AtomType;
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

#[cfg(feature = "with_bigdecimal")]
impl AtomTypeProvider for bigdecimal::BigDecimal {
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

pub trait SqlPlaceholderBind {
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
}
