# Build stage
FROM rust:1.76 as builder

COPY src ./src
COPY Cargo.toml ./

RUN apt-get update
RUN apt-get install -y cmake
RUN cargo build --release

# Prod stage
FROM debian:stable-slim
COPY --from=builder /target/release/acn /
COPY appsettings_prd.json .

ENTRYPOINT ["./acn"]
