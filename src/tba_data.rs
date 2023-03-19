use serde::Deserialize;

type TeamNumber = u16;

#[derive(Deserialize, Debug)]
#[serde(tag = "message_type", content = "message_data")]
#[serde(rename_all = "snake_case")]
pub enum TBAData {
    Verification(Verification),
    Ping(Ping),
    Broadcast(Broadcast),
}

#[derive(Deserialize, Debug)]
pub struct Verification {
    pub verification_key: String
}

#[derive(Deserialize, Debug)]
pub struct Ping {
    pub title: String,
    #[serde(rename = "desc")]
    pub description: String
}

#[derive(Deserialize, Debug)]
pub struct Broadcast {
    pub title: String,
    #[serde(rename = "desc")]
    pub description: String,
    pub url: String
}


#[derive(Deserialize, Debug)]
pub struct UpcomingMatch {
    pub event_key: String,
    pub match_key: String,
    pub team_key: Option<String>,
    pub event_name: String,
    pub team_keys: [TeamNumber; 6],
    pub scheduled_time: u32,
    pub predicted_time: u32,
    pub webcast: Webcast
}

#[derive(Deserialize, Debug)]
pub struct Webcast {
    #[serde(rename = "type")]
    pub webcast_type: String,
    pub channel: String,
    pub date: Option<String>,
    pub file: Option<String>,
}
