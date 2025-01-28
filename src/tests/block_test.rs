#[cfg(test)]
mod block_test {
    use crate::blockchain::Blockchain;

    #[test]
    fn test_create_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(String::from("Transaction #1"));
        assert_eq!(blockchain.chain.len(), 2);
    }
}