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
use crate::blockchain::Blockchain;
use crate::network::{connect_to_peer, start_server, Message};

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;

    rt.block_on(async {
        let blockchain = Arc::new(tokio::sync::Mutex::new(Blockchain::new(4)));
        let server_handle = tokio::spawn(start_server(blockchain));

        // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        connect_to_peer("127.0.0.1:8080", Message::RequestChain).await?;
        server_handle.await??;

        Ok(())
    })
}