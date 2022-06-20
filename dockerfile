# Copyright (c) 2022 Evolving Software Corporation
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

FROM rust:latest as builder
RUN cargo new --bin api-server

WORKDIR ./rust-server
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /server/target/release/server ./server
RUN apt update && apt install libpq-dev -y
CMD ["./server"]