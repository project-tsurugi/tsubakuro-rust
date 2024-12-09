use crate::jogasaki::proto::sql::response::Name as ProtoName;

use std::fmt::{Debug, Display};

pub struct TName {
    identfiers: Vec<String>,
}

impl TName {
    pub fn from(proto_name: &ProtoName) -> TName {
        let identifiers = proto_name
            .identifiers
            .iter()
            .map(|identifier| identifier.label.to_string())
            .collect();
        TName {
            identfiers: identifiers,
        }
    }

    pub fn identifiers(&self) -> &Vec<String> {
        &self.identfiers
    }
}

impl Display for TName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.identfiers.join(".");
        write!(f, "{}", s)
    }
}

impl Debug for TName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.to_string();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl TName {
        fn new(identifiers: Vec<String>) -> TName {
            TName {
                identfiers: identifiers,
            }
        }
    }

    #[test]
    fn tname_identifiers() {
        let name = TName::new(vec![
            String::from("scheme1"),
            String::from("database1"),
            String::from("table1"),
        ]);

        let i = name.identifiers();
        assert_eq!(3, i.len());
        assert_eq!(String::from("scheme1"), i[0]);
        assert_eq!(String::from("database1"), i[1]);
        assert_eq!(String::from("table1"), i[2]);
    }

    #[test]
    fn tname_to_string1() {
        let name = TName::new(vec![String::from("table1")]);
        assert_eq!("table1", name.to_string());
    }

    #[test]
    fn tname_to_string3() {
        let name = TName::new(vec![
            String::from("scheme1"),
            String::from("database1"),
            String::from("table1"),
        ]);
        assert_eq!("scheme1.database1.table1", name.to_string());
    }
}
