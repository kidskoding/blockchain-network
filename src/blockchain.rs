use std::rc::Rc;
use crate::block::Block;

/// A `Blockchain` is a sequence or collection of `Block`s that securely records
/// transactions, by using cryptographic hashing, to be stored in `Block`s
pub struct Blockchain {
    /// A collection of `Block`s
    pub chain: Vec<Block>,
}
impl Blockchain {
    /// Constructs a new `Blockchain` instance, by adding a `genesis_block`,
    /// the first `Block`, to this instance
    /// 
    /// # Returns
    /// - The current `Blockchain` instance - `Self`
    pub fn new() -> Self {
        let genesis_block = Block::new(0, String::from("Genesis Block"), None);
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    
    /// Adds a new `Block` instance to this `Blockchain`
    /// 
    /// # Parameters
    /// - `data` - A `String` representation of the `Block`s data
    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last();
        let new_block = match previous_block {
            None => Block::new(1, data, None),
            Some(block) => Block::new(block.index + 1, data, Some(Rc::from(block.hash.as_str())))
        };
        self.chain.push(new_block);
    }
    
    /// Validates the `Blockchain` by checking the hash of each block and ensuring
    /// that the hashes of the consecutive `Block`s match 
    /// 
    /// # Returns
    /// - A `bool` based on whether the hashes of all consecutive
    /// `Block`s match current. `true` if so, `false` otherwise
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            
            let format_str: String = format!(
                "{}{}{}{:?}",
                current.index, current.timestamp, current.data, current.previous_hash
            );
            if current.hash != Block::calculate_hash(format_str) 
                || current.previous_hash != Some(Rc::from(previous.hash.as_str())) {
                
                return false;
            }
        }
        true
    }
}