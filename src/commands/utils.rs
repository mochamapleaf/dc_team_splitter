use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use serenity::all::{ChannelId, Mentionable, parse_channel_mention, parse_user_mention, Team, UserId};

struct TeamSplitInfo{
    teams: Vec<(ChannelId, Vec<UserId>)>,
}
impl Debug for TeamSplitInfo{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, team) in self.teams.iter().enumerate() {
            f.write_fmt(format_args!("Team {}: {}\n", i+1, team.0.mention().to_string()))?;
            for uid in team.1.iter(){
               f.write_fmt(format_args!("{}\n", uid.mention().to_string()))?;
            }
            if team.1.len() == 0{ f.write_str("*Empty*")?; }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum TeamSplitInfoParseError{
    InvalidMessage,
}

impl FromStr for TeamSplitInfo{
    type Err = TeamSplitInfoParseError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        //don't need to deal with <@! as the messages are generated by code
        let mut buf = TeamSplitInfo{ teams: Vec::new() };
        let mut i = 1;
        while let Some(team_start_i) = s.find("Team "){
            let channel_start_i = team_start_i + s[team_start_i..].find("<#")
                .ok_or(TeamSplitInfoParseError::InvalidMessage)?;
            let channel_end_i = channel_start_i + s[channel_start_i..].find(">")
                .ok_or(TeamSplitInfoParseError::InvalidMessage)?;
            let channel_id = parse_channel_mention(&s[channel_start_i..=channel_end_i])
                .ok_or(TeamSplitInfoParseError::InvalidMessage)?;
            buf.teams.push((channel_id, Vec::new()));
            let mut mention_end_i = channel_end_i;
            while let Some(tmp_i) =  s[mention_end_i..].find("<"){
                let mention_start_i = mention_end_i + tmp_i;
                if s[mention_start_i..].starts_with("<#"){ break; }
                mention_end_i = mention_start_i + s[mention_start_i..].find(">")
                    .ok_or(TeamSplitInfoParseError::InvalidMessage)?;
                let user_id = parse_user_mention(&s[mention_start_i..=mention_end_i]).
                    ok_or(TeamSplitInfoParseError::InvalidMessage)?;
                buf.teams.last_mut().unwrap().1.push(user_id);
            }
            s = &s[mention_end_i..];
        }
        Ok(buf)
    }
}

#[test]
fn test_team_split_info(){
    let test_message = r#"Team 1: <#1145474363580227614>
<@538349814678280053>
<@1087471104772707936>
<@348743215073280063>
<@878644414873250583>
<@1189442167712100337>
<@758244016470220903>

Team 2: <#1173815870401822751>
<@640085555518223330>
<@426215555514843332>
<@487705555513693338>
<@1014755555157933378>
<@1130455555117633325>

"#;
    let info = TeamSplitInfo::from_str(test_message).unwrap();
    assert_eq!(format!("{:?}", info), test_message);
}