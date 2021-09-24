FROM rust:1.55 as builder
WORKDIR /src/yelb-appserver
COPY . .
RUN cargo build --release

FROM debian:stable-slim
COPY --from=builder /src/yelb-appserver/target/release/yelb-appserver /usr/local/bin/yelb-appserver

ENV YELB_DB_SERVER_ENDPOINT=yelb-db \
    YELB_DB_DATABASE=yelbdatabase \
    YELB_DB_SERVER_PORT=5432 \
    YELB_DB_USER=postgres \
    YELB_DB_PASS=postgres_password \    
    REDIS_SERVER_ENDPOINT=redis-server \
    REDIS_SERVER_PORT=6379 \
    PORT=4567 \
    RUST_LOG=error

CMD ["yelb-appserver"]