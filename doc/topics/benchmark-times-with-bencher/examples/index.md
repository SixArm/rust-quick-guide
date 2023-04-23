# Benchmark times - example

Edit file `Cargo.toml`, and add a dependency and configuration:

```toml
[dev-dependencies]
bencher = "*"

[[bench]]
name = "example"
harness = false
```

Create a project top-level directory `benches` then edit file `benches/example.rs`:

```rust
#[macro_use]
extern crate bencher;
use bencher::Bencher;

fn a(bench: &mut Bencher) {
    bench.iter(|| {
        (0..1000).fold(0, |x, y| x + y)
    })
}

fn b(bench: &mut Bencher) {
    const N: usize = 1024;
    bench.iter(|| {
        vec![0u8; N]
    });
    bench.bytes = N as u64;
}

benchmark_group!(benches, a, b);
benchmark_main!(benches);
```

Run `cargo bench` and you should see output such as:

```text
running 2 tests
test a ... bench:           0 ns/iter (+/- 0)
test b ... bench:          31 ns/iter (+/- 1) = 33032 MB/s
```
