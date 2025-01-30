use crate::block::Block;
use crate::blockchain::Blockchain;

/// A modular representation of a `Miner`, which generally refers to
/// a person or their computing resources - a GPU (Graphics Processing Unit) or
/// CPU (Central Processing Unit) - who participate and engage in cryptocurrency (crypto)
pub struct Miner {
    /// A 64-bit floating-point balance for this `Miner`, represented in cryptos
    pub balance: Option<f64>,
}
impl Miner {
    /// Mines a new `Block` and adds it to the `Blockchain`
    /// 
    /// Performs the [Proof of Work (PoW)](https://www.investopedia.com/terms/p/proof-work.asp)
    /// algorithm to mine the given `Block`
    /// and then attempts ot add it to the given `Blockchain` if this `Miner`
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
            None => {
                blockchain.add_block(block)?;
            }
        }
        
        Ok(())
    }
    
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
}