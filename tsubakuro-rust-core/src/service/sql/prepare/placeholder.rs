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

pub trait SqlPlaceholderOf<T> {
    fn of(name: &str, r#type: T) -> SqlPlaceholder;
}

impl SqlPlaceholderOf<AtomType> for SqlPlaceholder {
    fn of(name: &str, atom_type: AtomType) -> SqlPlaceholder {
        let type_info = TypeInfo::AtomType(atom_type.into());
        SqlPlaceholder::new(name, type_info, 0)
    }
}

impl SqlPlaceholder {
    pub fn of_type<T: AtomTypeProvider>(name: &str) -> Self {
        let atom_type = T::atom_type();
        SqlPlaceholder::of(name, atom_type)
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

pub trait SqlPlaceholderGenerator {
    fn placeholder(name: &str) -> SqlPlaceholder;
}

impl SqlPlaceholderGenerator for i32 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::of(name, AtomType::Int4)
    }
}

impl SqlPlaceholderGenerator for i64 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::of(name, AtomType::Int8)
    }
}

impl SqlPlaceholderGenerator for f32 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::of(name, AtomType::Float4)
    }
}

impl SqlPlaceholderGenerator for f64 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::of(name, AtomType::Float8)
    }
}

impl SqlPlaceholderGenerator for str {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::of(name, AtomType::Character)
    }
}

impl SqlPlaceholderGenerator for String {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::of(name, AtomType::Character)
    }
}

impl SqlPlaceholder {
    pub fn placeholder_for<T: AtomTypeProvider>(name: &str) -> Self {
        let atom_type = T::atom_type();
        SqlPlaceholder::of(name, atom_type)
    }
}

pub trait SqlPlaceholderFrom {
    fn placeholder_from<A: AtomTypeProvider>(self) -> SqlPlaceholder;
}

impl SqlPlaceholderFrom for &str {
    fn placeholder_from<A: AtomTypeProvider>(self) -> SqlPlaceholder {
        let atom_type = A::atom_type();
        SqlPlaceholder::of(self, atom_type)
    }
}

impl SqlPlaceholderFrom for String {
    fn placeholder_from<A: AtomTypeProvider>(self) -> SqlPlaceholder {
        let atom_type = A::atom_type();
        SqlPlaceholder::of(&self, atom_type)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn i32() {
        let target0 = SqlPlaceholder::of("test", AtomType::Int4);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Int4, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of_type::<i32>("test");
        assert_eq!(target0, target);

        let target = i32::placeholder("test");
        assert_eq!(target0, target);

        let target = "test".placeholder_from::<i32>();
        assert_eq!(target0, target);
    }

    #[test]
    fn i64() {
        let target0 = SqlPlaceholder::of("test", AtomType::Int8);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Int8, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of_type::<i64>("test");
        assert_eq!(target0, target);

        let target = i64::placeholder("test");
        assert_eq!(target0, target);

        let target = "test".placeholder_from::<i64>();
        assert_eq!(target0, target);
    }

    #[test]
    fn f32() {
        let target0 = SqlPlaceholder::of("test", AtomType::Float4);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Float4, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of_type::<f32>("test");
        assert_eq!(target0, target);

        let target = f32::placeholder("test");
        assert_eq!(target0, target);

        let target = "test".placeholder_from::<f32>();
        assert_eq!(target0, target);
    }

    #[test]
    fn f64() {
        let target0 = SqlPlaceholder::of("test", AtomType::Float8);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Float8, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of_type::<f64>("test");
        assert_eq!(target0, target);

        let target = f64::placeholder("test");
        assert_eq!(target0, target);

        let target = "test".placeholder_from::<f64>();
        assert_eq!(target0, target);
    }

    #[test]
    fn str() {
        let target0 = SqlPlaceholder::of("test", AtomType::Character);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Character, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of_type::<&str>("test");
        assert_eq!(target0, target);

        let target = str::placeholder("test");
        assert_eq!(target0, target);

        let target = "test".placeholder_from::<&str>();
        assert_eq!(target0, target);
    }

    #[test]
    fn string() {
        let target0 = SqlPlaceholder::of("test", AtomType::Character);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(AtomType::Character, target0.atom_type().unwrap());

        let target = SqlPlaceholder::of_type::<String>("test");
        assert_eq!(target0, target);

        let target = String::placeholder("test");
        assert_eq!(target0, target);

        let target = "test".placeholder_from::<String>();
        assert_eq!(target0, target);
    }
}
