use serde::Serialize;

use crate::core::account::Address;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    inner: TransactionInner,
    tip: u128,
}

#[derive(Debug, Clone, Serialize)]
pub enum TransactionInner {
    Transfer {
        from: Address,
        to: Address,
        amount: u128,
    },
    ContractCall()
}