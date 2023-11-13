FROM debian:stable
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY target/aarch64-unknown-linux-gnu/release/quicklook-rs /usr/local/bin/quicklook-rs

EXPOSE 3030

CMD ["quicklook-rs"]
