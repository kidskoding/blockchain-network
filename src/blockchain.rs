use std::rc::Rc;
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair};
use crate::transaction::Transaction;
use crate::block::Block;

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
    /// - `Self` - The current `Blockchain` instance
    pub fn new(difficulty: usize) -> Self {
        let mut genesis_block = Block::new(
            0, 
            Transaction::new(
                None,
                None,
                0.0,
            ), 
            None
        );
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
    /// 
    /// # Returns
    /// - `Result<(), &str>` - A result that contains whether the block was
    ///    successfully added or not. If the signature verification fails, an `Err(&str)`
    ///    is thrown
    pub fn add_block(&mut self, mut new_block: Block) -> Result<(), &str> {
        new_block.proof_of_work(self.difficulty);

        let rng = SystemRandom::new();
        let key_pair = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(key_pair.as_ref()).unwrap();
        
        new_block.transaction.sign(&key_pair);
        let public_key = key_pair.public_key().as_ref();
        if new_block.transaction.verify_signature(public_key) {
            self.chain.push(new_block);
            Ok(())
        } else {
            Err("Could not verify the signature of the block's transaction!")
        }
    }
    
    /// Gets the hash value for the most recent `Block` added to this `Blockchain`
    /// 
    /// # Returns
    /// - `Option<Rc<str>>` - An optional reference count value, containing the hash of the most recent `Block`
    ///   added to this `Blockchain`
    pub fn get_latest_block_hash(&self) -> Option<Rc<str>> {
        self.chain.last().map(|block| Rc::from(block.hash.as_str()))
    }
    
    /// Validates the `Blockchain` by checking the hash of each block and ensuring
    /// that the hashes of the consecutive `Block`s match
    ///
    /// # Returns
    /// - `Result<bool, &str>` - A result based on whether the hashes of all consecutive
    ///   `Block`s match that of the current `Block`. `true` if so, an `Err(&str)` otherwise
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