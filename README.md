# Discord ChatGPT Bot (Rust + Serenity)

## 概要
Rust と Serenity を使い、Discord のスラッシュコマンド `/ask` で ChatGPT API に質問できる Bot です。  
Docker コンテナ化済みで、どこでも同じ環境で動かせます。

---

## 準備

1. リポジトリをクローン  
   ```bash
   git clone https://github.com/your-org/discord_chatgpt_bot.git
   cd discord_chatgpt_bot
   ```

2. 環境変数を設定（.env ファイルを作成）
    ```bash
    DISCORD_TOKEN=あなたのDiscordBotトークン
    OPENAI_API_KEY=あなたのOpenAI APIキー
    ```

3. 必要ライブラリインストール
    Rust ツールチェインが入っていれば特別な準備不要です。

## ローカル実行

```bash
cargo run --release
```
Bot が起動し、スラッシュコマンド /ask を Discord 上に登録します（反映に数分かかる場合あり）。

## Docker 実行

1. イメージビルド
```bash
docker build -t discord_chatgpt_bot:latest .
```

2. コンテナ起動
```bash
docker run -d \
  -e DISCORD_TOKEN=あなたのDiscordBotトークン \
  -e OPENAI_API_KEY=あなたのOpenAI APIキー \
  --name chatgpt_bot \
  discord_chatgpt_bot:latest
  ```
## カスタマイズ例
- コマンドやレスポンスのロギング
- 会話履歴をDB（SQLite/PostgreSQL）に保存
- エラーハンドリング強化・再試行ロジック

## ライセンス
MIT