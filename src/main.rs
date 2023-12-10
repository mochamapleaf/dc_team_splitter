mod commands;

use serenity::all::{Command, ComponentInteractionDataKind, Ready, UserId};
use serenity::all::ChannelId;
use serenity::all::Route::Gateway;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::gateway::GatewayIntents;
use serenity::model::channel::Message;
//use serenity::model::gateway::Ready;

use log::{error, debug, info};

use serenity::model::application::Interaction;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::Permissions;
use crate::commands::split_team_vc::{TEAM_A_IDENTIFIER, TEAM_B_IDENTIFIER};

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
        match interaction {
            Interaction::Command(command) => {
                println!("command: {:#?}", command);
                let content: CreateInteractionResponseMessage = match command.data.name.as_str() {
                    "split_team" => commands::split_team::run(&command.data.options()),
                    "split_team_vc" => {
                        let guild_id = command.guild_id.expect("guild_id is None");
                        let guild = guild_id.to_guild_cached(&ctx.cache).expect("guild is None");
                        commands::split_team_vc::run(&command.data.options(), guild, &command.user)
                    },
                    _ => CreateInteractionResponseMessage::new().content("Unknown command".to_string()),
                };
                let builder = CreateInteractionResponse::Message(content);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {:?}", why);
                }
            },
            Interaction::Component(component) => {
                println!("{:#?}", component.data);
                match component.data.custom_id.as_str(){
                    "go_to_my_team" => {
                        //remain in vc if in team A
                        let message_ref = &component.message.content;
                        let team_b_start_i = message_ref.find(TEAM_B_IDENTIFIER)
                            .expect("TEAM_B_IDENTIFIER should be in message");
                        if let Some(i) = message_ref.find(component.user.id.mention().to_string().as_str()){
                            let mut to_channel_id: ChannelId;
                            if i > team_b_start_i{ //move player
                                //fetch team_b channel
                                let channel_id_start = message_ref[team_b_start_i..].find("<#").expect("channel mention should be in message");
                                let channel_id_end = message_ref[team_b_start_i+channel_id_start..].find(">").expect("channel mention should be in message") + team_b_start_i+channel_id_start;
                                to_channel_id = ChannelId::from(message_ref[team_b_start_i+channel_id_start+2..channel_id_end].parse::<u64>().expect("channel id should be u64"));
                            }else{ //fetch team_a channel
                                let channel_id_start = message_ref.find("<#").expect("channel mention should be in message");
                                let channel_id_end = message_ref[channel_id_start..].find(">").expect("channel mention should be in message") +channel_id_start;
                                to_channel_id = ChannelId::from(message_ref[channel_id_start+2..channel_id_end].parse::<u64>().expect("channel id should be u64"));
                            }
                            let guild_id = component.guild_id.expect("guild_id is None");
                            guild_id.move_member(&ctx.http, component.user.id, to_channel_id).await.expect("cannot move member");
                        }else{
                            //user not in any team, do nothing
                        }
                    },
                    "move_all_players" if component.member.as_ref()
                        .is_some_and(|member| member.permissions
                            .is_some_and(|permissions|
                                permissions.contains(Permissions::MOVE_MEMBERS))) => {
                        //check for admin rights above
                        let message_ref = &component.message.content;
                        let team_a_start_i = message_ref.find(TEAM_A_IDENTIFIER)
                            .expect("TEAM_A_IDENTIFIER should be in message");
                        let team_b_start_i = message_ref.find(TEAM_B_IDENTIFIER)
                            .expect("TEAM_B_IDENTIFIER should be in message");
                        let guild_id = component.guild_id.expect("guild_id is None");
                        //move team a players
                        //parse team_a channel
                        let channel_id_start = message_ref.find("<#").expect("channel mention should be in message");
                        let channel_id_end = message_ref[channel_id_start..].find(">").expect("channel mention should be in message") +channel_id_start;
                        let channel_a_id = ChannelId::from(message_ref[channel_id_start+2..channel_id_end].parse::<u64>().expect("channel id should be u64"));
                        let mut tmp_i = 0;
                            while let Some(user_start_i) = message_ref[tmp_i..team_b_start_i].find("<@") {
                                if let Some(user_end_i) = message_ref[tmp_i + user_start_i..].find(">") {
                                    let user_id = UserId::from(message_ref[tmp_i + user_start_i + 2..tmp_i +user_start_i+ user_end_i].parse::<u64>().expect("user id should be u64"));
                                    guild_id.move_member(&ctx.http, user_id, channel_a_id).await.expect("cannot move member");
                                    tmp_i += user_end_i;
                                }
                        }
                        //move team b players
                        let channel_id_start = message_ref[team_b_start_i..].find("<#").expect("channel mention should be in message");
                        let channel_id_end = message_ref[channel_id_start..].find(">").expect("channel mention should be in message") +channel_id_start;
                        let channel_b_id = ChannelId::from(message_ref[channel_id_start+2..channel_id_end].parse::<u64>().expect("channel id should be u64"));
                        let mut tmp_i = team_b_start_i;
                        while let Some(user_start_i) = message_ref[tmp_i..].find("<@") {
                            if let Some(user_end_i) = message_ref[tmp_i + user_start_i..].find(">") {
                                let user_id = UserId::from(message_ref[tmp_i + user_start_i + 2..tmp_i + user_start_i+ user_end_i].parse::<u64>().expect("user id should be u64"));
                                guild_id.move_member(&ctx.http, user_id, channel_b_id).await.expect("cannot move member");
                                tmp_i += user_end_i;
                            }
                        }
                    },
                    "team_a_channel" => {
                        let mut prev_content = component.message.content.clone();
                        let start_i = prev_content.find(TEAM_A_IDENTIFIER).expect("TEAM_A_IDENTIFIER should be in message") + TEAM_A_IDENTIFIER.len();
                        let end_i = prev_content[start_i..].find("\n").expect("newline should be in message") + start_i;
                        if let ComponentInteractionDataKind::ChannelSelect {values: selected_channels } = &component.data.kind{
                            prev_content.replace_range(start_i..end_i, selected_channels[0].mention().to_string().as_str());
                            let response = CreateInteractionResponseMessage::new()
                                .content(prev_content);
                            let response = CreateInteractionResponse::UpdateMessage(response);
                            if let Err(why) = component.create_response(&ctx.http, response).await {
                                println!("Cannot respond to component: {:?}", why);
                            }
                            return;
                        }
                    },
                    "team_b_channel" => {
                        let mut prev_content = component.message.content.clone();
                        let start_i = prev_content.find(TEAM_B_IDENTIFIER).expect("TEAM_B_IDENTIFIER should be in message") + TEAM_B_IDENTIFIER.len();
                        let end_i = prev_content[start_i..].find("\n").expect("newline should be in message") + start_i;
                        if let ComponentInteractionDataKind::ChannelSelect {values: selected_channels } = &component.data.kind{
                            prev_content.replace_range(start_i..end_i, selected_channels[0].mention().to_string().as_str());
                            let response = CreateInteractionResponseMessage::new()
                                .content(prev_content);
                            let response = CreateInteractionResponse::UpdateMessage(response);
                            if let Err(why) = component.create_response(&ctx.http, response).await {
                                println!("Cannot respond to component: {:?}", why);
                            }
                            return;
                        }
                    }
                    _ => {}
                }
                let response = CreateInteractionResponse::Acknowledge;
                if let Err(why) = component.create_response(&ctx.http, response).await {
                    println!("Cannot respond to component: {:?}", why);
                }
            }
            _ => {}
        }
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
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&dc_token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    println!("Bot is running...");

    if let Err(why)= client.start().await {
        println!("Client error: {:?}", why);
    }
}
