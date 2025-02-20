use std::error::Error;
use std::sync::Arc;
use tokio::runtime::Runtime;
use blockchain_network::blockchain::Blockchain;
use blockchain_network::network::{start_server};

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;

    rt.block_on(async {
        let blockchain = Arc::new(
            tokio::sync::Mutex::new(
                Blockchain::new(4)
            )
        );
        
        start_server(blockchain).await?;
        
        Ok(())
    })
}