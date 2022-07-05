FROM jo3mccain/rusty as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --verbose --color always

FROM debian:buster-slim as cli

ENV DEV_MODE=false \
    SERVER_PORT=8080

COPY --from=builder /project/target/release/maximus /maximus

EXPOSE $SERVER_PORT/tcp

ENTRYPOINT ["./maximus"]