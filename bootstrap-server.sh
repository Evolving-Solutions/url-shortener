#!/bin/sh
cd server
cargo install cargo-watch
RUST_BACKTRACE=1 RUST_LOG=actix_web=debug cargo watch -x 'run --bin server'