use serenity::all::{CreateInteractionResponseMessage, ResolvedOption, ResolvedValue, UserId};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

use rand::Rng;
use rand::seq::SliceRandom;
use serenity::prelude::Mentionable;

pub fn register() -> CreateCommand{
    let command = CreateCommand::new("split_team")
        .description("Split players into 2 teams")
        .add_option(CreateCommandOption::new(CommandOptionType::String,
                                             "user_list",
                                             "list of users")
            .required(true));
    command
}

pub fn run(options: &[ResolvedOption]) -> CreateInteractionResponseMessage{
    let mut user_list:Vec<String> = Vec::new();
    if let Some(ResolvedOption{ value: ResolvedValue::String(user_str), ..}) = options.first(){
        user_str.split("<@").for_each(|sub_str| {
            if let Some(end_index) = sub_str.find(">"){
                if let Ok(id) = sub_str[..end_index].parse::<u64>(){
                    user_list.push(UserId::new(id).mention().to_string() );
                }
            }
        });
        user_list.sort_unstable();
        user_list.dedup();
        let mut rng = rand::thread_rng();
        user_list.shuffle(&mut rng);
        let mut split_point = user_list.len()/2;
        if (user_list.len() % 2 == 1) && rng.gen_bool(0.5){  split_point += 1;  }
        let team_a: &[String] = &user_list[0..split_point];
        let team_b: &[String] = &user_list[split_point..];
        return CreateInteractionResponseMessage::new().content(format!("team A: \n{}\n\nteam B: \n{}\n", team_a.join("\n"), team_b.join("\n")).to_string());
    }
    return CreateInteractionResponseMessage::new().content("Error: empty user_list");
}
