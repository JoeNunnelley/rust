FROM rust:1.31 AS build-env

COPY . .
RUN cargo build 
ENTRYPOINT [ './target/debug/amortize' ]
