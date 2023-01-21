
#[inline]
fn fmix32(mut k: u32) -> u32 {
    const C9: u32 = 0x85ebca6b;
    const C10: u32 = 0xc2b2ae35;
    const R1: u32 = 13;
    const R3: u32 = 16;
    k ^= k >> R3;
    k = k.wrapping_mul(C9);
    k ^= k >> R1;
    k = k.wrapping_mul(C10);
    k ^= k >> R3;
    k
}

pub fn hash_128(bytes: &[u8], seed: u32) -> u128 {
    const C1: u32 = 0x239b961b;
    const C2: u32 = 0xab0e9789;
    const C3: u32 = 0x38b34ae5;
    const C4: u32 = 0xa1e38b93;
    const C5: u32 = 0x561ccd1b;
    const C6: u32 = 0x0bcaa747;
    const C7: u32 = 0x96cd1c35;
    const C8: u32 = 0x32ac3b17;
    const R1: u32 = 13;
    const R2: u32 = 15;
    const R3: u32 = 16;
    const R4: u32 = 17;
    const R5: u32 = 18;
    const R6: u32 = 19;
    const M: u32 = 5;
    let len = bytes.len();
    let mut h1: u32 = seed;
    let mut h2: u32 = seed;
    let mut h3: u32 = seed;
    let mut h4: u32 = seed;

    // Body
    for chunk in bytes.chunks_exact(16) {
        let k1 = u32::from_le_bytes(chunk[0..4].try_into().expect(""));
        let k2 = u32::from_le_bytes(chunk[4..8].try_into().expect(""));
        let k3 = u32::from_le_bytes(chunk[8..12].try_into().expect(""));
        let k4 = u32::from_le_bytes(chunk[12..16].try_into().expect(""));
        h1 ^= k1.wrapping_mul(C1).rotate_left(R2).wrapping_mul(C2);
        h1 = h1
            .rotate_left(R6)
            .wrapping_add(h2)
            .wrapping_mul(M)
            .wrapping_add(C5);
        h2 ^= k2.wrapping_mul(C2).rotate_left(R3).wrapping_mul(C3);
        h2 = h2
            .rotate_left(R4)
            .wrapping_add(h3)
            .wrapping_mul(M)
            .wrapping_add(C6);
        h3 ^= k3.wrapping_mul(C3).rotate_left(R4).wrapping_mul(C4);
        h3 = h3
            .rotate_left(R2)
            .wrapping_add(h4)
            .wrapping_mul(M)
            .wrapping_add(C7);
        h4 ^= k4.wrapping_mul(C4).rotate_left(R5).wrapping_mul(C1);
        h4 = h4
            .rotate_left(R1)
            .wrapping_add(h1)
            .wrapping_mul(M)
            .wrapping_add(C8);
    }

    // Tail
    // TODO find a solution to match x64 implementation

    h1 ^= len as u32;
    h2 ^= len as u32;
    h3 ^= len as u32;
    h4 ^= len as u32;
    h1 = h1.wrapping_add(h2);
    h1 = h1.wrapping_add(h3);
    h1 = h1.wrapping_add(h4);
    h2 = h2.wrapping_add(h1);
    h3 = h3.wrapping_add(h1);
    h4 = h4.wrapping_add(h1);

    h1 = fmix32(h1);
    h2 = fmix32(h2);
    h3 = fmix32(h3);
    h4 = fmix32(h4);

    h1 = h1.wrapping_add(h2);
    h1 = h1.wrapping_add(h3);
    h1 = h1.wrapping_add(h4);
    h2 = h2.wrapping_add(h1);
    h3 = h3.wrapping_add(h1);
    h4 = h4.wrapping_add(h1);

    ((h4 as u128) << 96) | ((h3 as u128) << 64) | ((h2 as u128) << 32) | h1 as u128
}

