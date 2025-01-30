#[cfg(test)]
mod blockchain_test {
    use std::rc::Rc;
    use crate::block::Block;
    use crate::blockchain::Blockchain;
    
    fn sample_blockchain_instance() -> Blockchain {
        let mut blockchain = Blockchain::new(4);
        for x in 1..5 {
            let block = Block::new(x, format!("Transaction #{x}"), blockchain.get_latest_block_hash());
            blockchain.add_block(block);
        }
        
        blockchain
    }
    
    #[test]
    fn test_create_blockchain() {
        let blockchain = sample_blockchain_instance();
        assert_eq!(blockchain.chain.len(), 5);
    }

    #[test]
    fn test_is_valid_blockchain() {
        let blockchain = sample_blockchain_instance();
        assert_eq!(blockchain.is_valid(), Ok(true));
    }
    
    #[test]
    fn test_invalid_blockchain() {
        let mut blockchain = Blockchain::new(4);

        let block = Block::new(
            1,
            String::from("Transaction #1"),
            None,
        );
        blockchain.add_block(block);

        let block2 = Block::new(
            2,
            String::from("Transaction #2"),
            blockchain.get_latest_block_hash(),
        );
        blockchain.add_block(block2);
        
        let tampered_block = &mut blockchain.chain[1];
        tampered_block.data = String::from("Tampered Transaction");
        
        let result = blockchain.is_valid();
        assert!(result.is_err(), "Blockchain should be invalid after tampering!");
    }
    
    #[test]
    fn test_hashes_manually() {
        let mut blockchain = Blockchain::new(4);

        let block = Block::new(
            1,
            String::from("Transaction #1"),
            None,
        );
        blockchain.add_block(block);

        let block2 = Block::new(
            2,
            String::from("Transaction #2"),
            blockchain.get_latest_block_hash(),
        );
        blockchain.add_block(block2);

        let block3 = Block::new(
            3,
            String::from("Transaction #3"),
            blockchain.get_latest_block_hash(),
        );
        blockchain.add_block(block3);

        assert_eq!(blockchain.chain[1].previous_hash, None);
        assert_eq!(blockchain.chain[2].previous_hash, 
                   Some(Rc::from(blockchain.chain[1].hash.as_str())));
        assert_eq!(blockchain.chain[3].previous_hash, 
                   Some(Rc::from(blockchain.chain[2].hash.as_str())));
    }
}
