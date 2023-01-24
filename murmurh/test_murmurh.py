import murmurh
import pytest


def test_mmh3_128_x64_default_seed():
    # Correspond to seed 0
    assert (
        murmurh.hash("foo", arch="x64", size=128)
        == 168394135621993849475852668931176482145
    )


def test_mmh3_128_x64_default_arch():
    # Correspond to arch "x64"
    assert (
        murmurh.hash("foo", seed=0, size=128) == 168394135621993849475852668931176482145
    )


def test_mmh3_128_x64_default_size():
    # Correspond to size 128
    assert (
        murmurh.hash("foo", seed=0, arch="x64")
        == 168394135621993849475852668931176482145
    )


@pytest.mark.parametrize(
    "key,seed, arch, size, expected",
    [
        ("", 0, "x64", 128, 0),
        ("", 42, "x64", 128, 277815913556825370913473028741106730275),
        ("foo", 0, "x64", 128, 168394135621993849475852668931176482145),
        ("hello world!1234", 0, "x64", 128, 240867822025444006610977441818626719586),
        (
            "The quick brown fox jumps over the lazy dog.",
            0,
            "x64",
            128,
            140055101589960098446263325149249471177,
        ),
    ],
)
def test_hash(key: str, seed: int, arch: str, size: int, expected: str):
    assert murmurh.hash(key, seed=seed, arch=arch, size=size) == expected


@pytest.mark.parametrize(
    "test_input,expected",
    [
        ("", b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
    ],
)
def test_mmh3_128_x64_bytes(test_input: str, expected: bytes):
    assert murmurh.hash_bytes(test_input, 0) == expected
