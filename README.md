### Rust implementation

Because why not?

## Steps

- Build the executable either for production `cargo build --release` or for debug `cargo build`.
- If built for release, run: `cat input.txt | ./target/release/rust-compute 100 5000`
- if built for debug, run: `cat input.txt | cargo run -- 100 5000`
