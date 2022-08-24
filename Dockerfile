# Copyright (c) 2022 Evolving Software Corporation
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT
FROM evolvingsoftware/rust as rust


RUN apt-get install pkg-config -y

RUN apt-get install libssl-dev -y


WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/myapp*

COPY . .

RUN cargo test

RUN cargo install --path .

RUN strip /root/.cargo/bin/url_shortener

# trim the binary

CMD ["/root/.cargo/bin/url_shortener"]

FROM alpine:latest

COPY --from=rust /root/.cargo/bin/url_shortener /root/.cargo/bin/url_shortener

CMD ["url_shortener"]