mod block;
mod blockchain;
mod tests;
mod transaction;
mod miner;
mod network;

use tokio::runtime::Runtime;
use crate::blockchain::Blockchain;
use crate::network::{connect_to_peer, start_server, Message};

fn main() {
    let mut blockchain = Blockchain::new(4);
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        tokio::spawn(start_server(&mut blockchain));
        connect_to_peer("127.0.0.1:8080", Message::RequestChain).await;
    });
}