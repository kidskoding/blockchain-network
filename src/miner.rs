use crate::block::Block;
use crate::blockchain::Blockchain;

/// A modular representation of a `Miner`, which generally refers to
/// a person or their computing resources - a GPU (Graphics Processing Unit) or
/// CPU (Central Processing Unit) - who participate and engage in cryptocurrency (crypto)
pub struct Miner {
    /// A 64-bit floating-point balance for this `Miner`, represented in cryptos
    /// 
    /// - Represented as a `Some(f64)` for `Miner`s that are human
    /// - Represented as a `None` for `Miner`s that are computing resources
    pub balance: Option<f64>,
}
impl Miner {
    /// An implementation of the 
    /// [Proof of Work (PoW)](https://www.investopedia.com/terms/p/proof-work.asp) 
    /// algorithm
    ///
    /// [Proof of Work (PoW)](https://www.investopedia.com/terms/p/proof-work.asp) 
    /// is a cryptographic puzzle/algorithm that a `Miner` must complete to demonstrate/prove
    /// that it did the necessary computing work
    /// required to add a new `Block` to the `Blockchain`
    ///
    /// Performed by mining the current `Block` instance
    /// by finding a valid hash that meets
    /// the given `difficulty` target
    ///
    /// # Parameters
    /// - `block` - The `Block` instance, as a mutable reference, to be mined
    /// - `difficulty` - The difficulty target for the `hash`,
    ///    representing how difficult it is for miners to
    ///    add new `Block`s to the `Blockchain`. 
    ///    - Represented in this prototype as the number of leading zeros required 
    ///      in the hash to consider the block valid.
    pub fn proof_of_work(block: &mut Block, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !block.hash.starts_with(&target) {
            block.nonce += 1;
            block.hash = Block::calculate_hash(block);
        }
    }
    
    /// Mines a new `Block` and adds it to the `Blockchain`
    /// 
    /// Performs the [Proof of Work (PoW)](https://www.investopedia.com/terms/p/proof-work.asp)
    /// algorithm to mine the given `Block`
    /// and then attempts to add it to the given `Blockchain` if this `Miner`
    /// has enough crypto balance to do so
    /// 
    /// # Parameters
    /// - `blockchain` - A mutable reference to the `Blockchain`, 
    ///    where the mined `Block` will be added
    /// - `block` - The `Block` to be mined and added to the `Blockchain`
    /// 
    /// # Returns
    /// - `Result<(), &str>` - Returns a result based on whether the given `Block` was successfully
    ///    mined and added to the given `Blockchain`
    pub fn mine_block<'a>(&mut self, blockchain: &'a mut Blockchain, mut block: Block) -> Result<(), &'a str> {
        Self::proof_of_work(&mut block, blockchain.difficulty);
        
        match self.balance {
            Some(ref mut value) => {
                if *value >= block.transaction.amount {
                    *value -= block.transaction.amount;
                } else { 
                    return Err("Insufficient crypto balance to mine the block!");
                }
            }
            _ => {} 
        }
        
        let reward = Self::calculate_block_reward(blockchain);
        blockchain.add_block(block)?;
        
        if let Some(ref mut val) = self.balance {
            *val += reward;
        }
        Ok(())
    }
    
    /// Calculates the reward for mining a block based on the block height.
    ///
    /// The reward is initially 50 cryptos and is halved every 210,000 blocks.
    /// This method follows the reward halving schedule, from many other cryptocurrencies like 
    /// [Bitcoin](https://bitcoin.org/en/)
    ///
    /// # Parameters
    /// - `block_height` - The height of the block for which the reward is being calculated.
    ///
    /// # Returns
    /// - `f64` - The calculated reward for the given block height.
    pub fn calculate_block_reward(blockchain: &Blockchain) -> f64 {
        let initial_reward = 50.0;
        let halving_interval = 210000;
        let halvings = blockchain.chain.len() - 1 / halving_interval;
        let base_reward = initial_reward / 2f64.powi(halvings as i32);
        base_reward * blockchain.difficulty as f64
    }
}