use ring::{digest, rand};

/// Generates a random array of constant size 64.
pub fn generate_random_array() -> [u8; digest::SHA512_OUTPUT_LEN] {
    let random = rand::SystemRandom::new();
    let random_array: [u8; digest::SHA512_OUTPUT_LEN] = rand::generate(&random).unwrap().expose();

    random_array
}

/// Returns a string out of a buffer for a given number of bytes.
pub fn get_content_from_buffer(buffer: &[u8], number_of_bytes: usize) -> String {
    String::from_utf8_lossy(&buffer[..number_of_bytes]).to_string()
}

#[cfg(test)]
mod utils {
    use super::*;

    #[test]
    fn check_random_array() {
        let a = generate_random_array();
        let b = generate_random_array();

        assert_ne!(&a[..], &b[..]);
        assert_eq!(a.len(), digest::SHA512_OUTPUT_LEN);
    }

    #[test]
    fn check_content_from_buffer() {
        let cat = "cat";
        let a: &[u8] = cat.as_ref();

        assert_eq!(get_content_from_buffer(&a[..], 3), cat);
    }
}
