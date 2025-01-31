#[cfg(test)]
mod miner_test {
    use crate::block::Block;
    use crate::blockchain::Blockchain;
    use crate::miner::Miner;
    use crate::transaction::Transaction;

    #[test]
    fn test_mine_block_with_sufficient_balance() {
        let mut miner = Miner { balance: Some(100.0) };
        let mut blockchain = Blockchain::new(4);
        let transaction = Transaction::new(Some("sender".to_string()), Some("receiver".to_string()), 10.0);
        let block = Block::new(1, transaction, None);

        let result = miner.mine_block(&mut blockchain, block);
        assert!(result.is_ok());
        assert_eq!(miner.balance, Some(190.0));
    }

    #[test]
    fn test_mine_block_with_insufficient_balance() {
        let mut miner = Miner { balance: Some(100.0) };
        let mut blockchain = Blockchain::new(4);
        let transaction = Transaction::new(Some("sender".to_string()), Some("receiver".to_string()), 110.0);
        let block = Block::new(1, transaction, None);

        let result = miner.mine_block(&mut blockchain, block);
        assert_eq!(result.is_err(), true);
    }
    
    #[test]
    fn test_mine_block_with_reward() {
        let mut blockchain = Blockchain::new(5);
        let mut miner = Miner {
            balance: Some(100.0),
        };
        let transaction = Transaction::new(Some("sender".to_string()), 
                                           Some("receiver".to_string()), 80.0);
        let block = Block::new(1, transaction, None);
        
        let result = miner.mine_block(&mut blockchain, block);
        assert!(result.is_ok());
        assert_eq!(miner.balance, Some(145.0));
    }
}