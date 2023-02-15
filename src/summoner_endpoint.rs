use crate::dispatcher::*;
use crate::region::*;
use crate::types::*;

pub fn get_summoner_by_name(
    summoner_name: String,
    region: &Region,
    api_key: &str,
    wait_for_rate_limit: bool,
) -> Result<Summoner, BlitzError> {
    let mut url = region.url();
    url = format!(
        "{}lol/summoner/v4/summoners/by-name/{}",
        &url, summoner_name
    );

    let res = match dispatcher::get(url, api_key, wait_for_rate_limit) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let summoner: Summoner = match serde_json::from_str(&res) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    Ok(summoner)
}

pub fn get_summoner_by_accountid(
    accountid: String,
    region: &Region,
    api_key: &str,
    wait_for_rate_limit: bool,
) -> Result<Summoner, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/summoner/v4/summoners/by-account/{}", &url, accountid);

    let res = match dispatcher::get(url, api_key, wait_for_rate_limit) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let summoner: Summoner = match serde_json::from_str(&res) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    Ok(summoner)
}

pub fn get_summoner_by_puuid(
    puuid: String,
    region: &Region,
    api_key: &str,
    wait_for_rate_limit: bool,
) -> Result<Summoner, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/summoner/v4/summoners/by-puuid/{}", &url, puuid);

    let res = match dispatcher::get(url, api_key, wait_for_rate_limit) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let summoner: Summoner = match serde_json::from_str(&res) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    Ok(summoner)
}

pub fn get_summoner_by_id(
    id: String,
    region: &Region,
    api_key: &str,
    wait_for_rate_limit: bool,
) -> Result<Summoner, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/summoner/v4/summoners/{}", &url, id);

    let res = match dispatcher::get(url, api_key, wait_for_rate_limit) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let summoner: Summoner = match serde_json::from_str(&res) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    Ok(summoner)
}
