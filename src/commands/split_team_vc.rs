use serenity::all::{ButtonStyle, ChannelId, ChannelType, CreateActionRow, CreateInteractionResponseMessage, Guild, PartialChannel, ReactionType, ResolvedOption, ResolvedValue, User};
use serenity::builder::{CreateButton, CreateCommand, CreateCommandOption, CreateSelectMenu, CreateSelectMenuKind};
use serenity::model::application::CommandOptionType;

use rand::Rng;
use rand::seq::SliceRandom;
use serenity::builder::CreateActionRow::{Buttons, SelectMenu};
use serenity::cache::GuildRef;
use serenity::prelude::*;

pub const TEAM_A_IDENTIFIER: &str = "Team A: ";
pub const TEAM_B_IDENTIFIER: &str = "Team B: ";

pub fn register() -> CreateCommand{
    let command = CreateCommand::new("split_team_vc")
        .description("Split players into 2 teams from voice channel")
        .add_option(CreateCommandOption::new(CommandOptionType::Channel,
                                             "voice_channel",
                                             "voice channel to fetch users from")
            .required(false));
    command
}

pub fn run(options: &[ResolvedOption], guild: GuildRef, user: &User) -> CreateInteractionResponseMessage {
    let mut user_list:Vec<String> = Vec::new();
    let mut cur_channel_id: ChannelId;
    //fetch channel from option
    if let Some(ResolvedOption{ value: ResolvedValue::Channel(channel), ..}) = options.first(){
            if channel.kind != ChannelType::Voice {
                return CreateInteractionResponseMessage::new().content("Error: channel is not voice channel".to_string());
            }
            cur_channel_id = channel.id.clone();
            //fetch all users in the same channel
            guild.voice_states.iter()
            .filter(|(_, voice_state)|
                        voice_state.channel_id.is_some_and(|id| id == channel.id))
                .for_each(|(user_id, _)| user_list.push(user_id.mention().to_string()));
    }else{ //use the current user voice channel
        //find voice channel current user is in
         let voice_channel = guild.voice_states.iter()
            .find(|(user_id, _)| user_id == &&user.id)
            .map(|(_, voice_state)| voice_state.channel_id)
            .expect("user not found")
            .expect("user voice channel not found");
        cur_channel_id = voice_channel.clone();
        //fetch all users in the same channel
        guild.voice_states.iter()
            .filter(|(_, voice_state)|
                voice_state.channel_id.is_some_and(|id| id == voice_channel))
            .for_each(|(user_id, _)| user_list.push(user_id.mention().to_string()));
    }
    user_list.sort_unstable();
    user_list.dedup();
    let mut rng = rand::thread_rng();
    user_list.shuffle(&mut rng);
    let mut split_point = user_list.len()/2;
    if (user_list.len() % 2 == 1) && rng.gen_bool(0.5){  split_point += 1;  }
    let team_a: &[String] = &user_list[0..split_point];
    let team_b: &[String] = &user_list[split_point..];
    //setup buttons
    let mut response_components = vec![];
    //select team A channel, default to current one
    response_components.push(SelectMenu(
        CreateSelectMenu::new("team_a_channel", CreateSelectMenuKind::Channel{
            channel_types: Some(vec![ChannelType::Voice]),
            default_channels: Some(vec![cur_channel_id])
    }).placeholder("Team A Channel").min_values(1).max_values(1)));
    //select team B channel, default to next one
    //find the next channel
    let next_channel_iter = guild.channels.iter();
    let next_channel_id = next_channel_iter.filter(|(_, channel)| channel.kind == ChannelType::Voice)
        .cycle().skip_while(|(_, channel) | channel.id != cur_channel_id).skip(1).next().unwrap().0;
    response_components.push(SelectMenu(
        CreateSelectMenu::new("team_b_channel", CreateSelectMenuKind::Channel{
            channel_types: Some(vec![ChannelType::Voice]),
            default_channels: Some(vec![next_channel_id.clone()])
        }).placeholder("Team B Channel").min_values(1).max_values(1)));

    //operation buttons
    response_components.push(Buttons(vec![
        CreateButton::new("go_to_my_team").label("Go to my team").emoji(ReactionType::Unicode("\u{267F}".to_string())).style(ButtonStyle::Primary),
        CreateButton::new("move_all_players").label("Move all players (Admin)").emoji(ReactionType::Unicode("\u{1F464}".to_string())).style(ButtonStyle::Danger)
    ]));
    let response_content = format!("{TEAM_A_IDENTIFIER}{}\n{}\n\n{TEAM_B_IDENTIFIER}{}\n{}\nChange team VC channel:\r",  cur_channel_id.mention().to_string(),team_a.join("\n"), next_channel_id.mention().to_string() , team_b.join("\n")).to_string();
    let response = CreateInteractionResponseMessage::new()
        .content(response_content).components(response_components);
    response
}