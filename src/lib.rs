pub mod x64;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

const X64: &str = "x64";

#[pyfunction(key, seed = 0, arch = "X64", size = 128)]
fn hash(key: String, seed: u64, arch: &str, size: u8) -> PyResult<u128> {
    route_hash(key, seed, arch, size)
}

#[pyfunction(key, seed = 0, arch = "X64", size = 128)]
fn hash_bytes<'a>(
    py: Python<'a>,
    key: String,
    seed: u64,
    arch: &str,
    size: u8,
) -> PyResult<&'a PyBytes> {
    let hashed = route_hash(key, seed, arch, size)?.to_le_bytes();
    Ok(PyBytes::new(py, hashed.as_slice()))
}

#[pyfunction(key, seed = 0, arch = "X64", size = 128)]
fn hash_hex(key: String, seed: u64, arch: &str, size: u8) -> PyResult<String> {
    let hashed = route_hash(key, seed, arch, size)?;
    Ok(format!("{:x}", hashed))
}

#[inline]
fn route_hash(key: String, seed: u64, arch: &str, size: u8) -> PyResult<u128> {
    match (arch, size) {
        ("x64", 128) => Ok(x64::hash_128(key.as_bytes(), seed)),
        ("x64", 64) => Err(exceptions::PyValueError::new_err("Not Implemented yet.")),
        ("x86", 128) => Err(exceptions::PyValueError::new_err("Not Implemented yet.")),
        ("x86", 64) => Err(exceptions::PyValueError::new_err("Not Implemented yet.")),
        _ => Err(exceptions::PyValueError::new_err("Invalid input.")),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn murmurh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash, m)?)?;
    m.add_function(wrap_pyfunction!(hash_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(hash_hex, m)?)?;
    Ok(())
}
