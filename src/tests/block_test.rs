#[cfg(test)]
mod block_test {
    use crate::block::Block;
    use crate::blockchain::Blockchain;

    #[test]
    fn test_create_block() {
        let mut blockchain = Blockchain::new(4);
        let block = Block::new(
            1, 
            String::from("Transaction #1"),
            blockchain.get_latest_block_hash(),
        );
        blockchain.add_block(block);
        
        let block2 = Block::new(
            2,
            String::from("Transaction #2"),
            blockchain.get_latest_block_hash(),
        );
        blockchain.add_block(block2);
        
        assert_eq!(blockchain.is_valid(), Ok(true));
        assert_eq!(blockchain.chain.len(), 3);
    }

    #[test]
    fn test_proof_of_work() {
        let mut block = Block::new(1, String::from("Transaction #1"), None);
        let difficulty = 4;
        block.proof_of_work(difficulty);
        let target = "0".repeat(difficulty);
        
        assert!(
            block.hash.starts_with(&target),
            "Block hash does not meet difficulty target. Expected: {} Found: {}",
            target,
            block.hash
        );
        assert!(block.nonce > 0, "Nonce was not incremented during mining.");
    }
}
