use serenity::all::{ResolvedOption, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

use rand::Rng;
use rand::seq::SliceRandom;

pub fn register() -> CreateCommand{
    let mut command = CreateCommand::new("split_team")
        .description("Split players into 2 teams")
        .add_option(CreateCommandOption::new(CommandOptionType::String,
                                             "user_list",
                                             "list of users")
            .required(true));
    command
}

pub fn run(options: &[ResolvedOption]) -> String{
    let mut user_list:Vec<&str> = Vec::new();
    if let Some(ResolvedOption{ value: ResolvedValue::String(user_str), ..}) = options.first(){
        user_str.split(" ").for_each(|user_str| user_list.push(user_str.trim()));
        user_list.sort_unstable();
        user_list.dedup();
        let mut rng = rand::thread_rng();
        user_list.shuffle(&mut rng);
        let mut split_point = user_list.len()/2;
        if (user_list.len() % 2 == 1) && rng.gen_bool(0.5){  split_point += 1;  }
        let team_a: &[&str] = &user_list[0..split_point];
        let team_b: &[&str] = &user_list[split_point..];
        return format!("team A: \n{}\n\nteam B: \n{}\n", team_a.join("\n"), team_b.join("\n")).to_string();
    }
    return format!("user_list: {:?}", user_list).to_string();
}
