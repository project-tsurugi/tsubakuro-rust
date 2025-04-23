use std::time::Duration;

use crate::jogasaki::proto::sql::request::transaction_option::ScanParallelOpt;
use crate::jogasaki::proto::sql::request::ReadArea;
use crate::jogasaki::proto::sql::request::TransactionOption as RequestTransactionOption;
use crate::jogasaki::proto::sql::request::TransactionPriority;
use crate::jogasaki::proto::sql::request::TransactionType;
use crate::jogasaki::proto::sql::request::WritePreserve;
use crate::util::string_to_prost_string;

/// Transaction option.
///
/// See [SqlClient::start_transaction()](crate::prelude::SqlClient::start_transaction).
///
/// # Examples
///
/// ## OCC
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let mut transaction_option = TransactionOption::new();
/// transaction_option.set_transaction_type(TransactionType::Short);
/// ```
///
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let transaction_option = TransactionOption::from(TransactionType::Short);
/// ```
///
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let transaction_option = TransactionOption::default();
/// ```
///
/// # LTX
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let mut transaction_option = TransactionOption::new();
/// transaction_option.set_transaction_type(TransactionType::Long);
/// transaction_option.set_write_preserve(&["table1", "table2"]);
/// ```
///
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let mut transaction_option = TransactionOption::from(TransactionType::Long);
/// transaction_option.set_write_preserve(&["table1", "table2"]);
/// ```
///
/// ## DDL(LTX)
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let mut transaction_option = TransactionOption::new();
/// transaction_option.set_transaction_type(TransactionType::Long);
/// transaction_option.set_modifies_definitions(true);
/// ```
///
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let mut transaction_option = TransactionOption::from(TransactionType::Long);
/// transaction_option.set_modifies_definitions(true);
/// ```
///
/// ## RTX
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let mut transaction_option = TransactionOption::new();
/// transaction_option.set_transaction_type(TransactionType::ReadOnly);
/// ```
///
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// let transaction_option = TransactionOption::from(TransactionType::ReadOnly);
/// ```
#[derive(Debug, Clone)]
pub struct TransactionOption {
    transaction_type: TransactionType,
    transaction_label: Option<String>,
    modifies_definitions: bool,
    write_preserve: Vec<String>,
    inclusive_read_area: Vec<String>,
    exclusive_read_area: Vec<String>,
    scan_parallel: Option<i32>,
    priority: TransactionPriority,
    close_timeout: Option<Duration>,
}

impl Default for TransactionOption {
    fn default() -> Self {
        Self::new()
    }
}

impl TransactionOption {
    // Creates a new instance.
    pub fn new() -> TransactionOption {
        TransactionOption {
            transaction_type: TransactionType::Short,
            transaction_label: None,
            modifies_definitions: false,
            write_preserve: vec![],
            inclusive_read_area: vec![],
            exclusive_read_area: vec![],
            scan_parallel: None,
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
    /// Set transaction type.
    pub fn set_transaction_type(&mut self, transaction_type: TransactionType) {
        self.transaction_type = transaction_type;
    }

    /// Get transaction type.
    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type
    }

    /// Get transaction label.
    pub fn transaction_label(&self) -> Option<&String> {
        self.transaction_label.as_ref()
    }

    /// Set modifies definitions.
    pub fn set_modifies_definitions(&mut self, modifies_definitions: bool) {
        self.modifies_definitions = modifies_definitions;
    }

    /// Get modifies definitions.
    pub fn modifies_definitions(&self) -> bool {
        self.modifies_definitions
    }

    /// Get write preserve.
    pub fn write_preserve(&self) -> &Vec<String> {
        &self.write_preserve
    }

    /// Get inclusive read area.
    pub fn inclusive_read_area(&self) -> &Vec<String> {
        &self.inclusive_read_area
    }

    /// Get exclusive read area.
    pub fn exclusive_read_area(&self) -> &Vec<String> {
        &self.exclusive_read_area
    }

    /// Set scan parallel.
    ///
    /// since 0.2.0
    pub fn set_scan_parallel(&mut self, scan_parallel: i32) {
        self.scan_parallel = Some(scan_parallel);
    }

    /// Get scan parallel.
    ///
    /// since 0.2.0
    pub fn scan_parallel(&self) -> Option<i32> {
        self.scan_parallel
    }

    /// Set priority.
    pub fn set_priority(&mut self, priority: TransactionPriority) {
        self.priority = priority;
    }

    /// Get priority.
    pub fn priority(&self) -> TransactionPriority {
        self.priority
    }

    /// Set close timeout.
    pub fn set_close_timeout(&mut self, timeout: Duration) {
        self.close_timeout = Some(timeout);
    }

    /// Get close timeout.
    pub fn close_timeout(&self) -> Option<Duration> {
        self.close_timeout
    }
}

/// Transaction option setter for String.
pub trait TransactionOptionSetter<T> {
    /// Set transaction label.
    fn set_transaction_label(&mut self, transaction_label: T);

    /// Set write preserve.
    fn set_write_preserve(&mut self, table_names: &[T]);

    /// Set inclusive read area.
    fn set_inclusive_read_area(&mut self, table_names: &[T]);

    /// Set exclusive read area.
    fn set_exclusive_read_area(&mut self, table_names: &[T]);
}

impl TransactionOptionSetter<&str> for TransactionOption {
    fn set_transaction_label(&mut self, transaction_label: &str) {
        self.transaction_label = Some(transaction_label.to_string());
    }

    fn set_write_preserve(&mut self, table_names: &[&str]) {
        self.write_preserve = table_names.iter().map(|s| s.to_string()).collect()
    }

    fn set_inclusive_read_area(&mut self, table_names: &[&str]) {
        self.inclusive_read_area = table_names.iter().map(|s| s.to_string()).collect()
    }

    fn set_exclusive_read_area(&mut self, table_names: &[&str]) {
        self.exclusive_read_area = table_names.iter().map(|s| s.to_string()).collect()
    }
}

impl TransactionOptionSetter<String> for TransactionOption {
    fn set_transaction_label(&mut self, transaction_label: String) {
        self.transaction_label = Some(transaction_label);
    }

    fn set_write_preserve(&mut self, table_names: &[String]) {
        self.write_preserve = table_names.iter().map(|s| s.to_string()).collect()
    }

    fn set_inclusive_read_area(&mut self, table_names: &[String]) {
        self.inclusive_read_area = table_names.iter().map(|s| s.to_string()).collect()
    }

    fn set_exclusive_read_area(&mut self, table_names: &[String]) {
        self.exclusive_read_area = table_names.iter().map(|s| s.to_string()).collect()
    }
}

impl TransactionOption {
    pub(crate) fn request(&self) -> RequestTransactionOption {
        let tx_label = self.transaction_label.as_ref();

        RequestTransactionOption {
            r#type: self.transaction_type.into(),
            priority: self.priority.into(),
            label: string_to_prost_string(tx_label),
            modifies_definitions: self.modifies_definitions,
            write_preserves: Self::to_write_preserve(&self.write_preserve),
            inclusive_read_areas: Self::to_read_area(&self.inclusive_read_area),
            exclusive_read_areas: Self::to_read_area(&self.exclusive_read_area),
            scan_parallel_opt: self.scan_parallel.map(ScanParallelOpt::ScanParallel),
        }
    }

    fn to_write_preserve(table_names: &[String]) -> Vec<WritePreserve> {
        table_names
            .iter()
            .map(|s| WritePreserve {
                table_name: s.clone(),
            })
            .collect()
    }

    fn to_read_area(table_names: &[String]) -> Vec<ReadArea> {
        table_names
            .iter()
            .map(|s| ReadArea {
                table_name: s.clone(),
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transaction_option() {
        {
            let option = TransactionOption::new();
            assert_eq!(TransactionType::Short, option.transaction_type());
            assert_eq!(None, option.transaction_label());
            assert_eq!(false, option.modifies_definitions());
            assert_eq!(true, option.write_preserve().is_empty());
            assert_eq!(true, option.inclusive_read_area().is_empty());
            assert_eq!(true, option.exclusive_read_area().is_empty());
            assert_eq!(None, option.scan_parallel());
            assert_eq!(TransactionPriority::Unspecified, option.priority());
            assert_eq!(None, option.close_timeout());

            let request = option.request();
            assert_eq!(TransactionType::Short, request.r#type());
            assert_eq!(TransactionPriority::Unspecified, request.priority());
            assert_eq!("", request.label);
            assert_eq!(false, request.modifies_definitions);
            assert_eq!(true, request.write_preserves.is_empty());
            assert_eq!(None, request.scan_parallel_opt);
            assert_eq!(true, request.inclusive_read_areas.is_empty());
            assert_eq!(true, request.exclusive_read_areas.is_empty());
        }
        {
            let mut option = TransactionOption::new();
            option.set_transaction_type(TransactionType::Long);
            option.set_transaction_label("transaction_label");
            option.set_modifies_definitions(true);
            option.set_write_preserve(&vec!["wp"]);
            option.set_inclusive_read_area(&vec!["r1"]);
            option.set_exclusive_read_area(&vec!["r2"]);
            option.set_scan_parallel(12);
            option.set_priority(TransactionPriority::Interrupt);
            option.set_close_timeout(Duration::from_secs(123));

            let request = option.request();
            assert_eq!(TransactionType::Long, request.r#type());
            assert_eq!(TransactionPriority::Interrupt, request.priority());
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
            assert_eq!(
                Some(ScanParallelOpt::ScanParallel(12)),
                request.scan_parallel_opt
            );

            assert_eq!(Some(Duration::from_secs(123)), option.close_timeout());
        }
    }
}
