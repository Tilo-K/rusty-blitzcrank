use crate::api_key::*;
use crate::dispatcher::*;
use crate::region::*;
use crate::timeline_types;
use crate::types::*;

pub fn get_match_ids(
    puuid: &str,
    big_region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
    options: Option<GetMatchIdsOpts>,
) -> Result<Vec<String>, BlitzError> {
    if !big_region.is_big() {
        return Err(BlitzError::InvalidRegion);
    }

    let mut url = format!(
        "{}lol/match/v5/matches/by-puuid/{}/ids",
        big_region.url(),
        &puuid
    );

    if let Some(data) = options {
        let mut first_param = true;

        if let Some(param) = data.start_time {
            if first_param {
                url += "?";
                first_param = false;
            } else {
                url += "&";
            }

            url = format!("{}startTime={}", url, param);
        }

        if let Some(param) = data.end_time {
            if first_param {
                url += "?";
                first_param = false;
            } else {
                url += "&";
            }

            url = format!("{}endTime={}", url, param);
        }

        if let Some(param) = data.queue {
            if first_param {
                url += "?";
                first_param = false;
            } else {
                url += "&";
            }

            url = format!("{}queue={}", url, param);
        }

        if let Some(param) = data.game_type {
            if first_param {
                url += "?";
                first_param = false;
            } else {
                url += "&";
            }

            match param {
                GameType::NORMAL => url = format!("{}type=normal", url),
                GameType::RANKED => url = format!("{}type=ranked", url),
                GameType::TOURNEY => url = format!("{}type=tourney", url),
                GameType::TUTORIAL => url = format!("{}type=tutorial", url),
            }
        }

        if let Some(param) = data.start {
            if first_param {
                url += "?";
                first_param = false;
            } else {
                url += "&";
            }

            url = format!("{}start={}", url, param);
        }

        if let Some(param) = data.count {
            if first_param {
                url += "?";
            } else {
                url += "&";
            }

            if param > 0 && param <= 100 {
                url = format!("{}count={}", url, param);
            } else {
                url = format!("{}count={}", url, 20); // 20 is the default by riot.
            }
        }
    }

    let res = dispatcher::get(url, api_key, wait_for_rate_limit, big_region.get_endpoint())?;

    let history: Vec<String> = match serde_json::from_str(&res) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    return Ok(history);
}

pub fn get_match(
    id: &str,
    big_region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<Match, BlitzError> {
    if !big_region.is_big() {
        return Err(BlitzError::InvalidRegion);
    }

    let url = format!("{}lol/match/v5/matches/{}", big_region.url(), id);
    let match_str = dispatcher::get(url, api_key, wait_for_rate_limit, big_region.get_endpoint())?;

    let m: Match = match serde_json::from_str(&match_str) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    return Ok(m);
}

pub fn get_match_timeline(
    id: &str,
    big_region: &Region,
    api_key: &mut ApiKey,
    wait_for_rate_limit: bool,
) -> Result<timeline_types::MatchTimeline, BlitzError> {
    if !big_region.is_big() {
        return Err(BlitzError::InvalidRegion);
    }

    let url = format!("{}lol/match/v5/matches/{}/timeline", big_region.url(), id);
    let match_str = dispatcher::get(url, api_key, wait_for_rate_limit, big_region.get_endpoint())?;

    let m: timeline_types::MatchTimeline = match serde_json::from_str(&match_str) {
        Ok(d) => d,
        Err(_e) => return Err(BlitzError::BadJson),
    };

    return Ok(m);
}
