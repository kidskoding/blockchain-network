#[cfg(test)]
mod transaction_tests {
    use ring::signature::{Ed25519KeyPair, KeyPair};
    use ring::rand::SystemRandom;
    use crate::transaction::Transaction;

    #[test]
    fn test_sign_and_verify_transaction() {
        let rng = SystemRandom::new();
        let key_pair = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(key_pair.as_ref()).unwrap();

        let mut transaction = Transaction::new(
            Some(String::from("Alice")),
            Some(String::from("Bob")),
            50.0,
            None
        );
        transaction.sign(&key_pair);

        let public_key = key_pair.public_key().as_ref();
        assert!(transaction.verify_signature(public_key));
    }

    #[test]
    fn test_invalid_signature() {
        let rng = SystemRandom::new();
        let key_pair = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(key_pair.as_ref()).unwrap();

        let mut transaction = Transaction::new(
            Some(String::from("Alice")),
            Some(String::from("Bob")),
            50.0,
            None
        );
        transaction.sign(&key_pair);

        transaction.amount = 100.0;

        let public_key = key_pair.public_key().as_ref();
        assert!(!transaction.verify_signature(public_key));
    }
}