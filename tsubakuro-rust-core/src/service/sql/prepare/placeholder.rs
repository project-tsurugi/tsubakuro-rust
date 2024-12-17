use crate::jogasaki::proto::sql::common::AtomType;
use crate::jogasaki::proto::sql::request::placeholder::{Placement, TypeInfo};
use crate::jogasaki::proto::sql::request::Placeholder;

#[derive(Debug)]
pub struct SqlPlaceholder {
    name: String,
    atom_type: Option<AtomType>,
    dimension: u32,
}

impl SqlPlaceholder {
    pub fn from_atom_type(name: &str, atom_type: AtomType) -> SqlPlaceholder {
        SqlPlaceholder {
            name: name.to_string(),
            atom_type: Some(atom_type),
            dimension: 0,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn atom_type(&self) -> Option<AtomType> {
        self.atom_type
    }
}

pub trait SqlPlaceholderGenerator {
    fn placeholder(name: &str) -> SqlPlaceholder;
}

impl SqlPlaceholderGenerator for i32 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::from_atom_type(name, AtomType::Int4)
    }
}

impl SqlPlaceholderGenerator for i64 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::from_atom_type(name, AtomType::Int8)
    }
}

impl SqlPlaceholderGenerator for f32 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::from_atom_type(name, AtomType::Float4)
    }
}

impl SqlPlaceholderGenerator for f64 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::from_atom_type(name, AtomType::Float8)
    }
}

impl SqlPlaceholderGenerator for str {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::from_atom_type(name, AtomType::Character)
    }
}

impl SqlPlaceholderGenerator for String {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholder::from_atom_type(name, AtomType::Character)
    }
}

impl SqlPlaceholder {
    pub(crate) fn request(&self) -> Placeholder {
        let placement = Placement::Name(self.name.clone());
        let type_info = {
            if let Some(atom_type) = self.atom_type {
                Some(TypeInfo::AtomType(atom_type.into()))
            } else {
                todo!()
            }
        };

        Placeholder {
            dimension: self.dimension,
            placement: Some(placement),
            type_info,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn i32() {
        let target = i32::placeholder("test");
        assert_eq!("test", target.name());
        assert_eq!(AtomType::Int4, target.atom_type().unwrap());
    }

    #[test]
    fn i64() {
        let target = i64::placeholder("test");
        assert_eq!("test", target.name());
        assert_eq!(AtomType::Int8, target.atom_type().unwrap());
    }

    #[test]
    fn f32() {
        let target = f32::placeholder("test");
        assert_eq!("test", target.name());
        assert_eq!(AtomType::Float4, target.atom_type().unwrap());
    }

    #[test]
    fn f64() {
        let target = f64::placeholder("test");
        assert_eq!("test", target.name());
        assert_eq!(AtomType::Float8, target.atom_type().unwrap());
    }

    #[test]
    fn str() {
        let target = str::placeholder("test");
        assert_eq!("test", target.name());
        assert_eq!(AtomType::Character, target.atom_type().unwrap());
    }

    #[test]
    fn string() {
        let target = String::placeholder("test");
        assert_eq!("test", target.name());
        assert_eq!(AtomType::Character, target.atom_type().unwrap());
    }
}
