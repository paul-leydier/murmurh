mod x64;

use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyfunction]
#[pyo3(text_signature = "(key, /, seed = 0, arch = \"x64\", size = 128)")]
fn hash(key: String, seed: Option<u64>, arch: Option<String>, size: Option<u8>) -> u128 {
    route_hash(key, seed, arch, size)
}

#[pyfunction]
#[pyo3(text_signature = "(key, /, seed = 0, arch = \"x64\", size = 128)")]
fn hash_bytes(
    py: Python,
    key: String,
    seed: Option<u64>,
    arch: Option<String>,
    size: Option<u8>,
) -> &PyBytes {
    let hashed = route_hash(key, seed, arch, size).to_le_bytes();
    PyBytes::new(py, hashed.as_slice())
}

#[pyfunction]
#[pyo3(text_signature = "(key, /, seed = 0, arch = \"x64\", size = 128)")]
fn hash_hex(key: String, seed: Option<u64>, arch: Option<String>, size: Option<u8>) -> String {
    let hashed = route_hash(key, seed, arch, size);
    format!("{:x}", hashed)
}

#[inline]
fn route_hash(key: String, seed: Option<u64>, arch: Option<String>, size: Option<u8>) -> u128 {
    x64::hash_128(key.as_bytes(), seed.unwrap_or_default())
}

/// A Python module implemented in Rust.
#[pymodule]
fn murmurh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash, m)?)?;
    m.add_function(wrap_pyfunction!(hash_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(hash_hex, m)?)?;
    Ok(())
}
