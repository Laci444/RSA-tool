use rug::Integer;
use rug::integer::Order;

pub fn encrypt(message: &str, e: &Integer, n: &Integer) -> Integer {
    let message_bytes = message.as_bytes();
    let message_int = Integer::from_digits(message_bytes, Order::LsfLe);
    message_int.pow_mod(e, n).expect("modular exponentiation failed")
}

pub fn decrypt(cipher: &Integer, d: &Integer, n: &Integer) -> Vec<u8> {
    let decrypted_int = cipher.clone().pow_mod(d, n).expect("modular exponentiation failed");
    decrypted_int.to_digits(Order::LsfLe)
}
