use std::io::Error;
use std::ops::Deref;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use crate::block::Block;
use crate::blockchain::Blockchain;

lazy_static! {
    pub static ref address: Arc<str> = Arc::from("127.0.0.1");
    pub static ref port: Arc<u16> = Arc::from(8080);
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    MineBlock(Block),
    RequestChain,
    ResponseChain(Vec<Block>),
    Connect(String),
    Disconnect(String),
}

pub async fn start_server(blockchain: Arc<tokio::sync::Mutex<Blockchain>>) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("{}:{}", *address, *port))
        .await?;
    println!("Blockchain server running on {}", format!("{}:{}", *address, *port));

    loop {
        let (socket, _) = listener.accept().await?;
        let blockchain = blockchain.clone();
        tokio::spawn(async move {
            handle_connection(socket, blockchain).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, blockchain: Arc<tokio::sync::Mutex<Blockchain>>) {
    let mut buffer: [u8; 1024] = [0; 1024];
    
    if let Ok(size) = socket.read(&mut buffer).await {
        if let Ok(message) = serde_json::from_slice::<Message>(&buffer[..size]) {
            let blockchain = blockchain.lock().await;
            match message {
                Message::MineBlock(block) => {
                    
                }
                Message::RequestChain => {
                    let chain = &blockchain.chain;
                    let response = Message::ResponseChain(chain.deref().to_vec());
                    if let Ok(response) = serde_json::to_vec(&response) {
                        let _ = socket.write_all(&response).await;
                    }
                }
                Message::Connect(name) => {
                    println!("{} connected on {} on port {}!", name, *address, *port);
                }
                Message::Disconnect(name) => {
                    println!("{} disconnected from the server!", name);
                }
                Message::ResponseChain(chain) => {
                    // Handle received chain
                }
            }
        }
    }
}