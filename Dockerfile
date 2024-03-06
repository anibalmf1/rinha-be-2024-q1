FROM rust:1.76.0 as builder
WORKDIR /usr/src/nilapi
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo install --path .
FROM debian:bookworm
COPY --from=builder /usr/local/cargo/bin/nilapi /usr/local/bin/nilapi
EXPOSE 8080
CMD ["nilapi"]