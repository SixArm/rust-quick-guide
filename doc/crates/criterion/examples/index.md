# Criterion crate - example

Add to your file `Cargo.toml`:

```toml
[dev-dependencies]
criterion = { version = "0.4", features = ["html_reports"] }

[bench]
name = "my_benchmark"
harness = false
```

Create a file `$PROJECT/benches/demo.rs` with this code:

```rust
use criterion::{
    black_box, criterion_group, criterion_main, Criterion
};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "fib 20",
        |b| b.iter(|| fibonacci(black_box(20)))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

Run this benchmark with `cargo bench`. You should see output like below.

```txt
fib 20   time:   [26.029 us 26.251 us 26.505 us]
Found 11 outliers among 99 measurements (11.11%)
  6 (6.06%) high mild
  5 (5.05%) high severe
```
