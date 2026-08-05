set shell := ["powershell", "-c"]

default: test

fmt:
    cargo fmt

test:
    cargo test --workspace --all-targets

check-all: fmt
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace --all-targets