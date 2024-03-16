FROM rust:1.76.0

WORKDIR /converter

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM alpine:3.19.1

COPY --from=builder /converter/target/release/converter .

ENV EXCHANGERATE_API_KEY=""

CMD [ "./converter" ]

