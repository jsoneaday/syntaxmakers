FROM rust:1.79 as builder
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/syntaxmakersserver
COPY ./server/. .

RUN cargo build --release
RUN echo "Searching for syntaxmakers-server..." && find / -name "syntaxmakers-server" 2>/dev/null

FROM debian:stable-slim
# Install OpenSSL runtime
RUN apt-get update && \
    apt-get install -y libssl3 && \
    apt-get clean
# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/syntaxmakersserver /usr/local/bin/syntaxmakersserver
WORKDIR /usr/local/bin/syntaxmakersserver
ENTRYPOINT ["./target/release/syntaxmakers-server"]