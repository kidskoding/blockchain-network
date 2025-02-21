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

        let mut client = Client::new(&name);
        if let Err(e) = client.connect().await {
            eprintln!("Failed to connect: {}", e);
            return;
        }
        
        loop {
            println!("Choose an available option:");
            println!("1. Mine and send blocks");
            println!("2. Exit");
            
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line");
            let choice = choice.trim();
            
            match choice {
                "1" => {
                    let transaction = blockchain_network::transaction::Transaction::new(
                        Some("sender".to_string()),
                        Some("receiver".to_string()),
                        10.0,
                        None
                    );
                    let block = Block::new(1, transaction, None);
                    let blockchain = Arc::new(tokio::sync::Mutex::new(Blockchain::new(4)));

                    if let Err(e) = client.request_block(blockchain, block).await {
                        eprintln!("Failed to mine and send block: {}", e);
                    } 
                }
                "2" =>  {
                    if let Err(e) = client.disconnect().await {
                        eprintln!("Failed to disconnect: {}", e);
                    }
                    return;
                },
                _ => println!("Invalid option. Please try again!"),
            }
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
    
    pub async fn disconnect(&self) -> Result<(), Error> {
        let full_address = format!("{}:{}", *address, *port);
        let mut socket = TcpStream::connect(&full_address).await?;
        let disconnect_message = Message::Disconnect(self.miner.identifier.to_string());
        let message = serde_json::to_vec(&disconnect_message)?;
        socket.write_all(&message).await?;
        Ok(())
    }

    pub async fn request_block(&mut self, blockchain: Arc<tokio::sync::Mutex<Blockchain>>, block: Block)
                                     -> Result<(), Error> {

        let mut blockchain = blockchain.lock().await;
        let miner = &mut self.miner;
        miner.mine_block(&mut blockchain, block.clone()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        println!("{}'s crypto balance: {}", miner.identifier, miner.balance);
        
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