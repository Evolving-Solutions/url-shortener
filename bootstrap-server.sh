#!/bin/sh
sudo apt-get install lld clang
cargo install cargo-watch
RUST_BACKTRACE=1 RUST_LOG=actix_web=debug cargo watch -x check -x test -x clippy -x fmt -x run