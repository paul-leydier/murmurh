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
