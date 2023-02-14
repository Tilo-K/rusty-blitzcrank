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
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Summoner {
    id: String,
    accountId: String,
    puuid: String,
    name: String,
    profileIconId: u32,
    revisionDate: i64,
    summonerLevel: u32,
}
