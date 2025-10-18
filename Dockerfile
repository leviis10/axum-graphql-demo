FROM rust:1.90.0-slim-trixie AS builder
WORKDIR /app
COPY ./app .
RUN cargo build --release

FROM alpine:3.22.2
WORKDIR /app
COPY --from=builder /app/target/release/app .
CMD ["./app"]