use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Handshake {
        node_id: Vec<u8>,
        version: String,
        chain_height: u64,
    },
    GetBlocks {
        from_height: u64,
        to_height: u64,
    },
    Blocks {
        blocks: Vec<Vec<u8>>,
    },
    GetTransactions,
    Transactions {
        transactions: Vec<Vec<u8>>,
    },
    NewBlock {
        block: Vec<u8>,
    },
    NewTransaction {
        transaction: Vec<u8>,
    },
    Ping,
    Pong,
}

