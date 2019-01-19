# Factorio Blueprint Library

Specification found [here](https://wiki.factorio.com/Blueprint_string_format).

`cd libblueprint; cargo build --release`


To fuzz:

```bash
./build.sh <harness>
ex: ./build.sh hfuzz_decode
```

Then in container:
```bash
cargo hfuzz run $harness
or with asan:
RUSTFLAGS="-Z sanitizer=address" cargo hfuzz run $harness
```