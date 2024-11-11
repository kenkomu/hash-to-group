use sha2::{Sha256, Digest};

fn sha256_h(input: &[u8]) -> u32 {
    // Compute SHA256 hash of the input
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    // Use the first two bytes for more variability
    let first_byte = result[0];
    let second_byte = result[1];
    let bit_0 = first_byte & 3; // Get the last two bits of the first byte
    let bit_1 = second_byte & 3; // Get the last two bits of the second byte

    println!("Input: {:?}, First Byte: {:08b}, Second Byte: {:08b}, bit_0: {}, bit_1: {}", 
             input, first_byte, second_byte, bit_0, bit_1);

    // Define Z5 group and its generators 2 and 3
    let g2 = 2;
    let g3 = 3;
    let modulus = 5;

    // Compute (2^bit_0 * 3^bit_1) % 5
    let power_g2 = mod_exp(g2, bit_0 as u32, modulus);
    let power_g3 = mod_exp(g3, bit_1 as u32, modulus);

    println!("power_g2: {}, power_g3: {}", power_g2, power_g3);

    (power_g2 * power_g3) % modulus
}

// Helper function to compute (base^exp) % modulus
fn mod_exp(base: u32, exp: u32, modulus: u32) -> u32 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    result
}

fn main() {
    // Test with empty string
    let result_empty = sha256_h(b"");
    println!("SHA256_H(\"\") = {}", result_empty);

    // Test with "SHA"
    let result_sha = sha256_h(b"SHA");
    println!("SHA256_H(\"SHA\") = {}", result_sha);

    // Test with "Math"
    let result_math = sha256_h(b"Math");
    println!("SHA256_H(\"Math\") = {}", result_math);
}
