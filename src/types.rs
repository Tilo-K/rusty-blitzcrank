use serde_derive::{Deserialize, Serialize};
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
