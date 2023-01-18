import murmurh
import pytest


@pytest.mark.parametrize(
    "test_input,expected",
    [
        ("", 0),
    ],
)
def test_mmh3_128_x64(test_input: str, expected: str):
    assert murmurh.hash(test_input, 0) == expected


def test_mmh3_128_x64_default_seed():
    # Correspond to seed 0
    assert murmurh.hash("foo") == 168394135621993849475852668931176482145


@pytest.mark.parametrize(
    "test_input,expected",
    [
        ("", b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
    ],
)
def test_mmh3_128_x64_bytes(test_input: str, expected: bytes):
    assert murmurh.hash_bytes(test_input, 0) == expected
