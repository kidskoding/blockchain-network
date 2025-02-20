use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::arc_string::ArcString;
use crate::transaction::Transaction;

/// A `Block` stores a transaction, a digital operation
/// that represents the transfer or exchange of information, assets,
/// or value between participants on the network
///
/// The transaction includes several important components, including the
/// `index`, `timestamp`, `data`, `previous_hash`, and `hash`
#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    /// The position of the `Block` in a blockchain
    pub index: u32,

    /// The time, in seconds, the `Block` was created
    pub timestamp: u64,

    /// The transaction being stored in this `Block`
    pub transaction: Transaction,

    /// A reference to the previous `Block`'s hash in a blockchain, or
    /// `None` if there isn't any
    pub previous_hash: Option<ArcString>,

    /// The hash of this `Block` instance, which is calculated
    /// by using a cryptographic hashing algorithm, such as
    /// [SHA-256](https://securiti.ai/glossary/secure-hash-algorithm-sha-256-bit/)
    pub hash: String,
    
    /// **number used once** - a random or incrementing 32-bit hexadecimal number used in
    /// [Proof of Work (PoW)](https://www.investopedia.com/terms/p/proof-work.asp) mining
    ///
    /// Serves as a counter in [Proof of Work (PoW)](https://www.investopedia.com/terms/p/proof-work.asp) mining 
    /// that is adjusted until
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
    pub fn new(index: u32, transaction: Transaction, previous_hash: Option<ArcString>) -> Self {
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
}