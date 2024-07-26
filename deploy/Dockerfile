FROM rust:1.79 as builder
# Install required packages for cross-compilation and OpenSSL
RUN apt-get update && \
    apt-get install -y \
    musl-tools \
    musl-dev \
    libssl-dev \
    && apt-get clean
# Download and install pre-built musl-compatible OpenSSL binaries
RUN wget https://musl.cc/x86_64-linux-musl-cross.tgz && \
    tar -xzf x86_64-linux-musl-cross.tgz && \
    cp -r x86_64-linux-musl-cross/x86_64-linux-musl/ /usr/local/musl && \
    rm -rf x86_64-linux-musl-cross.tgz x86_64-linux-musl-cross
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/syntaxmakersserver
COPY ./server/. .
# Set environment variables for OpenSSL cross-compilation
ENV OPENSSL_DIR=/usr/local/musl \
    OPENSSL_LIB_DIR=/usr/local/musl/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/musl/include \
    PKG_CONFIG_ALLOW_CROSS=1 \
    PKG_CONFIG_PATH=/usr/local/musl/lib/pkgconfig \
    CC=musl-gcc \
    CFLAGS="-I/usr/local/musl/include" \
    LDFLAGS="-L/usr/local/musl/lib"
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN find / -name "syntaxmakers-server" 2>/dev/null

FROM debian:buster-slim
# Install OpenSSL runtime
RUN apt-get update && \
    apt-get install -y libssl1.1 && \
    apt-get clean
# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/syntaxmakersserver /usr/local/bin/syntaxmakersserver
WORKDIR /usr/local/bin/syntaxmakersserver
ENTRYPOINT ["./target/x86_64-unknown-linux-gnu/release/syntaxmakers-server"]