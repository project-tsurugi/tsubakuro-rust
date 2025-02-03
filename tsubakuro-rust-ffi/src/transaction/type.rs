use tsubakuro_rust_core::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
#[allow(dead_code)]
pub enum TsurugiFfiTransactionType {
    /// use default transaction type.
    Unspecified = 0,
    /// short transactions (optimistic concurrency control).
    Short = 1,
    /// long transactions (pessimistic concurrency control).
    Long = 2,
    /// read only transactions (may be abort-free).
    ReadOnly = 3,
}

impl TsurugiFfiTransactionType {
    pub(crate) fn is_valid(value: i32) -> bool {
        matches!(value, 0 | 1 | 2 | 3)
    }
}

impl From<TransactionType> for TsurugiFfiTransactionType {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Unspecified => TsurugiFfiTransactionType::Unspecified,
            TransactionType::Short => TsurugiFfiTransactionType::Short,
            TransactionType::Long => TsurugiFfiTransactionType::Long,
            TransactionType::ReadOnly => TsurugiFfiTransactionType::ReadOnly,
        }
    }
}

impl From<TsurugiFfiTransactionType> for TransactionType {
    fn from(value: TsurugiFfiTransactionType) -> Self {
        match value {
            TsurugiFfiTransactionType::Unspecified => Self::Unspecified,
            TsurugiFfiTransactionType::Short => Self::Short,
            TsurugiFfiTransactionType::Long => Self::Long,
            TsurugiFfiTransactionType::ReadOnly => Self::ReadOnly,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
#[allow(dead_code)]
pub enum TsurugiFfiTransactionPriority {
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

impl TsurugiFfiTransactionPriority {
    pub(crate) fn is_valid(value: i32) -> bool {
        matches!(value, 0 | 1 | 2 | 3 | 4)
    }
}

impl From<TransactionPriority> for TsurugiFfiTransactionPriority {
    fn from(value: TransactionPriority) -> Self {
        match value {
            TransactionPriority::Unspecified => TsurugiFfiTransactionPriority::Unspecified,
            TransactionPriority::Interrupt => TsurugiFfiTransactionPriority::Interrupt,
            TransactionPriority::Wait => TsurugiFfiTransactionPriority::Wait,
            TransactionPriority::InterruptExclude => {
                TsurugiFfiTransactionPriority::InterruptExclude
            }
            TransactionPriority::WaitExclude => TsurugiFfiTransactionPriority::WaitExclude,
        }
    }
}

impl From<TsurugiFfiTransactionPriority> for TransactionPriority {
    fn from(value: TsurugiFfiTransactionPriority) -> Self {
        match value {
            TsurugiFfiTransactionPriority::Unspecified => Self::Unspecified,
            TsurugiFfiTransactionPriority::Interrupt => Self::Interrupt,
            TsurugiFfiTransactionPriority::Wait => Self::Wait,
            TsurugiFfiTransactionPriority::InterruptExclude => Self::InterruptExclude,
            TsurugiFfiTransactionPriority::WaitExclude => Self::WaitExclude,
        }
    }
}
