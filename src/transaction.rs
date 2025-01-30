use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a transaction in a `Blockchain`
#[derive(Debug)]
pub struct Transaction {
    /// The sender of this `Transaction`, or `None` if there isn't one
    sender: Option<String>,
    
    /// The recipient of this `Transaction`, or `None` if there isn't one
    recipient: Option<String>,
    
    /// The amount of this `Transaction`
    amount: f64,
    
    /// The timestamp when this `Transaction` was created 
    timestamp: u64,
    
    /// The digital signature of this `Transaction`
    signature: String,
    
    /// An optional unique identifier for this `Transaction`
    transaction_id: Option<String>,
    
    /// An optional fee for this `Transaction`
    fee: Option<f64>,
    
    /// Optional additional data or notes related to this `Transaction`
    metadata: Option<String>
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
}