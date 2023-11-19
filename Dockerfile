FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN apt update & apt install libpq-dev & rm -rf /var/lib/apt/lists/*
RUN cargo build --release -p cli -p quicklook-rs

FROM rust:latest as runtime
RUN apt update & apt install libpq-dev & rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/quicklook-rs /usr/local/bin/quicklook-rs
COPY --from=builder /app/target/release/cli /usr/local/bin/quicklook-cli

EXPOSE 3030

CMD ["quicklook-rs"]
