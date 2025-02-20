mod block;
mod blockchain;
mod tests;
mod transaction;
mod miner;
mod network;
mod arc_string;

use std::error::Error;
use std::sync::Arc;
use tokio::runtime::Runtime;
use crate::arc_string::ArcString;
use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::network::{connect_to_peer, start_server, Message};
use crate::transaction::Transaction;

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;

    rt.block_on(async {
        let blockchain = Arc::new(tokio::sync::Mutex::new(Blockchain::new(4)));
        let blockchain_copy = blockchain.clone();
        let server_handle = tokio::spawn(start_server(blockchain));

        let transaction = Transaction::new(
            Some("Alice".to_string()),
            Some("Bob".to_string()),
            0.0
        );
        
        let previous_hash = blockchain_copy.lock().await.chain.last()
            .map(|block| ArcString(Arc::new(block.hash.clone()))); 
        let new_block = Block::new(1, transaction, previous_hash);
        connect_to_peer("127.0.0.1:8080", Message::NewBlock(new_block)).await?;
        server_handle.await??;

        Ok(())
    })
}