mod commands;

use serenity::all::{Command, Ready};
use serenity::all::Route::Gateway;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::gateway::GatewayIntents;
use serenity::model::channel::Message;
//use serenity::model::gateway::Ready;

use serenity::model::application::Interaction;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready){
        println!("{} is connected!", ready.user.name);
        let guild_command =
            Command::create_global_command(&ctx.http, commands::split_team::register())
                .await;
        println!("guild_command: {:?}", guild_command);
        let guild_command =
            Command::create_global_command(&ctx.http, commands::split_team_vc::register())
                .await;
        println!("guild_command: {:?}", guild_command);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction){
        if let Interaction::Command(command) = interaction{
            println!("command: {:#?}", command);
            let content = match command.data.name.as_str(){
                "split_team" => commands::split_team::run(&command.data.options()),
                "split_team_vc" => {
                    let guild_id = command.guild_id.expect("guild_id is None");
                    let guild = guild_id.to_guild_cached(&ctx.cache).expect("guild is None");
                    commands::split_team_vc::run(&command.data.options(), guild, &command.user)
                },
                _ => "Unknown command".to_string(),
            };
            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await{
                println!("Cannot respond to slash command: {:?}", why);
            }

        }
        //is it possible to reach here???
    }

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
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
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
