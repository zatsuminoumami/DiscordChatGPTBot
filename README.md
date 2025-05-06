# Discord ChatGPT Bot (Rust + Serenity)

## 概要
Rust と Serenity を使い、Discord のスラッシュコマンド `/ask` で ChatGPT API に質問できる Bot です。  
Docker コンテナ化済みで、どこでも同じ環境で動かせます。

- Rust製（安全・高速）
- Dockerでどこでも起動可能
- スラッシュコマンド自動登録
- `.env` による簡易設定

---

## セットアップ

1. リポジトリをクローン  
   ```bash
   git@github.com:zatsuminoumami/DiscordChatGPTBot.git
   ```

2. 環境変数を設定（.env ファイルを作成）
    ```bash
    DISCORD_TOKEN=xxxxxxxxxxxxxxxxxxxxxxxxxxxx
    OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxxxxxx
    ```
このファイルは Docker 起動時に自動読み込みされます。

3. 必要ライブラリインストール
    Rust ツールチェインが入っていれば特別な準備不要です。

## ローカル実行
ビルド＆起動
```bash
cargo run --release
```
起動後、以下のログが表示されれば成功です。
```bash
chatGPTbot is connected!
スラッシュコマンド `/ask` を登録しました。
```
スラッシュコマンドが反映されるまで数分かかることがあります。

## Docker 実行

1. イメージビルド
```bash
docker build -t discord_chatgpt_bot:latest .
```

2. コンテナ起動
```bash
docker run -d --env-file .env --name gpt-bot chatgpt-discord-bot
```

3. ログを確認
```bash
docker logs -f gpt-bot
```
正常に起動すれば、Discord側で /ask が使えるようになります。

## カスタマイズ例
- コマンドやレスポンスのロギング
- 会話履歴をDB（SQLite/PostgreSQL）に保存
- エラーハンドリング強化・再試行ロジック

## ライセンス
MIT