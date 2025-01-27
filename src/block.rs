use std::time::{SystemTime, UNIX_EPOCH};

use sha2::digest::{DynDigest, Update};
use sha2::{Digest, Sha256};

/// A `Block` stores a transaction, a digital operation
/// that represents the transfer or exchange of information, assets,
/// or value between participants on the network
/// 
/// The transcation includes several important components, including the
/// `index`, `timestamp`, `data`, `previous_hash`, and `hash`
pub struct Block {
    /// The position of the `Block` in a blockchain
    pub index: u32,
    
    /// The time the `Block` was created
    pub timestamp: u64,
    
    /// The information being stored in the `Block` (e.g. a transaction)
    pub data: String,
    
    /// A reference (hash) to the previous `Block` in a blockchain
    pub previous_hash: String,
    
    /// The hash of the current `Block`, which is calculated from the `Block`'s contents
    pub hash: String,
}
impl Block {
    /// Constructs a new `Block` with the given index, data, and
    /// hash of the previous block
    /// 
    /// # Parameters
    /// - `index` - The position the block should be in a blockchain
    /// - `data` - The information that the `Block` should store
    /// - `previous_hash` - The reference/hash to the previous block in a blockchain
    /// 
    /// # Returns
    /// - A newly constructed `Block` instance that contains its 
    /// `index`, `timestamp`, `data`, `previous_hash`, and `hash`
    pub fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        let block_data = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        let hash = Block::calculate_hash(&block_data);
        
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
    
    /// Generates a [SHA-256](https://securiti.ai/glossary/secure-hash-algorithm-sha-256-bit/) 
    /// hash of the `Block`'s contents
    /// 
    /// # Parameters
    /// - `data` - A `&str` reference of the `Block`'s data 
    /// 
    /// # Returns
    /// - A `String` of the hash's current `Block` instance
    pub fn calculate_hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
