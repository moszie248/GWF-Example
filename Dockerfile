# Stage 1: Build the Rust application
FROM rust:1-slim AS builder
WORKDIR /Users/morshmello/github-workflow
COPY . .
RUN cargo build --release

# Stage 2: Create a lightweight runtime image
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /Users/morshmello/demo-k/automate/helloworld/target/release/helloworld .
CMD ["./helloworld"]
