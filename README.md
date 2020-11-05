# SocketAddr representation and conversion benchmarks

This crate is for benchmarking the change in internal representation of `Ipv4Addr`, `Ipv6Addr`,
`SocketAddrV4` and `SocketAddrV6` that is being discussed in [this internals thread].

[this internals thread]: https://internals.rust-lang.org/t/why-are-socketaddrv4-socketaddrv6-based-on-low-level-sockaddr-in-6/13321

## Usage

To print the memory usage of all the involved structs. So you can see how/if they change
with the other internal representation:

```bash
cargo run --bin print-mem
```

Spoiler: All sizes stay the same, except for `SocketAddrV4` that shrink from 16 to 6 bytes.
And some alignments become lower.

How to run the Criterion benchmarks in [benches/socketaddr-benchmark.rs](benches/socketaddr-benchmark.rs):

```bash
cargo +the_toolchain bench
```

Where `the_toolchain` is the Rust version you want to benchmark. I have built the latest master
at the time of writing this and applied my patches on top of it, built it again and ran the
benchmarks again.

Basically I did this:

```bash
cd $RUST_CHECKOUT
git checkout b1d9f31e043d0ba2782a4bb7416f18c4ba1c9044
./x.py build --stage 2
rustup toolchain link stage2 build/x86_64-unknown-linux-gnu/stage2

cd $THIS_BENCHMARK_REPO
cargo +stage2 bench

cd $RUST_CHECKOUT
git checkout simplify-socketaddr
./x.py build --stage 2

cd $THIS_BENCHMARK_REPO
cargo +stage2 bench
```

And then Criterion automatically shows regressions or performance improvements between runs.

