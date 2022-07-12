# Copyright (c) 2022 Evolving Software Corporation
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

FROM evolvingsoftware/rust as builder

RUN apt-get update


COPY ./ ./

RUN cargo install --path .

# Install MongoDB

FROM mongo:4.4-focal
COPY --from=builder ./ ./

# Default Command

CMD ["server"]
