FROM debian:stable as base

ARG TARGETPLATFORM

RUN apt-get update & apt-get install -y extra-runtime-dependencies pkg-config libssl-dev libpq-dev & rm -rf /var/lib/apt/lists/*

COPY target/x86_64-unknown-linux-gnu/release/quicklook-rs /usr/local/bin/quicklook-rs-amd64
COPY target/aarch64-unknown-linux-gnu/release/quicklook-rs /usr/local/bin/quicklook-rs-arm64

COPY target/x86_64-unknown-linux-gnu/release/cli /usr/local/bin/quicklook-cli-amd64
COPY target/aarch64-unknown-linux-gnu/release/cli /usr/local/bin/quicklook-cli-arm64

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
        mv /usr/local/bin/quicklook-rs-amd64 /usr/local/bin/quicklook-rs; mv /usr/local/bin/quicklook-cli-amd64 /usr/local/bin/quicklook-cli; \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
        mv /usr/local/bin/quicklook-rs-arm64 /usr/local/bin/quicklook-rs; mv /usr/local/bin/quicklook-cli-arm64 /usr/local/bin/quicklook-cli; \
    else \
        echo "Unsupported platform" && exit 1; \
    fi && \
    rm -f /usr/local/bin/*-arm64 /usr/local/bin/*-amd64;

EXPOSE 3030

CMD ["quicklook-rs"]
