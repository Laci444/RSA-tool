use rug::Integer;
use sha2::{Sha256, Digest};

pub fn sign(message: &[u8], d: &Integer, n: &Integer) -> Integer {
    let mut hasher = Sha256::new();
    hasher.update(message);
    let hash = hasher.finalize();
    let hash_int = Integer::from_digits(&hash, rug::integer::Order::MsfBe);
    hash_int.pow_mod(d, n).expect("modular exponentiation failed")
}

pub fn verify(message: &[u8], signature: &Integer, e: &Integer, n: &Integer) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(message);
    let expected_hash = hasher.finalize();
    let expected_int = Integer::from_digits(&expected_hash, rug::integer::Order::MsfBe);
    if let Ok(result) = signature.clone().pow_mod(e, n) {
        result == expected_int
    } else {
        false
    }
}
