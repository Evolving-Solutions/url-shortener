# Using Rust
FROM ubuntu:jammy
# Make sure it works.
CMD echo "Welcome to this Docker server. This application is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free"

# Update ubuntu
RUN apt-get update

RUN apt-get upgrade -y

# Install dependencies
RUN apt-get install build-essential -y \
        lld -y \
        clang -y\
        curl -y \
        wget -y \
        pkg-config -y \
        libssl-dev -y \
        libc6-dev -y

# Update the new packages
RUN apt-get update

# Install the latest version of rust.
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

RUN rustup target add x86_64-unknown-linux-gnu

# Check cargo is visible
RUN cargo --help