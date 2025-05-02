# 1. ビルド用イメージ
FROM rust:1.67-slim AS builder
WORKDIR /usr/src/bot
# 必要なパッケージ
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
# ソースコピー
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
# 依存ダウンロード
RUN cargo fetch
# 本番コードコピー
COPY . .
# リリースビルド
RUN cargo build --release

# 2. 実行用イメージ
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# SSL証明書など
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/bot/target/release/discord_chatgpt_bot /app/
# 環境変数は `docker run -e` で渡す想定
CMD ["./discord_chatgpt_bot"]
