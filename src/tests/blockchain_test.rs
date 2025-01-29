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
        assert_eq!(blockchain.is_valid(), true);
    }
}
