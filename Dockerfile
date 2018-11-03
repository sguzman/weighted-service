FROM alpine:latest AS base
RUN mkdir app
WORKDIR ./app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN apk add --no-cache libgcc openssl-dev rust cargo
RUN cargo build --release --jobs 2 --verbose

ADD . src
RUN cargo build --package weighted-consumer --bin weighted-consumer --verbose --jobs 2 --all-features --release .
RUN mv ./target/release/weighted-consumer /root

FROM alpine
COPY --from=base /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=base /app/target/release/weighted-consumer /main

ENTRYPOINT ["/main"]