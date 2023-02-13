#[allow(non_snake_case)]
pub mod types {
    use serde_derive::{Deserialize, Serialize};

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
}
