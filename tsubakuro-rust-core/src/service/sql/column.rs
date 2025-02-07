use crate::{jogasaki::proto::sql::common::Column as SqlColumn, prelude::AtomType};

impl SqlColumn {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn atom_type(&self) -> Option<AtomType> {
        match &self.type_info {
            Some(crate::jogasaki::proto::sql::common::column::TypeInfo::AtomType(atom_type)) => {
                AtomType::try_from(*atom_type).ok()
            }
            _ => None,
        }
    }
}
