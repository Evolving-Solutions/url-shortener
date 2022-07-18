# Copyright (c) 2022 Evolving Software Corporation
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT


FROM evolvingsoftware/rust as rust

WORKDIR '/app'

RUN apt-get update

RUN cargo help

EXPOSE 8080

# 3. Build only the dependencies to cache them
RUN cargo build --release

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/server"]

