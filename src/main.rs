mod network;

use network::{NetworkServer, Message};
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "belka-chain")]
#[command(about = "Belka Chain P2P Node")]
struct Args {
    #[arg(short, long, help = "Port to listen on")]
    port: u16,
    
    #[arg(short, long, help = "Peer address to connect to (optional)")]
    peer: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_start_p2p_network().await?;
    Ok(())
}

async fn  test_start_p2p_network() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let port = args.port;
    
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;
    let node_id = format!("node-{}", port).into_bytes();
    
    println!("=== Belka Chain Node (Port: {}) ===", port);
    
    let server = NetworkServer::new(addr, node_id.clone());
    server.start().await?;
    
    // if port != 8080 {
    //     let peer_addr: SocketAddr = format!("127.0.0.1:{}", port - 1).parse()?;
    //     println!("Connecting to peer at {}...", peer_addr);
    //     server.connect_to_peer(peer_addr, format!("node-{}", port - 1).into_bytes()).await?;
    //     sleep(Duration::from_secs(1)).await;
    // }
    
    let server_clone = server;
    let port_clone = port;

    sleep(Duration::from_secs(2)).await;
        
    loop {
        sleep(Duration::from_secs(3)).await;
        
        let peer_addresses = server_clone.get_peer_addresses().await;
        
        for addr in peer_addresses {
            let ping = Message::Ping;
            if server_clone.send_message(&addr, ping).await.is_ok() {
                println!("[Node {}] Sent Ping to {}", port_clone, addr);
            }
        }
    }
    // tokio::spawn(async move {
    //     sleep(Duration::from_secs(2)).await;
        
    //     loop {
    //         sleep(Duration::from_secs(3)).await;
            
    //         let peer_addresses = server_clone.get_peer_addresses().await;
            
    //         for addr in peer_addresses {
    //             let ping = Message::Ping;
    //             if server_clone.send_message(&addr, ping).await.is_ok() {
    //                 println!("[Node {}] Sent Ping to {}", port_clone, addr);
    //             }
    //         }
    //     }
    // });
    
    println!("Node running. Press Ctrl+C to stop.");
    
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
