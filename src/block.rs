use sha2::{Digest, Sha256};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::transaction::Transaction;

/// A `Block` stores a transaction, a digital operation
/// that represents the transfer or exchange of information, assets,
/// or value between participants on the network
///
/// The transaction includes several important components, including the
/// `index`, `timestamp`, `data`, `previous_hash`, and `hash`
#[allow(dead_code)]
pub struct Block {
    /// The position of the `Block` in a blockchain
    pub index: u32,

    /// The time, in seconds, the `Block` was created
    pub timestamp: u64,

    /// The transaction being stored in this `Block`
    pub transaction: Transaction,

    /// A reference to the previous `Block`'s hash in a blockchain, or
    /// `None` if there isn't any
    pub previous_hash: Option<Rc<str>>,

    /// The hash of the current `Block` instance, which is calculated
    /// by using a cryptographic hashing algorithm, like
    /// [SHA-256](https://securiti.ai/glossary/secure-hash-algorithm-sha-256-bit/)
    pub hash: String,
    
    /// **number used once** - a random or incrementing 32-bit hexadecimal number used in
    /// [Proof of Work (PoW)](https://www.investopedia.com/terms/p/proof-work.asp) mining
    ///
    /// In a `Blockchain`, it serves as a counter that is adjusted until
    /// a valid hash is found that meets the difficulty target of the `Blockchain`
    pub nonce: u32,
}
impl Block {
    /// Constructs a new `Block` with the given index, data, and
    /// hash of the previous block
    ///
    /// # Parameters
    /// - `index` - The position the block should be in a blockchain
    /// - `data` - The information that the `Block` should store
    /// - `previous_hash` - The reference/hash to the previous block in a blockchain,
    /// or `None` if there isn't one
    ///
    /// # Returns
    /// - `Self` - A newly constructed current `Block` instance that contains an
    ///   `index`, `timestamp`, `data`, `previous_hash`, and `hash`
    pub fn new(index: u32, transaction: Transaction, previous_hash: Option<Rc<str>>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Block {
            index,
            timestamp,
            transaction,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    /// Generates a [SHA-256](https://securiti.ai/glossary/secure-hash-algorithm-sha-256-bit/)
    /// hash of the `Block`'s contents
    ///
    /// # Parameters
    /// - `block` - A `Block` reference used to calculate its `hash`
    ///
    /// # Returns
    /// - `String` - A `String` representation containing the `hash` of the current `Block` instance
    pub fn calculate_hash(block: &Block) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}{:?}{:?}{:?}{:?}", 
                              block.index,
                              block.timestamp,
                              block.transaction,
                              block.previous_hash, 
                              block.nonce));
        format!("{:x}", hasher.finalize())
    }
    
    /// An implementation of the 
    /// [Proof of Work (PoW)](https://www.investopedia.com/terms/p/proof-work.asp) 
    /// algorithm
    /// 
    /// [Proof of Work (PoW)](https://www.investopedia.com/terms/p/proof-work.asp) 
    /// is a process/algorithm used to demonstrate/prove
    /// that a `Blockchain` did the necessary computing work
    /// required to add a new `Block` to the chain
    /// 
    /// Performed by mining the current `Block` instance
    /// by finding a valid hash that meets
    /// the given `difficulty` target
    /// 
    /// # Parameters
    /// - `difficulty` - The difficulty target for the `hash`,
    ///    representing how difficult it is for miners to
    ///    add new `Block`s to the `Blockchain`. 
    ///    - Represented in this prototype as the number of leading zeros required 
    ///      in the hash to consider the block valid.
    pub fn proof_of_work(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = Block::calculate_hash(self);
        }
    }
}