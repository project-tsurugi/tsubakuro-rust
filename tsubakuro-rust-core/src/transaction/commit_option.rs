use crate::jogasaki::proto::sql::request::CommitStatus as CommitType;

#[derive(Debug, Clone)]
pub struct CommitOption {
    commit_type: CommitType,
    auto_dispose: bool,
}

impl CommitOption {
    pub fn new() -> CommitOption {
        CommitOption {
            commit_type: CommitType::Unspecified,
            auto_dispose: false,
        }
    }

    pub fn set_commit_type(&mut self, commit_type: CommitType) {
        self.commit_type = commit_type;
    }

    pub fn commit_type(&self) -> CommitType {
        self.commit_type
    }

    pub fn set_auto_dispose(&mut self, auto_dispose: bool) {
        self.auto_dispose = auto_dispose;
    }

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
}
