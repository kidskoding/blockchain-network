use sha2::{Digest, Sha256};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

/// A `Block` stores a transaction, a digital operation
/// that represents the transfer or exchange of information, assets,
/// or value between participants on the network
///
/// The transcation includes several important components, including the
/// `index`, `timestamp`, `data`, `previous_hash`, and `hash`
pub struct Block {
    /// The position of the `Block` in a blockchain
    pub index: u32,

    /// The time, in seconds, the `Block` was created
    pub timestamp: u64,

    /// The information being stored in the `Block` (e.g. a transaction)
    pub data: String,

    /// A reference to the previous `Block`'s hash in a blockchain, or
    /// `None` if there isn't any
    pub previous_hash: Option<Rc<str>>,

    /// The hash of the current `Block` instance, which is calculated
    /// by using a cryptographic hashing algorithm, like
    /// [SHA-256](https://securiti.ai/glossary/secure-hash-algorithm-sha-256-bit/)
    pub hash: String,
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
    /// - A newly constructed `Block` instance that contains its
    /// `index`, `timestamp`, `data`, `previous_hash`, and `hash`
    pub fn new(index: u32, data: String, previous_hash: Option<Rc<str>>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let block_data = format!("{}{}{}{:?}", index, timestamp, data, previous_hash);
        let hash = Block::calculate_hash(block_data);

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
    pub fn calculate_hash(data: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
