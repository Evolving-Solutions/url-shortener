# Copyright (c) 2022 Evolving Software Corporation
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT


FROM evolvingsoftware/rust as rust

WORKDIR '/app'
# 2. Copy the binary to the local binary folder
COPY ./target/release/url_shortener /usr/local/bin/url_shortener
# When `docker run` is executed, launch the binary!
CMD ["url_shortener"]