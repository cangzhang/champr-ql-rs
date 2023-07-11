FROM rust:1.70 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path . && cargo build --release

FROM debian:stable
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/champr-quick-look /usr/local/bin/champr-quick-look

CMD ["champr-quick-look"]
