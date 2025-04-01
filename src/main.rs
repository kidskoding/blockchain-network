use std::sync::Arc;
use blockchain_network::client::Client;
use color_eyre::eyre::{Error, Result};
use clap::{Parser, Subcommand};
use blockchain_network::blockchain::Blockchain;
use blockchain_network::network::start_server;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Client,
    Server,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    match args.command {
        Some(Command::Server) => {
            let blockchain = Arc::new(tokio::sync::Mutex::new(Blockchain::new(4)));
            start_server(blockchain).await?;
        },
        Some(Command::Client) => {
            Client::run().await;
        },
        None => return Err(Error::msg("no command provided")),
    }

    Ok(())
}
