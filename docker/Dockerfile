FROM debian:stable as base

ARG TARGETPLATFORM
ARG PORT

RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# quicklook server
COPY target/x86_64-unknown-linux-gnu/release/quicklook-server quicklook-server-amd64
COPY target/aarch64-unknown-linux-gnu/release/quicklook-server quicklook-server-arm64
# quikclook cli
COPY target/x86_64-unknown-linux-gnu/release/quicklook-cli quicklook-cli-amd64
COPY target/aarch64-unknown-linux-gnu/release/quicklook-cli quicklook-cli-arm64

COPY docker/rename.sh rename.sh
RUN chmod +x rename.sh && rename.sh "$TARGETPLATFORM"

EXPOSE $PORT

CMD ["quicklook-server"]
