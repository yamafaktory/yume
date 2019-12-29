use ring::{digest, rand};

pub fn generate_random_array() -> [u8; digest::SHA512_OUTPUT_LEN] {
    let random = rand::SystemRandom::new();
    let random_array: [u8; digest::SHA512_OUTPUT_LEN] = rand::generate(&random).unwrap().expose();

    random_array
}
