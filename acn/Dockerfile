# Build stage
FROM rust:1.70 as builder

COPY /src ./src
COPY Cargo.toml ./

RUN apt-get update && apt-get install -y cmake && cargo build --release

# Prod stage
FROM gcr.io/distroless/cc
COPY --from=builder /target/release/acn /
COPY ../../appsettings_prd.json ./

ENTRYPOINT ["./acn"]
