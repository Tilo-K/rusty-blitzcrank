pub mod types {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Summoner {
        id: String,
        account_id: String,
        puuid: String,
        name: String,
        profile_icon_id: u32,
        revision_date: i64,
        summoner_level: u32,
    }
}
