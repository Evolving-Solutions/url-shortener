# Copyright (c) 2022 Evolving Software Corporation
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT


FROM evolvingsoftware/rust as rust

WORKDIR '/app'
# 2. Copy the binary to the local binary folder

COPY ./ ./

RUN apt-get install pkg-config -y

RUN apt-get install libssl-dev -y

RUN cargo build --release

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/url_shortener"]