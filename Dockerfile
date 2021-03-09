FROM rust:latest as build-env
ADD . /work
WORKDIR /work
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /work/target/release/actixexp /usr/local/bin/actixexp
CMD ["/usr/local/bin/actixexp"]
