#[cfg(test)]
mod blockchain_test {
    use crate::blockchain::Blockchain;
    #[test]
    fn test_create_blockchain() {
        let mut blockchain = Blockchain::new();
        for x in 1..5 {
            blockchain.add_block(format!("Transaction #{x}"));
        }
        assert_eq!(blockchain.chain.len(), 5);
    }

    #[test]
    fn test_is_valid_blockchain() {
        let mut blockchain = Blockchain::new();
        for x in 1..5 {
            blockchain.add_block(format!("Transaction #{x}"));
        }
        assert_eq!(blockchain.is_valid(), Ok(true));
    }
    
    #[test]
    fn test_invalid_blockchain() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block(String::from("Transaction #1"));
        blockchain.add_block(String::from("Transaction #2"));
        
        let tampered_block = &mut blockchain.chain[1];
        tampered_block.data = String::from("Tampered Transaction");
        
        let result = blockchain.is_valid();
        assert!(result.is_err(), "Blockchain should be invalid after tampering!");
    }
}
