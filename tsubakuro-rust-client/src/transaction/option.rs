use crate::jogasaki::proto::sql::request::TransactionOption as TransactionOptionReuest;
use crate::jogasaki::proto::sql::request::TransactionPriority as TransactionPriorityCase;
use crate::jogasaki::proto::sql::request::TransactionType as TransactionTypeCase;
use crate::util::string_to_prost_string;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum TransactionType {
    /// short transactions (optimistic concurrency control)
    Occ = 1,
    /// long transactions (pessimistic concurrency control)
    Ltx = 2,
    /// read only transactions (may be abort-free)
    Rtx = 3,
}

#[derive(Debug, Clone)]
pub struct TransactionOption {
    transaction_type: TransactionType,
    transaction_label: Option<String>,
}

impl TransactionOption {
    pub fn new() -> TransactionOption {
        TransactionOption {
            transaction_type: TransactionType::Occ,
            transaction_label: None,
        }
    }

    pub fn set_transaction_type(&mut self, transaction_type: TransactionType) {
        self.transaction_type = transaction_type;
    }

    pub fn transaction_type(&self) -> &TransactionType {
        &self.transaction_type
    }

    pub fn set_transaction_label<T: Into<Option<String>>>(&mut self, transaction_label: T) {
        self.transaction_label = transaction_label.into();
    }

    pub fn transaction_label(&self) -> Option<&String> {
        self.transaction_label.as_ref()
    }

    pub(crate) fn as_request(&self) -> TransactionOptionReuest {
        let tx_type = match self.transaction_type {
            TransactionType::Occ => TransactionTypeCase::Short,
            TransactionType::Ltx => TransactionTypeCase::Long,
            TransactionType::Rtx => TransactionTypeCase::ReadOnly,
        };
        let tx_label = self.transaction_label.as_ref();

        TransactionOptionReuest {
            r#type: tx_type.into(),
            priority: TransactionPriorityCase::Unspecified.into(),
            label: string_to_prost_string(tx_label),
            modifies_definitions: false,
            write_preserves: vec![],
            inclusive_read_areas: vec![],
            exclusive_read_areas: vec![],
        }
    }
}

impl From<TransactionType> for TransactionOption {
    fn from(value: TransactionType) -> Self {
        let mut option = TransactionOption::new();
        option.set_transaction_type(value);
        option
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum CommitType {
    /// the default commit type (rely on the database settings).
    Default = 0,
    /// commit operation has accepted, and the transaction will never abort except system errors.
    Accepted = 10,
    /// commit data has been visible for others.
    Available = 20,
    /// commit data has been saved on the local disk.
    Stored = 30,
    /// commit data has been propagated to the all suitable nodes.
    Propagated = 40,
}

#[derive(Debug, Clone)]
pub struct CommitOption {
    commit_type: CommitType,
    auto_dispose: bool,
}

impl CommitOption {
    pub fn new() -> CommitOption {
        CommitOption {
            commit_type: CommitType::Default,
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
