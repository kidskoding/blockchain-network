#[cfg(test)]
mod blockchain_test {
    use std::rc::Rc;
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
                format!("signature#{x}"),
            );
            let block = Block::new(x, transaction, blockchain.get_latest_block_hash());
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

        let transaction1 = Transaction::new(
            Some(String::from("Alice")),
            Some(String::from("Bob")),
            50.0,
            String::from("signature1"),
        );
        let block = Block::new(1, transaction1, None);
        blockchain.add_block(block);

        let transaction2 = Transaction::new(
            Some(String::from("Bob")),
            Some(String::from("Charlie")),
            30.0,
            String::from("signature2"),
        );
        let block2 = Block::new(2, transaction2, blockchain.get_latest_block_hash());
        blockchain.add_block(block2);

        let tampered_block = &mut blockchain.chain[1];
        tampered_block.transaction = Transaction::new(
            Some(String::from("Tampered")),
            Some(String::from("Transaction")),
            0.0,
            String::from("tampered_signature"),
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
            String::from("signature1"),
        );
        let block = Block::new(1, transaction1, None);
        blockchain.add_block(block);

        let transaction2 = Transaction::new(
            Some(String::from("Bob")),
            Some(String::from("Charlie")),
            30.0,
            String::from("signature2"),
        );
        let block2 = Block::new(2, transaction2, blockchain.get_latest_block_hash());
        blockchain.add_block(block2);

        let transaction3 = Transaction::new(
            Some(String::from("Charlie")),
            Some(String::from("Dave")),
            20.0,
            String::from("signature3"),
        );
        let block3 = Block::new(3, transaction3, blockchain.get_latest_block_hash());
        blockchain.add_block(block3);

        assert_eq!(blockchain.chain[1].previous_hash, None);
        assert_eq!(blockchain.chain[2].previous_hash,
                   Some(Rc::from(blockchain.chain[1].hash.as_str())));
        assert_eq!(blockchain.chain[3].previous_hash,
                   Some(Rc::from(blockchain.chain[2].hash.as_str())));
    }
}