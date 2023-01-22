#[path = "../src/lib.rs"]
mod murmurh;

use criterion::{criterion_group, criterion_main, Criterion};
use murmurh::x64::hash_128;

fn bench_hash_128(c: &mut Criterion) {
    c.bench_function("empty", |b| b.iter(|| hash_128("".as_bytes(), 0)));
    c.bench_function("fox", |b| {
        b.iter(|| hash_128("The quick brown fox jumps over the lazy dog.".as_bytes(), 0))
    });
    c.bench_function("lorem", |b| {
        b.iter(|| hash_128("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".as_bytes(), 0))
    });
}

criterion_group!(benches, bench_hash_128);
criterion_main!(benches);
