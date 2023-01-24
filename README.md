# 💨 murmurh 💨
 
⚡️ Python bindings for a rust implementation of aapleby's MurMurHash.

## 💪 Contributing

Contributions are welcome! Please feel free to open issues and pull requests, and
we'll do our best to react in a timely manner.

To develop locally, you will need:
- A dedicated Python environment
- The `requirements.txt` python libraries installed

```shell
pip install -r requirements.txt
```

Once you performed changes to the rust source code, you can build the project using:
```shell
maturin develop
```

And you will then be able to run the python unit tests:
```shell
pytest
```

## ⏱️ Benchmarks

First, you should install `murmurh` using the release (optimised) build.
```shell
maturin develop --release
```

Then, you can use `timeit` to benchmark:
```shell
python -m timeit -s "import murmurh" "murmurh.hash('foo')"
```
> 1000000 loops, best of 5: 200 nsec per loop

```shell
python -m timeit -s "import mmh3" "mmh3.hash128('foo')"
```
> 2000000 loops, best of 5: 136 nsec per loop