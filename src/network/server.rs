use crate::network::{Peer, Message};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{RwLock, mpsc};

pub struct NetworkServer {
    peers: Arc<RwLock<HashMap<SocketAddr, Peer>>>,
    message_senders: Arc<RwLock<HashMap<SocketAddr, mpsc::UnboundedSender<Message>>>>,
    listening_address: SocketAddr,
    node_id: Vec<u8>,
}

impl NetworkServer {
    pub fn new(listening_address: SocketAddr, node_id: Vec<u8>) -> Self {
        NetworkServer {
            peers: Arc::new(RwLock::new(HashMap::new())),
            message_senders: Arc::new(RwLock::new(HashMap::new())),
            listening_address,
            node_id,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(self.listening_address).await?;
        println!("Network server listening on {}", self.listening_address);
        
        let peers = Arc::clone(&self.peers);
        let message_senders = Arc::clone(&self.message_senders);
        let node_id = self.node_id.clone();
        
        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        println!("New connection from {}", addr);
                        
                        let mut peer = Peer::new(addr, vec![]);
                        peer.connect();
                        peers.write().await.insert(addr, peer);
                        
                        let (tx, rx) = mpsc::unbounded_channel();
                        message_senders.write().await.insert(addr, tx);
                        
                        let peers_clone = Arc::clone(&peers);
                        let message_senders_clone = Arc::clone(&message_senders);
                        let node_id_clone = node_id.clone();
                        
                        tokio::spawn(async move {
                            handle_connection(stream, addr, peers_clone, message_senders_clone, rx, node_id_clone).await;
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                    }
                }
            }
        });
        
        Ok(())
    }

    pub async fn connect_to_peer(&self, address: SocketAddr, node_id: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(address).await?;
        println!("Connected to peer at {}", address);
        
        let mut peer = Peer::new(address, node_id);
        peer.connect();
        self.peers.write().await.insert(address, peer.clone());
        
        let (tx, rx) = mpsc::unbounded_channel();
        self.message_senders.write().await.insert(address, tx);
        
        let peers = Arc::clone(&self.peers);
        let message_senders = Arc::clone(&self.message_senders);
        let node_id = self.node_id.clone();
        
        tokio::spawn(async move {
            handle_connection(stream, address, peers, message_senders, rx, node_id).await;
        });
        
        Ok(())
    }

    pub async fn send_message(&self, address: &SocketAddr, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        let senders = self.message_senders.read().await;
        if let Some(tx) = senders.get(address) {
            tx.send(message)?;
            return Ok(());
        }
        Err("No connection to peer".into())
    }

    pub async fn peer_count(&self) -> usize {
        self.peers.read().await.values().filter(|p| p.is_connected).count()
    }

    pub fn get_peers_arc(&self) -> Arc<RwLock<HashMap<SocketAddr, Peer>>> {
        Arc::clone(&self.peers)
    }

    pub async fn get_peer_addresses(&self) -> Vec<SocketAddr> {
        self.peers.read().await
            .iter()
            .filter(|(_, p)| p.is_connected)
            .map(|(addr, _)| *addr)
            .collect()
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    peers: Arc<RwLock<HashMap<SocketAddr, Peer>>>,
    message_senders: Arc<RwLock<HashMap<SocketAddr, mpsc::UnboundedSender<Message>>>>,
    mut rx: mpsc::UnboundedReceiver<Message>,
    _node_id: Vec<u8>,
) {
    let mut buffer = [0u8; 1024];
    
    let (mut stream_rx, mut stream_tx) = tokio::io::split(stream);
    let stream_writer = Arc::new(tokio::sync::Mutex::new(stream_tx));
    
    let stream_writer_clone = Arc::clone(&stream_writer);
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Ok(encoded) = bincode::serialize(&message) {
                let mut writer = stream_writer_clone.lock().await;
                if writer.write_all(&encoded).await.is_err() {
                    break;
                }
            }
        }
    });
    
    loop {
        tokio::select! {
            result = stream_rx.read(&mut buffer) => {
                match result {
                    Ok(0) => {
                        println!("Connection closed with {}", addr);
                        break;
                    }
                    Ok(n) => {
                        match bincode::deserialize::<Message>(&buffer[..n]) {
                            Ok(message) => {
                                println!("[{}] Received: {:?}", addr, message);
                                
                                let response = match message {
                                    Message::Ping => {
                                        println!("[{}] Responding with Pong", addr);
                                        Message::Pong
                                    }
                                    Message::Pong => {
                                        println!("[{}] Received Pong!", addr);
                                        continue;
                                    }
                                    _ => continue,
                                };
                                
                                if let Ok(encoded) = bincode::serialize(&response) {
                                    let mut writer = stream_writer.lock().await;
                                    if writer.write_all(&encoded).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to deserialize message from {}: {}", addr, e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from {}: {}", addr, e);
                        break;
                    }
                }
            }
        }
    }
    
    peers.write().await.remove(&addr);
    message_senders.write().await.remove(&addr);
}

