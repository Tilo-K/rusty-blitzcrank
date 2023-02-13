use crate::dispatcher::*;
use crate::region::*;
use crate::types::types::*;

#[derive(Debug)]
pub enum SummonerError {
    SummonerNotFound,
}

pub fn get_summoner_by_name(
    summoner_name: String,
    region: Region,
    api_key: &str,
) -> Result<Summoner, SummonerError> {
    let mut url = region.url();
    url = format!(
        "{}lol/summoner/v4/summoners/by-name/{}",
        &url, summoner_name
    );

    let res = dispatcher::get(url, api_key).expect("Error loading JSON");
    let summoner: Summoner = serde_json::from_str(&res).expect("Error parsing JSON");

    Ok(summoner)
}

pub fn get_summoner_by_accountid(
    accountid: String,
    region: Region,
    api_key: &str,
) -> Result<Summoner, SummonerError> {
    let mut url = region.url();
    url = format!("{}lol/summoner/v4/summoners/by-account/{}", &url, accountid);

    let res = dispatcher::get(url, api_key).expect("Error loading JSON");
    let summoner: Summoner = serde_json::from_str(&res).expect("Error parsing JSON");

    Ok(summoner)
}
