use std::time::Duration;

use crate::jogasaki::proto::sql::request::ReadArea;
use crate::jogasaki::proto::sql::request::TransactionOption as RequestTransactionOption;
use crate::jogasaki::proto::sql::request::TransactionPriority;
use crate::jogasaki::proto::sql::request::TransactionType;
use crate::jogasaki::proto::sql::request::WritePreserve;
use crate::util::string_to_prost_string;

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
            transaction_type: TransactionType::Short,
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
            assert_eq!(TransactionPriority::Unspecified, option.priority());
            assert_eq!(None, option.close_timeout());

            let request = option.request();
            assert_eq!(TransactionType::Short, request.r#type());
            assert_eq!(TransactionPriority::Unspecified, request.priority());
            assert_eq!("", request.label);
            assert_eq!(false, request.modifies_definitions);
            assert_eq!(true, request.write_preserves.is_empty());
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
            assert_eq!(Some(Duration::from_secs(123)), option.close_timeout());
        }
    }
}
