use std::char::from_u32;

use rand::{prelude::*, thread_rng, Rng, rngs::{adapter::ReseedingRng, OsRng}};
use rand_chacha::{ChaCha8Rng, ChaCha20Core};

fn main() {
    let password_length = 10;
    let mut result = String::new();

    for _ in 0..password_length {
        let number = thread_rng().gen_range(48..122);
        let ch = from_u32(number).unwrap();

        result.push(ch);
    }

    println!("{}", result);

    generate_random_numbers_with_a_seed(1220);

    non_deterministic();
}

fn generate_random_numbers_with_a_seed(seed: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    println!(
        "Deterministic Random numbers with seed:{}",
        rng.gen::<i32>()
    );
}

fn non_deterministic() {
    let prng = ChaCha20Core::from_entropy();
    let mut reseeding_rng = ReseedingRng::new(prng, 0, OsRng);

    println!("Random number: {}", reseeding_rng.gen::<u64>());
}
