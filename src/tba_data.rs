use serde::Deserialize;
use serde_json::Value;

type Year = u16;

#[derive(Deserialize, Debug)]
#[serde(tag = "message_type", content = "message_data")]
#[serde(rename_all = "snake_case")]
pub enum TBAData {
    UpcomingMatch(UpcomingMatch),
    ScheduleUpdated(ScheduleUpdated),
    MatchScore(MatchScore),
    MatchVideo(MatchVideo),
    StartingCompLevel(StartingCompLevel),
    AllianceSelection(AllianceSelection),
    AwardsPosted(AwardsPosted),
    Ping(Ping),
    Broadcast(Broadcast),
    Verification(Verification),
}

// https://www.thebluealliance.com/apidocs/webhooks#upcoming_match
#[derive(Deserialize, Debug)]
pub struct UpcomingMatch {
    pub event_key: String,
    pub match_key: String,
    pub team_key: Option<String>,
    pub event_name: String,
    pub team_keys: Vec<String>,
    pub scheduled_time: i64,
    pub predicted_time: i64,
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
#[derive(Deserialize, Debug)]
pub struct MatchScore {
    pub event_key: String,
    pub match_key: String,
    pub team_key: Option<String>,
    pub event_name: String,
    #[serde(rename = "match")]
    pub m_match: Match,
}

#[derive(Deserialize, Debug)]
pub struct Match {
    pub key: String,
    pub comp_level: String,
    pub set_number: u8,
    pub match_number: u8,
    pub alliances: Option<MatchAlliances>,
    pub winning_alliance: Option<String>,
    pub event_key: String,
    pub time: Option<i64>,
    pub actual_time: Option<i64>,
    pub predicted_time: Option<i64>,
    pub post_result_time: Option<i64>,
    // Took this long to avoid using Value, but here we are
    pub score_breakdown: Value,
    pub videos: Option<Vec<Video>>,
}

#[derive(Deserialize, Debug)]
pub struct MatchAlliances {
    pub red: MatchAlliance,
    pub blue: MatchAlliance,
}

#[derive(Deserialize, Debug)]
pub struct MatchAlliance {
    // This is really frustrating to encode, exclusively becase it can be -1 or
    // null if the match isn't plays
    pub score: Option<i32>,
    pub team_keys: Vec<String>,
    pub surrogate_team_keys: Option<Vec<String>>,
    pub dq_team_keys: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Video {
    #[serde(rename = "type")]
    pub video_type: String,
    pub key: String,
}

// https://www.thebluealliance.com/apidocs/webhooks#match_video
#[derive(Deserialize, Debug)]
pub struct MatchVideo {
    pub event_key: String,
    pub match_key: String,
    pub team_key: Option<String>,
    pub event_name: String,
    pub m_match: Match,
}

// https://www.thebluealliance.com/apidocs/webhooks#starting_comp_level
#[derive(Deserialize, Debug)]
pub struct StartingCompLevel {
    pub event_name: String,
    pub comp_level: String,
    pub event_key: String,
    pub scheduled_time: Option<i64>
}

// https://www.thebluealliance.com/apidocs/webhooks#alliance_selection
#[derive(Deserialize, Debug)]
pub struct AllianceSelection {
    pub event_key: String,
    pub event_name: String,
    pub team_key: Option<String>,
    pub event: AllianceSelectionEvent
}

#[derive(Deserialize, Debug)]
pub struct AllianceSelectionEvent {
    pub alliances: Vec<AllianceSelectionAlliance>,
}

#[derive(Deserialize, Debug)]
pub struct AllianceSelectionAlliance {
    pub declines: Vec<String>,
    pub picks: Vec<String>,
}

// https://www.thebluealliance.com/apidocs/webhooks#awards
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
