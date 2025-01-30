use crate::block::Block;
use std::rc::Rc;

/// A `Blockchain` is a sequence or collection of `Block`s that securely records
/// transactions, by using cryptographic hashing, to be stored in `Block`s
pub struct Blockchain {
    /// A collection of `Block`s
    pub chain: Vec<Block>,
    
    /// A numeric factor representing how difficult it is
    /// to add a `Block` to this `Blockchain` instance. 
    /// 
    /// 
    /// The higher the value, the more difficult it is to match
    /// the `Block`'s starting hash value 
    pub difficulty: usize,
}
#[allow(dead_code)]
impl Blockchain {
    /// Constructs a new `Blockchain` instance, by adding a `genesis_block`,
    /// the first `Block`, to this instance
    ///
    /// # Returns
    /// - The current `Blockchain` instance - `Self`
    pub fn new(difficulty: usize) -> Self {
        let mut genesis_block = Block::new(0, String::from("Genesis Block"), None);
        genesis_block.proof_of_work(difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty
        }
    }

    /// Adds a new `Block` instance to this `Blockchain`
    ///
    /// # Parameters
    /// - `new_block` - A `Block` instance to be added to the current
    ///   `Blockchain` instance
    pub fn add_block(&mut self, mut new_block: Block) {
        new_block.proof_of_work(self.difficulty);
        self.chain.push(new_block);
    }
    
    pub fn get_latest_block_hash(&self) -> Option<Rc<str>> {
        self.chain.last().map(|block| Rc::from(block.hash.as_str()))
    }
    
    /// Validates the `Blockchain` by checking the hash of each block and ensuring
    /// that the hashes of the consecutive `Block`s match
    ///
    /// # Returns
    /// - A `bool` based on whether the hashes of all consecutive
    /// `Block`s match current. `true` if so, `false` otherwise
     pub fn is_valid(&self) -> Result<bool, &str> {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != Some(Rc::from(previous.hash.as_str())) {
                return Err("Blockchain is invalid! \
                    Blocks were moved and a hash mismatch has occurred")
            }
        }
        Ok(true)
    }
}
