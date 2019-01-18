# Factorio Blueprint Library

Specification found [here](https://wiki.factorio.com/Blueprint_string_format).

`cargo build --release`


To fuzz:

```bash
cargo install honggfuzz #requires additional steps see [here](https://crates.io/crates/honggfuzz)
cd test/hfuzz
cargo hfuzz run hfuzz_decode
```
