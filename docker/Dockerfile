FROM debian:stable as base

ARG TARGETPLATFORM

RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*

# quicklook-rs server
COPY target/x86_64-unknown-linux-gnu/release/quicklook-rs /usr/local/bin/quicklook-rs-amd64
COPY target/aarch64-unknown-linux-gnu/release/quicklook-rs /usr/local/bin/quicklook-rs-arm64
# quikclook cli
COPY target/x86_64-unknown-linux-gnu/release/cli /usr/local/bin/quicklook-cli-amd64
COPY target/aarch64-unknown-linux-gnu/release/cli /usr/local/bin/quicklook-cli-arm64

COPY docker/rename.sh /usr/local/bin/rename.sh
RUN chmod +x /usr/local/bin/rename.sh && rename.sh "$TARGETPLATFORM"

EXPOSE 3030

CMD ["quicklook-rs"]
