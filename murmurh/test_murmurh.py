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
    "key,seed, arch, size, expected_int, expected_bytes",
    [
        (
            "",
            0,
            "x64",
            128,
            0,
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ),
        (
            "",
            42,
            "x64",
            128,
            277815913556825370913473028741106730275,
            b"#\x85\x1b\xfa}\xa7*\xf0\xb9\xcb\x11\xda\x10f\x01\xd1",
        ),
        (
            "foo",
            0,
            "x64",
            128,
            168394135621993849475852668931176482145,
            b"aE\xf5\x01W\x86q\xe2\x87}\xba+\xe4\x87\xaf~",
        ),
        (
            "hello world!1234",
            0,
            "x64",
            128,
            240867822025444006610977441818626719586,
            b"b\x7f\xe3;\xfdN\xf6\xc0\xadl\x18\xf2\x81t5\xb5",
        ),
        (
            "The quick brown fox jumps over the lazy dog.",
            0,
            "x64",
            128,
            140055101589960098446263325149249471177,
            b"\xc9\x02\xe9\x9e\x1fH\x99\xcd\xe7\xb6\x87\x89\xa3\xa1]i",
        ),
    ],
)
def test_hash(
    key: str, seed: int, arch: str, size: int, expected_int: int, expected_bytes: bytes
):
    assert murmurh.hash(key, seed=seed, arch=arch, size=size) == expected_int
    assert murmurh.hash_bytes(key, seed=seed, arch=arch, size=size) == expected_bytes
