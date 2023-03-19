use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "message_type", content = "message_data")]
#[serde(rename_all = "snake_case")]
pub enum TBAData {
    Verification(Verification),
    Ping(Ping)
}

#[derive(Deserialize, Debug)]
pub struct Verification {
    pub verification_key: String
}

#[derive(Deserialize, Debug)]
pub struct Ping {
    pub title: String,
    pub desc: String
}
