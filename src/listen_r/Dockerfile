# Build stage
FROM rust:1.70 as builder

COPY lib lib
COPY listen_r/src ./src
COPY listen_r/Cargo.toml ./

RUN apt-get update
RUN apt-get install -y cmake
RUN cargo build --release --bin=listen_r

# Prod stage
FROM gcr.io/distroless/cc
COPY --from=builder /target/release/listen_r /
COPY appsettings_prd.json .

ENTRYPOINT ["./listen_r"]
