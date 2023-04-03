# Build stage
FROM rust:1.67 as builder

COPY /src ./src
COPY Cargo.toml ./

RUN cargo build --release

# Prod stage
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/acn_r /
COPY .env ./
COPY appsettings.json ./

CMD ["./acn_r"]