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
    /// # Returns
    /// For example, `varchar(10)` returns `VARCHAR`.
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

    /// Returns SQL type for the column.
    ///
    /// # Returns
    /// For example, `varchar(10)` returns `VARCHAR(10)`.
    ///
    /// since 0.3.0
    pub fn sql_type(&self) -> Option<String> {
        let base_name = self.sql_type_name()?;
        let atom_type = self.atom_type()?;

        match atom_type {
            AtomType::Decimal => match self.precision() {
                Some((precision, arbitrary)) => match self.scale() {
                    Some((scale, scale_arbitrary)) =>
                    {
                        #[allow(clippy::collapsible_else_if)]
                        if arbitrary {
                            if scale_arbitrary {
                                Some(format!("{base_name}(*, *)"))
                            } else {
                                Some(format!("{base_name}(*, {scale})"))
                            }
                        } else {
                            if scale_arbitrary {
                                Some(format!("{base_name}({precision}, *)"))
                            } else {
                                Some(format!("{base_name}({precision}, {scale})"))
                            }
                        }
                    }
                    None => {
                        if arbitrary {
                            Some(format!("{base_name}(*)"))
                        } else {
                            Some(format!("{base_name}({precision})"))
                        }
                    }
                },
                None => Some(base_name.to_string()),
            },
            AtomType::Character | AtomType::Octet => match self.length() {
                Some((length, arbitrary)) => {
                    if arbitrary {
                        Some(format!("{base_name}(*)"))
                    } else {
                        Some(format!("{base_name}({length})"))
                    }
                }
                None => Some(base_name.to_string()),
            },
            _ => Some(base_name.to_string()),
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

    #[test]
    fn sql_type_char() {
        let mut column = SqlColumn::default();
        column.type_info = Some(
            crate::jogasaki::proto::sql::common::column::TypeInfo::AtomType(
                crate::jogasaki::proto::sql::common::AtomType::Character.into(),
            ),
        );
        column.varying_opt =
            Some(crate::jogasaki::proto::sql::common::column::VaryingOpt::Varying(false));
        assert_eq!(Some("CHAR".to_string()), column.sql_type());

        {
            column.length_opt =
                Some(crate::jogasaki::proto::sql::common::column::LengthOpt::Length(123));

            assert_eq!(Some("CHAR(123)".to_string()), column.sql_type());
        }
        {
            column.length_opt =
                Some(crate::jogasaki::proto::sql::common::column::LengthOpt::ArbitraryLength(()));

            assert_eq!(Some("CHAR(*)".to_string()), column.sql_type());
        }
    }
    #[test]
    fn sql_type_decimal() {
        let mut column = SqlColumn::default();
        column.type_info = Some(
            crate::jogasaki::proto::sql::common::column::TypeInfo::AtomType(
                crate::jogasaki::proto::sql::common::AtomType::Decimal.into(),
            ),
        );
        assert_eq!(Some("DECIMAL".to_string()), column.sql_type());

        {
            column.precision_opt =
                Some(crate::jogasaki::proto::sql::common::column::PrecisionOpt::Precision(15));

            assert_eq!(Some("DECIMAL(15)".to_string()), column.sql_type());

            {
                column.scale_opt =
                    Some(crate::jogasaki::proto::sql::common::column::ScaleOpt::Scale(3));

                assert_eq!(Some("DECIMAL(15, 3)".to_string()), column.sql_type());
            }
            {
                column.scale_opt =
                    Some(crate::jogasaki::proto::sql::common::column::ScaleOpt::ArbitraryScale(()));

                assert_eq!(Some("DECIMAL(15, *)".to_string()), column.sql_type());
            }
        }
        {
            column.precision_opt = Some(
                crate::jogasaki::proto::sql::common::column::PrecisionOpt::ArbitraryPrecision(()),
            );
            column.scale_opt = None;

            assert_eq!(Some("DECIMAL(*)".to_string()), column.sql_type());

            {
                column.scale_opt =
                    Some(crate::jogasaki::proto::sql::common::column::ScaleOpt::Scale(3));

                assert_eq!(Some("DECIMAL(*, 3)".to_string()), column.sql_type());
            }
            {
                column.scale_opt =
                    Some(crate::jogasaki::proto::sql::common::column::ScaleOpt::ArbitraryScale(()));

                assert_eq!(Some("DECIMAL(*, *)".to_string()), column.sql_type());
            }
        }
    }
}
