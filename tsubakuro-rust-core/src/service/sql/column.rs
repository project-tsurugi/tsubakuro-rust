use crate::{jogasaki::proto::sql::common::Column as SqlColumn, prelude::AtomType};

impl SqlColumn {
    /// Get name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get AtomType.
    pub fn atom_type(&self) -> Option<AtomType> {
        match &self.type_info {
            Some(crate::jogasaki::proto::sql::common::column::TypeInfo::AtomType(atom_type)) => {
                AtomType::try_from(*atom_type).ok()
            }
            _ => None,
        }
    }

    /// Get length for data types.
    ///
    /// # Returns
    /// - `length` - defined length. Valid when `arbitrary_length` is `false`.
    /// - `arbitrary_length` - arbitrary length (*).
    ///
    /// since 0.2.0
    pub fn length(&self) -> Option<(u32, bool)> {
        use crate::jogasaki::proto::sql::common::column::LengthOpt;
        match self.length_opt {
            Some(LengthOpt::Length(length)) => Some((length, false)),
            Some(LengthOpt::ArbitraryLength(_)) => Some((0, true)),
            None => None,
        }
    }

    /// Get precision for decimal types.
    ///
    /// # Returns
    /// - `precision` - defined precision. Valid when `arbitrary_precision` is `false`.
    /// - `arbitrary_precision` - arbitrary precision (*).
    ///
    /// since 0.2.0
    pub fn precision(&self) -> Option<(u32, bool)> {
        use crate::jogasaki::proto::sql::common::column::PrecisionOpt;
        match self.precision_opt {
            Some(PrecisionOpt::Precision(precision)) => Some((precision, false)),
            Some(PrecisionOpt::ArbitraryPrecision(_)) => Some((0, true)),
            None => None,
        }
    }

    /// Get scale for decimal types.
    ///
    /// # Returns
    /// - `scale` - defined scale. Valid when `arbitrary_scale` is `false`.
    /// - `arbitrary_scale` - arbitrary scale (*).
    ///
    /// since 0.2.0
    pub fn scale(&self) -> Option<(u32, bool)> {
        use crate::jogasaki::proto::sql::common::column::ScaleOpt;
        match self.scale_opt {
            Some(ScaleOpt::Scale(scale)) => Some((scale, false)),
            Some(ScaleOpt::ArbitraryScale(_)) => Some((0, true)),
            None => None,
        }
    }

    /// Whether the column type is nullable.
    ///
    /// since 0.2.0
    pub fn nullable(&self) -> Option<bool> {
        match self.nullable_opt {
            Some(crate::jogasaki::proto::sql::common::column::NullableOpt::Nullable(nullable)) => {
                Some(nullable)
            }
            _ => None,
        }
    }

    /// Whether the column type is varying.
    ///
    /// since 0.2.0
    pub fn varying(&self) -> Option<bool> {
        match self.varying_opt {
            Some(crate::jogasaki::proto::sql::common::column::VaryingOpt::Varying(varying)) => {
                Some(varying)
            }
            _ => None,
        }
    }

    /// Returns description of the column.
    ///
    /// since 0.2.0
    pub fn description(&self) -> Option<&String> {
        use crate::jogasaki::proto::sql::common::column::DescriptionOpt;
        match &self.description_opt {
            Some(DescriptionOpt::Description(description)) => Some(description),
            _ => None,
        }
    }

    /// Returns SQL type name for the column.
    ///
    /// since 0.3.0
    pub fn sql_type_name(&self) -> Option<&'static str> {
        let atom_type = self.atom_type()?;

        match atom_type {
            AtomType::Boolean => Some("BOOLEAN"),
            AtomType::Int4 => Some("INT"),
            AtomType::Int8 => Some("BIGINT"),
            AtomType::Float4 => Some("REAL"),
            AtomType::Float8 => Some("DOUBLE"),
            AtomType::Decimal => Some("DECIMAL"),
            AtomType::Character => {
                if self.varying().unwrap_or(false) {
                    Some("VARCHAR")
                } else {
                    Some("CHAR")
                }
            }
            AtomType::Octet => {
                if self.varying().unwrap_or(false) {
                    Some("VARBINARY")
                } else {
                    Some("BINARY")
                }
            }
            AtomType::Bit => None,
            AtomType::Date => Some("DATE"),
            AtomType::TimeOfDay => Some("TIME"),
            AtomType::TimePoint => Some("TIMESTAMP"),
            AtomType::DatetimeInterval => None,
            AtomType::TimeOfDayWithTimeZone => Some("TIME WITH TIME ZONE"),
            AtomType::TimePointWithTimeZone => Some("TIMESTAMP WITH TIME ZONE"),
            AtomType::Clob => Some("CLOB"),
            AtomType::Blob => Some("BLOB"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sql_type_name_char() {
        let mut column = SqlColumn::default();
        column.type_info = Some(
            crate::jogasaki::proto::sql::common::column::TypeInfo::AtomType(
                crate::jogasaki::proto::sql::common::AtomType::Character.into(),
            ),
        );
        column.varying_opt =
            Some(crate::jogasaki::proto::sql::common::column::VaryingOpt::Varying(false));

        assert_eq!(Some("CHAR"), column.sql_type_name());
    }

    #[test]
    fn sql_type_name_varchar() {
        let mut column = SqlColumn::default();
        column.type_info = Some(
            crate::jogasaki::proto::sql::common::column::TypeInfo::AtomType(
                crate::jogasaki::proto::sql::common::AtomType::Character.into(),
            ),
        );
        column.varying_opt =
            Some(crate::jogasaki::proto::sql::common::column::VaryingOpt::Varying(true));

        assert_eq!(Some("VARCHAR"), column.sql_type_name());
    }
}
