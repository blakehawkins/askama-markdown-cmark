default:
    just --list

example:
    cargo run --example example

check:
    cargo test
    cargo check
    cargo fmt
    cargo clippy

copy-doctest:
    cat src/lib.rs | grep "///" | cut -d'/' -f4- | cut -d' ' -f2- > README.md
