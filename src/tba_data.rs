use serde::Deserialize;

type TeamNumber = u16;
type Year = u16;

#[derive(Deserialize, Debug)]
#[serde(tag = "message_type", content = "message_data")]
#[serde(rename_all = "snake_case")]
pub enum TBAData {
    Verification(Verification),
    Ping(Ping),
    Broadcast(Broadcast),
    UpcomingMatch(UpcomingMatch),
    ScheduleUpdated(ScheduleUpdated),
}

// https://www.thebluealliance.com/apidocs/webhooks#upcoming_match
#[derive(Deserialize, Debug)]
pub struct UpcomingMatch {
    pub event_key: String,
    pub match_key: String,
    pub team_key: Option<String>,
    pub event_name: String,
    pub team_keys: [TeamNumber; 6],
    pub scheduled_time: u32,
    pub predicted_time: u32,
    pub webcast: Webcast,
}

#[derive(Deserialize, Debug)]
pub struct Webcast {
    #[serde(rename = "type")]
    pub webcast_type: String,
    pub channel: String,
    pub date: Option<String>,
    pub file: Option<String>,
}

// https://www.thebluealliance.com/apidocs/webhooks#match_score
// TODO

// https://www.thebluealliance.com/apidocs/webhooks#match_video
// TODO

// https://www.thebluealliance.com/apidocs/webhooks#starting_comp_level
// TODO

// https://www.thebluealliance.com/apidocs/webhooks#alliance_selection
// TODO

// https://www.thebluealliance.com/apidocs/webhooks#awards_posted
#[derive(Deserialize, Debug)]
pub struct AwardsPosted {
    pub event_key: String,
    pub team_key: Option<String>,
    pub event_name: String,
    pub awards: Vec<Award>,
}

#[derive(Deserialize, Debug)]
pub struct Award {
    pub name: String,
    pub award_type: u8,
    pub event_key: String,
    pub recipient_list: Vec<Recipient>,
}

#[derive(Deserialize, Debug)]
pub struct Recipient {
    pub team_key: Option<String>,
    pub awardee: Option<String>,
    pub year: Year,
}

// https://www.thebluealliance.com/apidocs/webhooks#schedule_updated
#[derive(Deserialize, Debug)]
pub struct ScheduleUpdated {
    pub event_key: String,
    pub event_name: String,
    pub first_match_time: Option<u32>,
}

// https://www.thebluealliance.com/apidocs/webhooks#ping
#[derive(Deserialize, Debug)]
pub struct Ping {
    pub title: String,
    #[serde(rename = "desc")]
    pub description: String,
}

// https://www.thebluealliance.com/apidocs/webhooks#broadcast
#[derive(Deserialize, Debug)]
pub struct Broadcast {
    pub title: String,
    #[serde(rename = "desc")]
    pub description: String,
    pub url: String,
}

// https://www.thebluealliance.com/apidocs/webhooks#verification
#[derive(Deserialize, Debug)]
pub struct Verification {
    pub verification_key: String,
}
