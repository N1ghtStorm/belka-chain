pub type Address = [u8; 32];

pub struct Account {
    address: Address,
    balance: u128,
    nonce: u64,
    contract_code: Option<Vec<u8>>,
}

impl Account {
    pub fn new(address: Address) -> Self {
        Account { address, balance: 0, nonce: 0, contract_code: None }
    }
}