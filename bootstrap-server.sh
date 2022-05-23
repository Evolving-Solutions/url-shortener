#!/bin/sh
cd server
cargo install cargo-watch
cargo watch -x 'run --bin server'