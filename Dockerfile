FROM rust as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release --verbose --color always

FROM debian:buster-slim as cli

ENV DEV_MODE=false \
    SERVER_PORT=8080

COPY --from=builder /app/target/release/maximus /maximus

EXPOSE $SERVER_PORT/tcp

ENTRYPOINT ["./maximus"]