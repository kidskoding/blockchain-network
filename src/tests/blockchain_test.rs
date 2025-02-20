#[cfg(test)]
mod blockchain_test {
    use std::sync::Arc;
    use crate::arc_string::ArcString;
    use crate::block::Block;
    use crate::blockchain::Blockchain;
    use crate::transaction::Transaction;

    fn sample_blockchain_instance() -> Blockchain {
        let mut blockchain = Blockchain::new(4);
        for x in 1..5 {
            let transaction = Transaction::new(
                Some(format!("Sender #{x}")),
                Some(format!("Recipient #{x}")),
                x as f64 * 10.0,
                None
            );
            let block = Block::new(x, transaction, blockchain.get_latest_block_hash());
            blockchain.add_block(block).unwrap();
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

        let transaction1 = Transaction::new(
            Some(String::from("Alice")),
            Some(String::from("Bob")),
            50.0,
            None
        );
        let block = Block::new(1, transaction1, None);
        blockchain.add_block(block).unwrap();

        let transaction2 = Transaction::new(
            Some(String::from("Bob")),
            Some(String::from("Charlie")),
            30.0,
            None
        );
        let block2 = Block::new(2, transaction2, blockchain.get_latest_block_hash());
        blockchain.add_block(block2).unwrap();

        let tampered_block = &mut blockchain.chain[1];
        tampered_block.transaction = Transaction::new(
            Some(String::from("Tampered")),
            Some(String::from("Transaction")),
            0.0,
            None
        );

        let result = blockchain.is_valid();
        assert!(result.is_err(), "Blockchain should be invalid after tampering!");
    }

    #[test]
    fn test_hashes_manually() {
        let mut blockchain = Blockchain::new(4);

        let transaction1 = Transaction::new(
            Some(String::from("Alice")),
            Some(String::from("Bob")),
            50.0,
            None
        );
        let block = Block::new(1, transaction1, None);
        blockchain.add_block(block).unwrap();

        let transaction2 = Transaction::new(
            Some(String::from("Bob")),
            Some(String::from("Charlie")),
            30.0,
            None
        );
        let block2 = Block::new(2, transaction2, blockchain.get_latest_block_hash());
        blockchain.add_block(block2).unwrap();

        let transaction3 = Transaction::new(
            Some(String::from("Charlie")),
            Some(String::from("Dave")),
            20.0,
            None,
        );
        let block3 = Block::new(3, transaction3, blockchain.get_latest_block_hash());
        blockchain.add_block(block3).unwrap();

        assert_eq!(blockchain.chain[1].previous_hash, None);
        assert_eq!(blockchain.chain[2].previous_hash,
                   Some(ArcString::from(Arc::from(blockchain.chain[1].hash.clone()))));
        assert_eq!(blockchain.chain[3].previous_hash,
                   Some(ArcString::from(Arc::from(blockchain.chain[2].hash.clone()))));
    }
}