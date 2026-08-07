set shell := ["powershell", "-c"]

default: check-all

fmt:
    cargo fmt

test:
    cargo test --workspace --all-targets

check-all: fmt
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace --all-targets

build-release:
    cargo build --workspace --release

clean:
    cargo clean