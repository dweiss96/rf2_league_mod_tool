# RF2 League Mod Creator

## Development
### Dependencies
Cargo Watch: `cargo install cargo-watch`

### Execution
One Time: `cargo run`
With Watcher: `cargo watch -c -x run`


### Code Quality
Test: `cargo test`

Fmt: `cargo fmt`

Lint: `cargo clippy`

fmt check mode: `cargo fmt --check`
strict ci LINT: `RUSTFLAGS="-D warnings" cargo clippy`