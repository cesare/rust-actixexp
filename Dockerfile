FROM rust:latest as build-env
ADD . /work
WORKDIR /work
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install musl-tools -y
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM busybox
COPY --from=build-env /work/target/x86_64-unknown-linux-musl/release/actixexp /usr/local/bin/actixexp
ENTRYPOINT ["/usr/local/bin/actixexp"]
