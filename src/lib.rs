use std::str::from_utf8;

use pyo3::prelude::*;

#[pyfunction]
fn mmh3_128_x64(key: String, seed: u64) -> PyResult<String> {
    // Initialization
    let a = mmh3_128_x64_bytes(key.as_bytes(), seed);
    Ok(format!("{:x}", a))
}

fn mmh3_128_x64_bytes(bytes: &[u8], seed: u64) -> u128 {
    let len = bytes.len();
    let mut h1 = seed;
    let mut h2 = seed;
    let c1: u64 = 0x87c37b91114253d5;
    let c2: u64 = 0x4cf5ad432745937f;

    // Body
    for chunk in bytes.chunks_exact(16){
        let mut k1 = u64::from_le_bytes(chunk[0..8].try_into().expect("oups"));
        let mut k2 = u64::from_le_bytes(chunk[8..16].try_into().expect("oups"));

        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(31);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
        h1 = h1.rotate_left(27);
        h1 = h1.wrapping_add(h2);
        h1 = h1.wrapping_mul(5).wrapping_add(0x52dce729);
        k2 = k2.wrapping_mul(c2);
        k2 = k2.rotate_left(33);
        k2 = k2.wrapping_mul(c1);
        h2 ^= k2;
        h2 = h2.rotate_left(31);
        h2 = h2.wrapping_add(h1);
        h2 = h2.wrapping_mul(5).wrapping_add(0x38495ab5);
    }

    // Tail
    let mut remainder = len % 16;
    let cursor = len - remainder;
    if remainder > 0 {
        if remainder >= 8 {
            let mut tail: [u8; 8] = [0; 8];
            for (i, b) in bytes[cursor+8..cursor+remainder].iter().enumerate() {
                tail[i] = *b;
            }
            println!("{:?}", from_utf8(&tail));
            let mut k2 = u64::from_le_bytes(tail);
            remainder = 8;
            k2 = k2.wrapping_mul(c2);
            k2 = k2.rotate_left(33);
            k2 = k2.wrapping_mul(c1);
            h2 ^= k2;
        }
        let mut tail: [u8; 8] = [0; 8];
        for (i, b) in bytes[cursor..cursor+remainder].iter().enumerate() {
            tail[i] = *b;
        }
        println!("{:?}", from_utf8(&tail));
        let mut k1 = u64::from_le_bytes(tail);
        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(31);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }

    h1 ^= len as u64;
    h2 ^= len as u64;
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    h1 = fmix64(h1);
    h2 = fmix64(h2);

    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    println!("{:x}", (h2 as u128) << 64 | h1 as u128);

    (h2 as u128) << 64 | h1 as u128
}

#[inline]
fn fmix64(mut k: u64) -> u64 {
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51afd7ed558ccd);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ceb9fe1a85ec53);
    k ^= k >> 33;
    k
}

/// A Python module implemented in Rust.
#[pymodule]
fn murmurh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mmh3_128_x64, m)?)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_string() {
        let result = mmh3_128_x64_bytes("".as_bytes(), 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_hello() {
        let result = mmh3_128_x64_bytes("hello world!1234".as_bytes(), 0);
        assert_eq!(result, 240867822025444006610977441818626719586);
    }

    #[test]
    fn test_foo() {
        let result = mmh3_128_x64_bytes("foo".as_bytes(), 0);
        assert_eq!(result, 168394135621993849475852668931176482145);
    }

    #[test]
    fn test_seed() {
        let result = mmh3_128_x64_bytes("foo".as_bytes(), 42);
        assert_eq!(result, 215966891540331383248189432718888555506);
    }

    #[test]
    fn test_fox() {
        let result = mmh3_128_x64_bytes("The quick brown fox jumps over the lazy dog.".as_bytes(), 0);
        assert_eq!(result, 140055101589960098446263325149249471177);
    }

    #[test]
    fn test_lorem() {
        let result = mmh3_128_x64_bytes("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".as_bytes(), 0);
        assert_eq!(result, 317940214952246513203822842115116886626);
    }
}
