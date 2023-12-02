use serenity::all::Ready;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::gateway::GatewayIntents;
use serenity::model::channel::Message;
//use serenity::model::gateway::Ready;


struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
#[tokio::main]
async fn main() {
    let dc_token = std::env::var("DISCORD_TOKEN").expect("Expected a discord token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&dc_token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    println!("Bot is running...");

    if let Err(why)= client.start().await {
        println!("Client error: {:?}", why);
    }
}
