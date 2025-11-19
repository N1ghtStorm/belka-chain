use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Peer {
    pub address: SocketAddr,
    pub node_id: Vec<u8>,
    pub is_connected: bool,
}

impl Peer {
    pub fn new(address: SocketAddr, node_id: Vec<u8>) -> Self {
        Peer {
            address,
            node_id,
            is_connected: false,
        }
    }

    pub fn connect(&mut self) {
        self.is_connected = true;
    }

    pub fn disconnect(&mut self) {
        self.is_connected = false;
    }
}

