# Copyright (c) 2022 Evolving Software Corporation
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT
FROM evolvingsoftware/rust as rust

# RUN apt-get install pkg-config -y

# RUN apt-get install libssl-dev -y

WORKDIR ./usr/src/url_shortener

COPY Cargo.toml .

COPY Cargo.lock .

RUN mkdir src

RUN touch src/main.rs

RUN mkdir .cargo

COPY .cargo/config.toml .cargo/config.toml

RUN cargo vendor > .cargo/config.toml

COPY ./src ./src

RUN cargo test

RUN cargo install --path . --verbose

RUN strip /root/.cargo/bin/url_shortener

FROM evolvingsoftware/rust

COPY --from=rust /root/.cargo/bin/url_shortener /bin

RUN ls /bin

CMD ["url_shortener"]