use serenity::all::{ChannelType, Guild, ResolvedOption, ResolvedValue, User};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

use rand::Rng;
use rand::seq::SliceRandom;
use serenity::cache::GuildRef;
use serenity::prelude::*;

pub fn register() -> CreateCommand{
    let mut command = CreateCommand::new("split_team_vc")
        .description("Split players into 2 teams from voice channel")
        .add_option(CreateCommandOption::new(CommandOptionType::Channel,
                                             "voice_channel",
                                             "voice channel to fetch users from")
            .required(false));
    command
}

pub fn run(options: &[ResolvedOption], guild: GuildRef, user: &User) -> String{
    let mut user_list:Vec<String> = Vec::new();
    //fetch channel from option
    if let Some(ResolvedOption{ value: ResolvedValue::Channel(channel), ..}) = options.first(){
            if channel.kind != ChannelType::Voice {
                return "Error: channel is not voice channel".to_string();
            }
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
    return format!("team A: \n{}\n\nteam B: \n{}\n", team_a.join("\n"), team_b.join("\n")).to_string();
}
