use std::ops::Deref;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use crate::block::Block;
use crate::blockchain::Blockchain;

#[derive(Serialize, Deserialize)]
pub enum Message {
    NewBlock(Block),
    RequestChain,
    ResponseChain(Vec<Block>)
}

pub async fn start_server(blockchain: &mut Blockchain) {
    let listener = TcpListener::bind("127.0.0.1:8080");
    println!("Blockchain server running on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.await.unwrap().accept();
        tokio::spawn(async move {
            handle_connection(socket, blockchain).await;
        });
    }
}
async fn handle_connection(mut socket: TcpStream, blockchain: &mut Blockchain) {
    let mut buffer = [0; 1024];
    socket.read(&mut buffer).await.unwrap();

    let message: Message = serde_json::from_slice(&buffer).unwrap();
    match message {
        Message::NewBlock(block) => {
            blockchain.add_block(block).unwrap();
        }
        Message::RequestChain => {
            let chain = &blockchain.chain;
            let response = Message::ResponseChain(chain.deref().to_vec());
            let response = serde_json::to_vec(&response).unwrap();
            socket.write_all(&response).await.unwrap();
        }
        Message::ResponseChain(chain) => {
            // Handle received chain
        }
    }
}

pub async fn connect_to_peer(address: &str, message: Message) {
    let mut socket = TcpStream::connect(address).await.unwrap();
    let message = serde_json::to_vec(&message).unwrap();
    socket.write_all(&message).await.unwrap();
}