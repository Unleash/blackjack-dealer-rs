FROM lukemathwalker/cargo-chef:latest-rust-1.84 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release


FROM debian:bullseye-slim

LABEL org.opencontainers.image.description="Blackjack dealer for takehome assignment"

WORKDIR /app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata curl \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 1337

ENV TZ=Etc/UTC \
    APP_USER=blackjack

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER

COPY --from=builder /app/target/release/server /app

RUN chown -R $APP_USER:$APP_USER /app/server

USER $APP_USER

CMD ["./server"]
