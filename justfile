check:
  cargo fmt -- --check
  cargo clippy --all-targets -- -D warnings

mine path:
  cargo run --example mining {{path}} --release

build:
  cargo build --release
