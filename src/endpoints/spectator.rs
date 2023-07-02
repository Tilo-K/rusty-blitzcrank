use crate::api_key::*;
use crate::dispatcher::*;
use crate::region::*;
use crate::types::*;

pub async fn get_active_game(
    id: &str,
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<CurrentGameInfo, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/spectator/v4/active-games/by-summoner/{}", &url, id);

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;
    let curr_game: CurrentGameInfo = serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(curr_game)
}

pub async fn get_featured_games(
    region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<FeaturedGames, BlitzError> {
    let mut url = region.url();
    url = format!("{}lol/spectator/v4/featured-games", &url);

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, region.get_endpoint()).await?;
    let featured_games: FeaturedGames =
        serde_json::from_str(&res).map_err(|_| BlitzError::BadJson)?;

    Ok(featured_games)
}
