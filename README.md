# SIMD experiment

## Running

From this repository's root:

```sh
(cd scalar && cargo run --release -- <path to data file> <letter to count>)
(cd auto   && cargo run --release -- <path to data file> <letter to count>)
(cd simd   && cargo run --release -- <path to data file> <letter to count>)
```

For test data, try `enwik8` or `enwik9` from
<https://mattmahoney.net/dc/textdata.html>.

## Build settings

`auto` and `simd` use Cargo's default build settings. The only special flags are
in `scalar/.cargo/config.toml`:

```toml
[build]
rustflags = ["-C", "no-vectorize-loops", "-C", "no-vectorize-slp"]
```

These disable LLVM's automatic loop and SLP vectorization for the scalar project
and its dependencies.

## Inspecting assembly

```sh
(cd scalar && cargo rustc --release -p shared -- --emit=asm)
(cd auto   && cargo rustc --release -p shared -- --emit=asm)
(cd simd   && cargo rustc --release -- --emit=asm)
```

The outputs will be:

- `shared-*.s` under `scalar/target/release/deps/` and
  `auto/target/release/deps/`
- `simd-*.s` under `simd/target/release/deps/`.
