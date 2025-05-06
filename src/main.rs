use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            application_command::{ApplicationCommand},
            Interaction, InteractionResponseType,
        },
    },
    prelude::*,
};
use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(cmd) = interaction {
            if cmd.data.name == "ask" {
                // オプションからプロンプト取得
                let prompt = cmd.data.options
                    .get(0)
                    .and_then(|opt| opt.value.as_ref())
                    .and_then(|val| val.as_str())
                    .unwrap_or("");

                // ChatGPT API 呼び出し
                let answer = get_chatgpt_response(prompt).await
                    .unwrap_or_else(|e| format!("エラー: {}", e));

                // 応答
                if let Err(e) = cmd.create_interaction_response(&ctx.http, |resp| {
                    resp.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| m.content(answer))
                }).await {
                    eprintln!("レスポンスエラー: {:?}", e);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // グローバルコマンドとして登録
        let _commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name("ask")
                    .description("ChatGPTに質問します")
                    .create_option(|opt| {
                        opt.name("prompt")
                            .description("質問内容")
                            .kind(serenity::model::interactions::application_command::ApplicationCommandOptionType::String)
                            .required(true)
                    })
            })
        }).await;

        println!("スラッシュコマンド `/ask` を登録しました。");
    }
}

// OpenAI とのやり取り構造体
#[derive(Serialize)]
struct ChatGPTRequest { model: String, messages: Vec<ChatMessage> }
#[derive(Serialize)]
struct ChatMessage { role: String, content: String }
#[derive(Deserialize)]
struct ChatGPTResponse { choices: Vec<Choice> }
#[derive(Deserialize)]
struct Choice { message: ChatMessageContent }
#[derive(Deserialize)]
struct ChatMessageContent { content: String }

async fn get_chatgpt_response(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();
    let req = ChatGPTRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![ ChatMessage { role: "user".into(), content: prompt.into() } ],
    };
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&req)
        .send()
        .await?;
    let body: ChatGPTResponse = res.json().await?;
    Ok(body.choices.get(0)
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "応答がありませんでした".into()))
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN")
        .expect("`.env`にDISCORD_TOKENを設定してください");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES;
    let mut client = serenity::Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Botクライアントの作成に失敗しました");
    if let Err(e) = client.start().await {
        eprintln!("Bot起動エラー: {:?}", e);
    }
}