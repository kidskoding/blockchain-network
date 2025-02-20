use std::f64::NAN;
use std::io::Error;
use std::ops::Deref;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::miner::Miner;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    NewBlock(Block),
    RequestChain,
    ResponseChain(Vec<Block>)
}

pub async fn start_server(blockchain: Arc<tokio::sync::Mutex<Blockchain>>) -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Blockchain server running on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        println!("Accepted new connection on 127.0.0.1 on port 8080!");
        let blockchain = blockchain.clone();
        tokio::spawn(async move {
            handle_connection(socket, blockchain).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, blockchain: Arc<tokio::sync::Mutex<Blockchain>>) {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut miner = Miner::new(Arc::from("Anirudh"), true);
    
    if let Ok(size) = socket.read(&mut buffer).await {
        if let Ok(message) = serde_json::from_slice::<Message>(&buffer[..size]) {
            let mut blockchain = blockchain.lock().await;
            match message {
                Message::NewBlock(block) => {
                    if let Ok(_) = miner.mine_block(&mut blockchain, block) {
                        println!("New block mined and added to the blockchain. {} balance: {:?} cryptos",
                                 String::from((&*miner.identifier).to_owned() + "'s"), miner.balance.unwrap_or(f64::NAN));
                    } else {
                        println!("Failed to mine the requested block.");
                    }
                }
                Message::RequestChain => {
                    let chain = &blockchain.chain;
                    let response = Message::ResponseChain(chain.deref().to_vec());
                    if let Ok(response) = serde_json::to_vec(&response) {
                        let _ = socket.write_all(&response).await;
                    }
                }
                Message::ResponseChain(chain) => {
                    // Handle received chain
                }
            }
        }
    }
}

pub async fn connect_to_peer(address: &str, message: Message) -> Result<(), Error> {
    let mut socket = TcpStream::connect(address).await?;
    let message = serde_json::to_vec(&message)?;
    socket.write_all(&message).await?;
    Ok(())
}