FROM rust:1 AS build-env
WORKDIR /app
COPY ./crates ./crates
COPY ./Cargo.toml .
RUN cargo build --release --no-default-features --features docker
RUN chmod +x ./target/release/sync

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/release/sync /sync

CMD ["/sync"]
