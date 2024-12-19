use crate::jogasaki::proto::sql::common::AtomType;
use crate::jogasaki::proto::sql::request::placeholder::{Placement, TypeInfo};
use crate::jogasaki::proto::sql::request::Placeholder as SqlPlaceholder;

pub struct SqlPlaceholderUtil;

impl SqlPlaceholderUtil {
    pub fn from_atom_type(name: &str, atom_type: AtomType) -> SqlPlaceholder {
        let placement = Placement::Name(name.to_string());
        let type_info = TypeInfo::AtomType(atom_type.into());

        SqlPlaceholder {
            dimension: 0,
            placement: Some(placement),
            type_info: Some(type_info),
        }
    }
}

pub trait SqlPlaceholderGenerator {
    fn placeholder(name: &str) -> SqlPlaceholder;
}

impl SqlPlaceholderGenerator for i32 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholderUtil::from_atom_type(name, AtomType::Int4)
    }
}

impl SqlPlaceholderGenerator for i64 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholderUtil::from_atom_type(name, AtomType::Int8)
    }
}

impl SqlPlaceholderGenerator for f32 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholderUtil::from_atom_type(name, AtomType::Float4)
    }
}

impl SqlPlaceholderGenerator for f64 {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholderUtil::from_atom_type(name, AtomType::Float8)
    }
}

impl SqlPlaceholderGenerator for str {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholderUtil::from_atom_type(name, AtomType::Character)
    }
}

impl SqlPlaceholderGenerator for String {
    fn placeholder(name: &str) -> SqlPlaceholder {
        SqlPlaceholderUtil::from_atom_type(name, AtomType::Character)
    }
}

pub trait SqlPlaceholderInfo {
    fn name(&self) -> Option<&String>;
    fn atom_type(&self) -> Option<AtomType>;
}

impl SqlPlaceholderInfo for SqlPlaceholder {
    fn name(&self) -> Option<&String> {
        if let Some(Placement::Name(ref name)) = self.placement {
            Some(name)
        } else {
            None
        }
    }

    fn atom_type(&self) -> Option<AtomType> {
        if let Some(TypeInfo::AtomType(atom_type)) = self.type_info {
            AtomType::try_from(atom_type).ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn i32() {
        let target = i32::placeholder("test");
        assert_eq!("test", target.name().unwrap());
        assert_eq!(AtomType::Int4, target.atom_type().unwrap());
    }

    #[test]
    fn i64() {
        let target = i64::placeholder("test");
        assert_eq!("test", target.name().unwrap());
        assert_eq!(AtomType::Int8, target.atom_type().unwrap());
    }

    #[test]
    fn f32() {
        let target = f32::placeholder("test");
        assert_eq!("test", target.name().unwrap());
        assert_eq!(AtomType::Float4, target.atom_type().unwrap());
    }

    #[test]
    fn f64() {
        let target = f64::placeholder("test");
        assert_eq!("test", target.name().unwrap());
        assert_eq!(AtomType::Float8, target.atom_type().unwrap());
    }

    #[test]
    fn str() {
        let target = str::placeholder("test");
        assert_eq!("test", target.name().unwrap());
        assert_eq!(AtomType::Character, target.atom_type().unwrap());
    }

    #[test]
    fn string() {
        let target = String::placeholder("test");
        assert_eq!("test", target.name().unwrap());
        assert_eq!(AtomType::Character, target.atom_type().unwrap());
    }
}
