FROM rust:1.84 AS builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libsqlite3-dev libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
CMD ["backend"]

