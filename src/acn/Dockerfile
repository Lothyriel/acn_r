# Build stage
FROM rust:1.70 as builder

COPY lib lib
COPY acn/src ./src
COPY acn/Cargo.toml ./

RUN apt-get update
RUN apt-get install -y cmake
RUN cargo build --release --bin=acn

# Prod stage
FROM gcr.io/distroless/cc
COPY --from=builder /target/release/acn /
COPY appsettings_prd.json .

ENTRYPOINT ["./acn"]
