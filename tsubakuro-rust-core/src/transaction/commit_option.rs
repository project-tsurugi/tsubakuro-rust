use crate::jogasaki::proto::sql::request::{CommitOption, CommitStatus as CommitType};

/// Commit option.
///
/// See [SqlClient::commit()](crate::prelude::SqlClient::commit).
///
/// # Examples
///
/// ## Stored
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let mut commit_option = CommitOption::new();
/// commit_option.set_commit_type(CommitType::Stored);
/// ```
///
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let commit_option = CommitOption::from(CommitType::Stored);
/// ```
///
/// ## Default
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let mut commit_option = CommitOption::new();
/// commit_option.set_commit_type(CommitType::Unspecified);
/// ```
///
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let commit_option = CommitOption::from(CommitType::Unspecified);
/// ```
///
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let transaction_option = TransactionOption::default();
/// ```
impl CommitOption {
    /// Creates a new instance.
    pub fn new() -> CommitOption {
        CommitOption {
            notification_type: CommitType::Unspecified.into(),
            auto_dispose: false,
        }
    }

    /// Set commit type.
    pub fn set_commit_type(&mut self, commit_type: CommitType) {
        self.notification_type = commit_type.into();
    }

    /// Get commit type.
    pub fn commit_type(&self) -> CommitType {
        self.notification_type()
    }

    /// Set auto dispose.
    pub fn set_auto_dispose(&mut self, auto_dispose: bool) {
        self.auto_dispose = auto_dispose;
    }

    /// Get auto dispose.
    pub fn auto_dispose(&self) -> bool {
        self.auto_dispose
    }
}

impl From<CommitType> for CommitOption {
    fn from(value: CommitType) -> Self {
        let mut option = CommitOption::new();
        option.set_commit_type(value);
        option
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn commit_option() {
        {
            let option = CommitOption::new();
            assert_eq!(CommitType::Unspecified, option.commit_type());
            assert_eq!(false, option.auto_dispose());
        }
        {
            let mut option = CommitOption::new();
            option.set_commit_type(CommitType::Stored);
            option.set_auto_dispose(true);

            assert_eq!(CommitType::Stored, option.commit_type());
            assert_eq!(true, option.auto_dispose());
        }
    }

    #[test]
    fn commit_option_default() {
        let option = CommitOption::default();
        assert_eq!(CommitType::Unspecified, option.commit_type());
        assert_eq!(false, option.auto_dispose());
    }

    #[test]
    fn commit_option_from() {
        let option = CommitOption::from(CommitType::Stored);
        assert_eq!(CommitType::Stored, option.commit_type());
        assert_eq!(false, option.auto_dispose());
    }
}
