FROM rust:1.70 as builder
WORKDIR /usr/src/syntaxmakersserver
COPY . .
RUN cargo install --debug --path .
 
FROM debian:bullseye
RUN apt-get update
RUN apt-get install -y wget
RUN apt-get install -y build-essential
COPY --from=builder /usr/src/syntaxmakersserver /usr/local/bin/syntaxmakersserver
WORKDIR /usr/local/bin/syntaxmakersserver
ENTRYPOINT [ "./target/debug/syntaxmakers-server" ]