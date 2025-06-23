use crate::jogasaki::proto::sql::response::Name as ProtoName;

/// Table name.
#[derive(Clone, PartialEq)]
pub struct TName {
    identifiers: Vec<String>,
}

impl TName {
    /// Creates a new instance.
    pub fn new(identifiers: Vec<String>) -> TName {
        TName { identifiers }
    }

    pub(crate) fn from(proto_name: &ProtoName) -> TName {
        let identifiers = proto_name
            .identifiers
            .iter()
            .map(|identifier| identifier.label.to_string())
            .collect();
        TName { identifiers }
    }

    /// Get identifiers.
    pub fn identifiers(&self) -> &Vec<String> {
        &self.identifiers
    }

    /// Get database name.
    ///
    /// since 0.3.0
    pub fn database_name(&self) -> Option<&String> {
        let index = self.identifiers.len() as isize - 1 - 2;
        if index < 0 {
            return None;
        }
        self.identifiers.get(index as usize)
    }

    /// Get schema name.
    ///
    /// since 0.3.0
    pub fn schema_name(&self) -> Option<&String> {
        let index = self.identifiers.len() as isize - 1 - 1;
        if index < 0 {
            return None;
        }
        self.identifiers.get(index as usize)
    }

    /// Get last name.
    ///
    /// since 0.3.0
    pub fn last_name(&self) -> Option<&String> {
        self.identifiers.last()
    }
}

impl std::fmt::Display for TName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.identifiers.join(".");
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for TName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tname_identifiers() {
        let name = TName::new(vec![
            String::from("database1"),
            String::from("schema1"),
            String::from("table1"),
        ]);

        let i = name.identifiers();
        assert_eq!(3, i.len());
        assert_eq!(String::from("database1"), i[0]);
        assert_eq!(String::from("schema1"), i[1]);
        assert_eq!(String::from("table1"), i[2]);

        assert_eq!(Some(&String::from("database1")), name.database_name());
        assert_eq!(Some(&String::from("schema1")), name.schema_name());
        assert_eq!(Some(&String::from("table1")), name.last_name());
    }

    #[test]
    fn tname_to_string1() {
        let name = TName::new(vec![String::from("table1")]);
        assert_eq!("table1", name.to_string());

        assert_eq!(None, name.database_name());
        assert_eq!(None, name.schema_name());
        assert_eq!(Some(&String::from("table1")), name.last_name());
    }

    #[test]
    fn tname_to_string3() {
        let name = TName::new(vec![
            String::from("database1"),
            String::from("schema1"),
            String::from("table1"),
        ]);
        assert_eq!("database1.schema1.table1", name.to_string());
    }
}
