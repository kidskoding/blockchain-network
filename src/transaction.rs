use std::time::{SystemTime, UNIX_EPOCH};
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, ED25519};

/// Represents a transaction for a `Block`
#[derive(Debug)]
pub struct Transaction {
    /// The sender of this `Transaction`, or `None` if there isn't one
    pub sender: Option<String>,
    
    /// The recipient of this `Transaction`, or `None` if there isn't one
    pub recipient: Option<String>,
    
    /// The amount of this `Transaction`
    pub amount: f64,
    
    /// The timestamp when this `Transaction` was created 
    pub timestamp: u64,
    
    /// The digital signature of this `Transaction`
    pub signature: String,
    
    /// An optional unique identifier for this `Transaction`
    pub transaction_id: Option<String>,
    
    /// An optional fee for this `Transaction`
    pub fee: Option<f64>,
    
    /// Optional additional data or notes related to this `Transaction`
    pub metadata: Option<String>
}
impl Transaction {
    /// Creates a new `Transaction`.
    ///
    /// # Parameters
    /// - `sender`: The sender of this `Transaction`.
    /// - `recipient`: The recipient of this `Transaction`.
    /// - `amount`: The amount of this `Transaction`.
    /// - `signature`: The digital signature of this `Transaction`.
    ///
    /// # Returns
    /// A new `Transaction` instance with the current timestamp.
    pub fn new(sender: Option<String>, recipient: Option<String>, amount: f64, signature: String) -> Self {
        Transaction {
            sender,
            recipient,
            amount,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            signature,
            transaction_id: None,
            fee: None,
            metadata: None
        }
    }
   
    /// Securely signs this `Transaction`
    /// 
    /// # Parameters
    /// - `key_pair` - The ed25519 key-pair needed to sign this `Transaction`
    pub fn sign(&mut self, key_pair: &Ed25519KeyPair) {
        let message = format!("{:?}{:?}{:?}{:?}",
            self.sender,
            self.recipient,
            self.amount,
            self.timestamp,
        );
        let sig = key_pair.sign(message.as_bytes());
        self.signature = base64::encode(sig.as_ref());
    }
    
    /// Verifies the signature of this `Transaction` by using the miner's `public_key`
    /// 
    /// # Parameters
    /// - `public_key` - The miner's public key, which is used to verify the digital signature
    ///    of the transaction
    /// 
    /// # Returns
    /// - A `bool` containing whether the signature could be verified
    pub fn verify_signature(&self, public_key: &[u8]) -> bool {
        let message = format!("{:?}{:?}{:?}{:?}",
                              self.sender,
                              self.recipient,
                              self.amount,
                              self.timestamp);
        let sig = base64::decode(&self.signature).expect("Invalid base64 signature");
        let public_key = UnparsedPublicKey::new(&ED25519, public_key);
        public_key.verify(message.as_bytes(), &sig).is_ok()
    }
}