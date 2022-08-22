#!/bin/sh
# Installs rust and required build dependencies

# ToDo: Check if rust is installed
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
# Install Build Dependencies
sudo apt-get install lld clang -y