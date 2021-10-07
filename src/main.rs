use std::env;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;

const MAGIC_MESSAGE: &str = "Jorgen and Quint are working on something very secretive.";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!secret" {
            if let Err(e) = msg.channel_id.say(&ctx.http, MAGIC_MESSAGE).await {
                println!("Error sending message: {:?}!", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(e) = client.start().await {
        println!("Error creating client: {:?}!", e);
    }
}
