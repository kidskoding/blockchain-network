use std::io;
use std::io::Error;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use blockchain_network::block::Block;
use blockchain_network::blockchain::Blockchain;
use blockchain_network::miner::Miner;
use blockchain_network::network::{address, port, Message};

pub struct Client {
    miner: Miner,
}

impl Client {
    pub fn new(name: &str) -> Self {
        Client {
            miner: Miner::new(Arc::from(name)),
        }
    }
    
    pub async fn run() {
        let mut name = String::new();

        println!("Enter your name:");
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();

        let client = Client::new(&name);
        if let Err(e) = client.connect().await {
            eprintln!("Failed to connect: {}", e);
        }
    }
    
    pub async fn connect(&self) -> Result<(), Error> {
        let full_address = format!("{}:{}", *address, *port);
        let mut socket = TcpStream::connect(&full_address).await?;
        let auth_message = Message::Connect(self.miner.identifier.to_string()); 
        let message = serde_json::to_vec(&auth_message)?;
        socket.write_all(&message).await?;

        let mut buffer = [0; 1024];
        loop {
            let size = socket.read(&mut buffer).await?;
            if size == 0 {
                break;
            }
            let response: serde_json::Value = serde_json::from_slice(&buffer[..size])?;
            println!("Received message: {:?}", response);
        }
        Ok(())
    }

    pub async fn mine_and_send_block(&mut self, blockchain: Arc<tokio::sync::Mutex<Blockchain>>, block: Block)
                                     -> Result<(), Error> {

        let mut blockchain = blockchain.lock().await;
        let mut miner = &mut self.miner;
        miner.mine_block(&mut blockchain, block.clone()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let message = Message::MineBlock(block);
        let serialized_message = serde_json::to_vec(&message)?;
        let full_address = format!("{}:{}", *address, *port);
        let mut socket = TcpStream::connect(&full_address).await?;
        socket.write_all(&serialized_message).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    Client::run().await;
}