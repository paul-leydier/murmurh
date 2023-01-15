import murmurh
import pytest


@pytest.mark.parametrize(
    "test_input,expected",
    [
        ("", "0"),
    ],
)
def test_mmh3_128_x64(test_input: str, expected: str):
    assert murmurh.mmh3_128_x64(test_input, 0) == expected


@pytest.mark.parametrize(
    "test_input,expected",
    [
        ("", b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
    ],
)
def test_mmh3_128_x64_bytes(test_input: str, expected: bytes):
    assert murmurh.mmh3_128_x64_bytes(test_input, 0) == expected
