FROM debian:buster AS builder

RUN apt-get update && apt-get install -y curl build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly-2021-10-11

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM debian:buster

COPY --from=builder /target/release/yukikaze /usr/local/bin/

ENV PORT=8080
ENV RUST_LOG="info"

WORKDIR /root
CMD PORT=$PORT /usr/local/bin/yukikaze
