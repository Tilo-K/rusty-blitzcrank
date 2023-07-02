use crate::api_key::*;
use crate::dispatcher::dispatcher;
use crate::region::*;
use crate::types::*;

pub async fn get_champion_masteries(
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

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;

    let masterys: Vec<ChampionMastery> =
        serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(masterys)
}

pub async fn get_champion_mastery_for_champion(
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

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;
    let mastery: ChampionMastery = serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(mastery)
}

pub async fn get_champion_mastery_top(
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

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;
    let mastery: Vec<ChampionMastery> =
        serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(mastery)
}

pub async fn get_champion_mastery_score(
    id: &str,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<u64, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/champion-mastery/v4/scores/by-summoner/{}", &url, id);

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;
    let masterys: u64 = serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(masterys)
}
