use crate::api_key::*;
use crate::dispatcher::dispatcher;
use crate::region::*;
use crate::types::*;

pub fn get_champion_masteries(
    id: &str,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<Vec<ChampionMastery>, BlitzError> {
    let mut url = region.url();
    url = format!(
        "{}lol/champion-mastery/v4/champion-masteries/by-summoner/{}",
        &url, id
    );

    let res = match dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let masterys: Vec<ChampionMastery> = match serde_json::from_str(&res) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    Ok(masterys)
}

pub fn get_champion_mastery_for_champion(
    id: &str,
    champion_id: u16,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<ChampionMastery, BlitzError> {
    let mut url = region.url();
    url = format!(
        "{}lol/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}",
        &url, id, champion_id
    );

    let res = match dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let mastery: ChampionMastery = match serde_json::from_str(&res) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    Ok(mastery)
}

pub fn get_champion_mastery_top(
    id: &str,
    count: Option<u16>,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<Vec<ChampionMastery>, BlitzError> {
    let mut url = region.url();
    url = format!(
        "{}lol/champion-mastery/v4/champion-masteries/by-summoner/{}/top?count={}",
        &url,
        id,
        count.unwrap_or(3)
    );

    let res = match dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let mastery: Vec<ChampionMastery> = match serde_json::from_str(&res) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    Ok(mastery)
}

pub fn get_champion_mastery_score(
    id: &str,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<u64, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/champion-mastery/v4/scores/by-summoner/{}", &url, id);

    let res = match dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let masterys: u64 = match serde_json::from_str(&res) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    Ok(masterys)
}
