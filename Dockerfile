# Stage 1: Build the Rust application
FROM rust:1-slim AS builder
COPY . .
RUN cargo build --release

# Stage 2: Create a lightweight runtime image
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /target/release/hello_world .
ENTRYPOINT ["./helloworld"]
