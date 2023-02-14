use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub enum BlitzError {
    RateLimited,
    SummonerNotFound,
    BadJson,
    BadStatuscode(u16),
    RequestError(Option<String>),
    Forbidden,
    NotFound,
    InvalidRegion,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Summoner {
    pub id: String,
    pub accountId: String,
    pub puuid: String,
    pub name: String,
    pub profileIconId: u32,
    pub revisionDate: i64,
    pub summonerLevel: u32,
}

pub enum GameType {
    RANKED,
    NORMAL,
    TOURNEY,
    TUTORIAL,
}
pub struct GetMatchIdsOpts {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub queue: Option<i16>,
    pub game_type: Option<GameType>,
    pub start: Option<i16>,
    pub count: Option<u8>,
}
