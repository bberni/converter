FROM rust:1.76.0 as builder

WORKDIR /converter

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM debian:12.5-slim

RUN apt-get update && apt-get install libssl-dev ca-certificates -y

COPY --from=builder  /converter/target/release/converter .

ENV EXCHANGERATE_API_KEY=""
RUN mkdir .cache
ENTRYPOINT [ "./converter" ]