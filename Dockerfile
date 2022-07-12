# Copyright (c) 2022 Evolving Software Corporation
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

FROM evolvingsoftware/rust

COPY ./ ./

EXPOSE 5665:8080

RUN cargo install --path .

# Default Command
CMD ["server"]