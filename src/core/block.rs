use crate::core::transaction::Transaction;
use crate::crypto::Hash;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_hash: Hash,
    pub timestamp: u64,
}

impl BlockHeader {
    pub fn hash(&self) -> Result<Hash, Box<dyn std::error::Error>> {
        Hash::hash_struct(self)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub state_root: Hash,
    pub timestamp: u64,
}

impl Block {
    pub fn new(
        version: u32,
        previous_hash: Hash,
        transactions: Vec<Transaction>,
        state_root: Hash,
        timestamp: u64,
    ) -> Self {
        Block {
            header: BlockHeader {
                version,
                previous_hash,
                timestamp,
            },
            transactions,
            state_root,
            timestamp,
        }
    }

    pub fn hash(&self) -> Result<Hash, Box<dyn std::error::Error>> {
        Hash::hash_struct(self)
    }

    pub fn parent_hash(&self) -> Hash {
        self.header.previous_hash
    }
}