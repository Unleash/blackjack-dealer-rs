FROM lukemathwalker/cargo-chef:latest-rust-1.59 AS chef

WORKDIR app

FROM chef as planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

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

COPY --from=builder /app/target/release/blackjack-dealer-rs /app

RUN chown -R $APP_USER:$APP_USER /app/blackjack-dealer-rs

USER $APP_USER

CMD ["./blackjack-dealer-rs"]
