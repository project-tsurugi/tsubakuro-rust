use std::time::Duration;

use crate::jogasaki::proto::sql::request::ReadArea;
use crate::jogasaki::proto::sql::request::TransactionOption as RequestTransactionOption;
use crate::jogasaki::proto::sql::request::TransactionPriority as RequestTransactionPriority;
use crate::jogasaki::proto::sql::request::TransactionType as RequestTransactionType;
use crate::jogasaki::proto::sql::request::WritePreserve;
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

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i32)]
pub enum TransactionPriority {
    /// use default transaction priority.
    Unspecified = 0,
    /// halts the running transactions immediately.
    Interrupt = 1,
    /// prevents new transactions and waits for the running transactions will end.
    Wait = 2,
    /// halts the running transactions immediately, and keep lock-out until its end.
    InterruptExclude = 3,
    /// prevents new transactions and waits for the running transactions will end, and keep lock-out until its end.
    WaitExclude = 4,
}

#[derive(Debug, Clone)]
pub struct TransactionOption {
    transaction_type: TransactionType,
    transaction_label: Option<String>,
    modifies_definitions: bool,
    write_preserve: Vec<String>,
    inclusive_read_area: Vec<String>,
    exclusive_read_area: Vec<String>,
    priority: TransactionPriority,
    close_timeout: Option<Duration>,
}

impl TransactionOption {
    pub fn new() -> TransactionOption {
        TransactionOption {
            transaction_type: TransactionType::Occ,
            transaction_label: None,
            modifies_definitions: false,
            write_preserve: vec![],
            inclusive_read_area: vec![],
            exclusive_read_area: vec![],
            priority: TransactionPriority::Unspecified,
            close_timeout: None,
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

impl TransactionOption {
    pub fn set_transaction_type(&mut self, transaction_type: TransactionType) {
        self.transaction_type = transaction_type;
    }

    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type
    }

    pub fn transaction_label(&self) -> Option<&String> {
        self.transaction_label.as_ref()
    }

    pub fn set_modifies_definitions(&mut self, modifies_definitions: bool) {
        self.modifies_definitions = modifies_definitions;
    }

    pub fn modifies_definitions(&self) -> bool {
        self.modifies_definitions
    }

    pub fn write_preserve(&self) -> &Vec<String> {
        &self.write_preserve
    }

    pub fn inclusive_read_area(&self) -> &Vec<String> {
        &self.inclusive_read_area
    }

    pub fn exclusive_read_area(&self) -> &Vec<String> {
        &self.exclusive_read_area
    }

    pub fn set_priority(&mut self, priority: TransactionPriority) {
        self.priority = priority;
    }

    pub fn priority(&self) -> TransactionPriority {
        self.priority
    }

    pub fn set_close_timeout(&mut self, timeout: Duration) {
        self.close_timeout = Some(timeout);
    }

    pub fn close_timeout(&self) -> Option<Duration> {
        self.close_timeout
    }
}

pub trait TransactionOptionSetter<T> {
    fn set_transaction_label(&mut self, transaction_label: T);
    fn set_write_preserve(&mut self, table_names: &Vec<T>);
    fn set_inclusive_read_area(&mut self, table_names: &Vec<T>);
    fn set_exclusive_read_area(&mut self, table_names: &Vec<T>);
}

impl TransactionOptionSetter<&str> for TransactionOption {
    fn set_transaction_label(&mut self, transaction_label: &str) {
        self.transaction_label = Some(transaction_label.to_string());
    }

    fn set_write_preserve(&mut self, table_names: &Vec<&str>) {
        self.write_preserve = table_names.iter().map(|s| s.to_string()).collect()
    }

    fn set_inclusive_read_area(&mut self, table_names: &Vec<&str>) {
        self.inclusive_read_area = table_names.iter().map(|s| s.to_string()).collect()
    }

    fn set_exclusive_read_area(&mut self, table_names: &Vec<&str>) {
        self.exclusive_read_area = table_names.iter().map(|s| s.to_string()).collect()
    }
}

impl TransactionOptionSetter<String> for TransactionOption {
    fn set_transaction_label(&mut self, transaction_label: String) {
        self.transaction_label = Some(transaction_label);
    }

    fn set_write_preserve(&mut self, table_names: &Vec<String>) {
        self.write_preserve = table_names.iter().map(|s| s.to_string()).collect()
    }

    fn set_inclusive_read_area(&mut self, table_names: &Vec<String>) {
        self.inclusive_read_area = table_names.iter().map(|s| s.to_string()).collect()
    }

    fn set_exclusive_read_area(&mut self, table_names: &Vec<String>) {
        self.exclusive_read_area = table_names.iter().map(|s| s.to_string()).collect()
    }
}

impl TransactionOption {
    pub(crate) fn as_request(&self) -> RequestTransactionOption {
        let tx_type = match self.transaction_type {
            TransactionType::Occ => RequestTransactionType::Short,
            TransactionType::Ltx => RequestTransactionType::Long,
            TransactionType::Rtx => RequestTransactionType::ReadOnly,
        };
        let tx_priority = match self.priority {
            TransactionPriority::Unspecified => RequestTransactionPriority::Unspecified,
            TransactionPriority::Interrupt => RequestTransactionPriority::Interrupt,
            TransactionPriority::Wait => RequestTransactionPriority::Wait,
            TransactionPriority::InterruptExclude => RequestTransactionPriority::InterruptExclude,
            TransactionPriority::WaitExclude => RequestTransactionPriority::WaitExclude,
        };
        let tx_label = self.transaction_label.as_ref();

        RequestTransactionOption {
            r#type: tx_type.into(),
            priority: tx_priority.into(),
            label: string_to_prost_string(tx_label),
            modifies_definitions: self.modifies_definitions,
            write_preserves: Self::to_write_preserve(&self.write_preserve),
            inclusive_read_areas: Self::to_read_area(&self.inclusive_read_area),
            exclusive_read_areas: Self::to_read_area(&self.exclusive_read_area),
        }
    }

    fn to_write_preserve(table_names: &Vec<String>) -> Vec<WritePreserve> {
        table_names
            .iter()
            .map(|s| WritePreserve {
                table_name: s.clone(),
            })
            .collect()
    }

    fn to_read_area(table_names: &Vec<String>) -> Vec<ReadArea> {
        table_names
            .iter()
            .map(|s| ReadArea {
                table_name: s.clone(),
            })
            .collect()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transaction_option() {
        {
            let option = TransactionOption::new();
            assert_eq!(TransactionType::Occ, option.transaction_type());
            assert_eq!(None, option.transaction_label());
            assert_eq!(false, option.modifies_definitions());
            assert_eq!(true, option.write_preserve().is_empty());
            assert_eq!(true, option.inclusive_read_area().is_empty());
            assert_eq!(true, option.exclusive_read_area().is_empty());
            assert_eq!(TransactionPriority::Unspecified, option.priority());
            assert_eq!(None, option.close_timeout());

            let request = option.as_request();
            assert_eq!(RequestTransactionType::Short, request.r#type());
            assert_eq!(RequestTransactionPriority::Unspecified, request.priority());
            assert_eq!("", request.label);
            assert_eq!(false, request.modifies_definitions);
            assert_eq!(true, request.write_preserves.is_empty());
            assert_eq!(true, request.inclusive_read_areas.is_empty());
            assert_eq!(true, request.exclusive_read_areas.is_empty());
        }
        {
            let mut option = TransactionOption::new();
            option.set_transaction_type(TransactionType::Ltx);
            option.set_transaction_label("transaction_label");
            option.set_modifies_definitions(true);
            option.set_write_preserve(&vec!["wp"]);
            option.set_inclusive_read_area(&vec!["r1"]);
            option.set_exclusive_read_area(&vec!["r2"]);
            option.set_priority(TransactionPriority::Interrupt);
            option.set_close_timeout(Duration::from_secs(123));

            let request = option.as_request();
            assert_eq!(RequestTransactionType::Long, request.r#type());
            assert_eq!(RequestTransactionPriority::Interrupt, request.priority());
            assert_eq!("transaction_label", request.label);
            assert_eq!(true, request.modifies_definitions);
            assert_eq!(
                vec![WritePreserve {
                    table_name: "wp".to_string()
                }],
                request.write_preserves
            );
            assert_eq!(
                vec![ReadArea {
                    table_name: "r1".to_string()
                }],
                request.inclusive_read_areas
            );
            assert_eq!(
                vec![ReadArea {
                    table_name: "r2".to_string()
                }],
                request.exclusive_read_areas
            );
            assert_eq!(Some(Duration::from_secs(123)), option.close_timeout());
        }
    }

    #[test]
    fn commit_option() {
        {
            let option = CommitOption::new();
            assert_eq!(CommitType::Default, option.commit_type());
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
