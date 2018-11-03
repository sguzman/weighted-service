FROM liuchong/rustup:musl AS base
RUN mkdir app
WORKDIR ./app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup install nightly
ADD src src

ARG name=weighted-consumer
RUN cargo build --package $name --bin $name --verbose --jobs 2 --all-features --release --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=base /root/app/target/x86_64-unknown-linux-musl/release/weighted-consumer /main

ENTRYPOINT ["/main"]