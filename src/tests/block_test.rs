#[cfg(test)]
mod block_test {
    use crate::block::Block;
    use crate::blockchain::Blockchain;
    use crate::miner::Miner;
    use crate::transaction::Transaction;

    #[test]
    fn test_create_block() {
        let mut blockchain = Blockchain::new(4);
        let transaction1 = Transaction::new(
            Some(String::from("Alice")),
            Some(String::from("Bob")),
            50.0,
            None
        );
        let block = Block::new(
            1,
            transaction1,
            blockchain.get_latest_block_hash(),
        );
        blockchain.add_block(block).unwrap();

        let transaction2 = Transaction::new(
            Some(String::from("Bob")),
            Some(String::from("Charlie")),
            30.0,
            None
        );
        let block2 = Block::new(
            2,
            transaction2,
            blockchain.get_latest_block_hash(),
        );
        blockchain.add_block(block2).unwrap();

        assert_eq!(blockchain.is_valid(), Ok(true));
        assert_eq!(blockchain.chain.len(), 3);
    }

    #[test]
    fn test_proof_of_work() {
        let transaction = Transaction::new(
            Some(String::from("Alice")),
            Some(String::from("Bob")),
            50.0,
            None
        );
        let mut block = Block::new(1, transaction, None);
        let difficulty = 4;
        Miner::proof_of_work(&mut block, difficulty);
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