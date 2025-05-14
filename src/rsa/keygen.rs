use rug::{Integer, Complete};
use rug::integer::IsPrime;
use rug::rand::RandState;
use rand::{Rng};

pub fn generate_rsa_keypair(bits: u32) -> (Integer, Integer, Integer) {
    let mut rng = rand::thread_rng();
    let mut state = RandState::new_mersenne_twister();
    state.seed(&Integer::from(rng.r#gen::<u64>()));

    let one = Integer::from(1);

    let p = loop {
        let candidate = Integer::from(Integer::random_bits(bits / 2, &mut state));
        if candidate.is_probably_prime(40) != IsPrime::No {
            break candidate;
        }
    };

    let q = loop {
        let candidate = Integer::from(Integer::random_bits(bits / 2, &mut state));
        if candidate != p && candidate.is_probably_prime(40) != IsPrime::No {
            break candidate;
        }
    };

    let n = (&p * &q).complete();
    let phi = (&p - &one).complete() * (&q - &one).complete();

    let e = Integer::from(65537);
    let d = e.clone().invert(&phi).expect("e and phi(n) must be coprime");

    (e, d, n)
}
