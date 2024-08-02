FROM rust:1.79 AS builder
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*
COPY server/ /usr/src/syntaxmakersserver/server/
WORKDIR /usr/src/syntaxmakersserver/server
RUN cargo build --release
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs
COPY webclient/ /usr/src/syntaxmakersserver/webclient/
WORKDIR /usr/src/syntaxmakersserver/webclient
RUN npm install
RUN npm run build

FROM debian:stable-slim
RUN apt-get update && \
    apt-get install -y \
    nginx \
    libssl3 \
    curl \
    jq \
    && apt-get clean
COPY --from=builder /usr/src/syntaxmakersserver /usr/local/bin/syntaxmakersserver
RUN rm /etc/nginx/sites-enabled/default
EXPOSE 80 443
RUN mv /usr/local/bin/syntaxmakersserver/webclient/nginx.conf /etc/nginx/conf.d/nginx.conf
# RUN mv /usr/local/bin/syntaxmakersserver/server/.env.production /usr/local/bin/syntaxmakersserver/server/.env
WORKDIR /usr/local/bin/syntaxmakersserver/server
CMD ["sh", "-c", "nginx && ./target/release/syntaxmakers-server"]