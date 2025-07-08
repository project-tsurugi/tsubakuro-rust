/// return value from functions
#[repr(i16)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SqlReturn {
    SQL_SUCCESS = 0,
    SQL_SUCCESS_WITH_INFO = 1,
    SQL_NO_DATA = 100,

    SQL_PARAM_DATA_AVAILABLE = 101,

    SQL_ERROR = -1,
    SQL_INVALID_HANDLE = -2,

    SQL_STILL_EXECUTING = 2,
    SQL_NEED_DATA = 99,
}

impl std::fmt::Debug for SqlReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SqlReturn::*;
        match self {
            SQL_SUCCESS => write!(f, "SQL_SUCCESS(0)"),
            SQL_SUCCESS_WITH_INFO => write!(f, "SQL_SUCCESS_WITH_INFO(1)"),
            SQL_NO_DATA => write!(f, "SQL_NO_DATA(100)"),
            SQL_PARAM_DATA_AVAILABLE => write!(f, "SQL_PARAM_DATA_AVAILABLE(101)"),
            SQL_ERROR => write!(f, "SQL_ERROR(-1)"),
            SQL_INVALID_HANDLE => write!(f, "SQL_INVALID_HANDLE(-2)"),
            SQL_STILL_EXECUTING => write!(f, "SQL_STILL_EXECUTING(2)"),
            SQL_NEED_DATA => write!(f, "SQL_NEED_DATA(99)"),
        }
    }
}

impl SqlReturn {
    pub fn is_success(&self) -> bool {
        matches!(
            self,
            SqlReturn::SQL_SUCCESS | SqlReturn::SQL_SUCCESS_WITH_INFO
        )
    }

    pub fn or(&self, other: SqlReturn) -> SqlReturn {
        match self {
            SqlReturn::SQL_SUCCESS => other,
            SqlReturn::SQL_SUCCESS_WITH_INFO => {
                if other.is_success() {
                    *self
                } else {
                    other
                }
            }
            rc => *rc,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sql_return_or() {
        let s = SqlReturn::SQL_SUCCESS;
        let i = SqlReturn::SQL_SUCCESS_WITH_INFO;
        let e = SqlReturn::SQL_ERROR;
        let f = SqlReturn::SQL_INVALID_HANDLE;

        assert_eq!(s, s.or(s));
        assert_eq!(i, s.or(i));
        assert_eq!(e, s.or(e));
        assert_eq!(f, s.or(f));

        assert_eq!(i, i.or(s));
        assert_eq!(i, i.or(i));
        assert_eq!(e, i.or(e));
        assert_eq!(f, i.or(f));

        assert_eq!(e, e.or(s));
        assert_eq!(e, e.or(i));
        assert_eq!(e, e.or(e));
        assert_eq!(e, e.or(f));

        assert_eq!(f, f.or(s));
        assert_eq!(f, f.or(i));
        assert_eq!(f, f.or(e));
        assert_eq!(f, f.or(f));
    }
}
