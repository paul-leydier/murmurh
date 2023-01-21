use std::convert::TryInto;

const C1: u64 = 0x87c37b91114253d5;
const C2: u64 = 0x4cf5ad432745937f;
const C3: u64 = 0x52dce729;
const C4: u64 = 0x38495ab5;
const C5: u64 = 0xff51afd7ed558ccd;
const C6: u64 = 0xc4ceb9fe1a85ec53;

#[inline]
fn fmix64(mut k: u64) -> u64 {
    k ^= k >> 33;
    k = k.wrapping_mul(C5);
    k ^= k >> 33;
    k = k.wrapping_mul(C6);
    k ^= k >> 33;
    k
}

pub fn hash_128(bytes: &[u8], seed: u64) -> u128 {
    let len = bytes.len();
    let mut h1 = seed;
    let mut h2 = seed;

    // Body
    for chunk in bytes.chunks_exact(16) {
        let k1 = u64::from_le_bytes(chunk[0..8].try_into().expect("oups"));
        let k2 = u64::from_le_bytes(chunk[8..16].try_into().expect("oups"));
        h1 ^= k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2);
        h1 = h1
            .rotate_left(27)
            .wrapping_add(h2)
            .wrapping_mul(5)
            .wrapping_add(C3);
        h2 ^= k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1);
        h2 = h2
            .rotate_left(31)
            .wrapping_add(h1)
            .wrapping_mul(5)
            .wrapping_add(C4);
    }

    // Tail
    let mut remainder = len % 16;
    let cursor = len - remainder;
    if remainder > 0 {
        if remainder > 8 {
            let mut tail: [u8; 8] = [0; 8];
            for (i, b) in bytes[cursor + 8..cursor + remainder].iter().enumerate() {
                tail[i] = *b;
            }
            let k2 = u64::from_le_bytes(tail);
            remainder = 8;
            h2 ^= k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1);
        }
        let mut tail: [u8; 8] = [0; 8];
        for (i, b) in bytes[cursor..cursor + remainder].iter().enumerate() {
            tail[i] = *b;
        }
        let k1 = u64::from_le_bytes(tail);
        h1 ^= k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2);
    }

    h1 ^= len as u64;
    h2 ^= len as u64;
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    h1 = fmix64(h1);
    h2 = fmix64(h2);

    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    (h2 as u128) << 64 | h1 as u128
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_string() {
        let result = hash_128("".as_bytes(), 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_16chars() {
        let result = hash_128("hello world!1234".as_bytes(), 0);
        assert_eq!(result, 240867822025444006610977441818626719586);
    }

    #[test]
    fn test_24chars() {
        let result = hash_128("hello world!123456789012".as_bytes(), 0);
        assert_eq!(result, 237085087629596244627904512008431960258);
    }

    #[test]
    fn test_foo() {
        let result = hash_128("foo".as_bytes(), 0);
        assert_eq!(result, 168394135621993849475852668931176482145);
    }

    #[test]
    fn test_seed() {
        let result = hash_128("foo".as_bytes(), 42);
        assert_eq!(result, 215966891540331383248189432718888555506);
    }

    #[test]
    fn test_fox() {
        let result = hash_128("The quick brown fox jumps over the lazy dog.".as_bytes(), 0);
        assert_eq!(result, 140055101589960098446263325149249471177);
    }

    #[test]
    fn test_lorem() {
        let result = hash_128("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".as_bytes(), 0);
        assert_eq!(result, 317940214952246513203822842115116886626);
    }
}
