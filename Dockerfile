FROM debian:buster AS builder

RUN apt-get update && apt-get install -y curl build-essential openssl libssl-dev pkg-config

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly-2020-12-21

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM debian:buster

RUN apt-get update && apt-get install -y openssl libssl-dev

COPY --from=builder \
  /target/release/yukikaze \
  /usr/local/bin/

ENV PORT 8080

WORKDIR /root
CMD ROCKET_PORT=$PORT /usr/local/bin/yukikaze
