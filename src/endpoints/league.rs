use crate::api_key::*;
use crate::dispatcher::*;
use crate::region::*;
use crate::types::*;

pub async fn get_league_entries(
    id: &str,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<Vec<LeagueEntry>, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/league/v4/entries/by-summoner/{}", &url, id);

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;

    let league_entrys: Vec<LeagueEntry> =
        serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(league_entrys)
}

pub async fn get_league_entries_for_division(
    division: Division,
    queue: Queue,
    tier: Tier,
    page: Option<u16>,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<Vec<LeagueEntry>, BlitzError> {
    let mut url = region.url();
    url = format!(
        "{}lol/league/v4/entries/{}/{}/{}?page={}",
        url,
        &queue,
        &tier,
        &division,
        page.unwrap_or(1)
    );

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;

    let league_entrys: Vec<LeagueEntry> =
        serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(league_entrys)
}
pub async fn get_grandmasterleagues(
    queue: Queue,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<Vec<LeagueEntry>, BlitzError> {
    let mut url = region.url();
    url = format!(
        "{}lol/league/v4/grandmasterleagues/by-queue/{}",
        &url, queue
    );

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;
    let league_entrys: Vec<LeagueEntry> =
        serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(league_entrys)
}

pub async fn get_masterleagues(
    queue: Queue,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<Vec<LeagueEntry>, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/league/v4/masterleagues/by-queue/{}", &url, queue);

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;
    let league_entrys: Vec<LeagueEntry> =
        serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(league_entrys)
}

pub async fn get_challengerleagues(
    queue: Queue,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<Vec<LeagueEntry>, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/league/v4/challengerleagues/by-queue/{}", &url, queue);

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;

    let league_entrys: Vec<LeagueEntry> =
        serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(league_entrys)
}

pub async fn get_league(
    leagueid: &String,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<LeagueList, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/league/v4/leagues/{}", &url, leagueid);

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;
    let league_list: LeagueList = serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(league_list)
}
