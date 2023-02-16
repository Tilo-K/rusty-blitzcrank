use crate::api_key::*;
use crate::matches_endpoint;
use crate::region::*;
use crate::summoner_endpoint;
use crate::timeline_types;
use crate::types::*;

pub struct Client {
    api_key: ApiKey,
    wait_for_rate_limit: bool,
}

impl Client {
    pub fn get_api_key(&self) -> &ApiKey {
        return &self.api_key;
    }

    pub fn set_wait_for_rate_limit(&mut self, set_to: bool) {
        self.wait_for_rate_limit = set_to;
    }

    pub fn get_summoner_by_name(
        &mut self,
        summoner_name: &str,
        region: &Region,
    ) -> Result<Summoner, BlitzError> {
        return summoner_endpoint::get_summoner_by_name(
            summoner_name,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_summoner_by_accountid(
        &mut self,
        accountid: &str,
        region: &Region,
    ) -> Result<Summoner, BlitzError> {
        return summoner_endpoint::get_summoner_by_accountid(
            accountid,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_summoner_by_puuid(
        &mut self,
        puuid: &str,
        region: &Region,
    ) -> Result<Summoner, BlitzError> {
        return summoner_endpoint::get_summoner_by_puuid(
            puuid,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_summoner_by_id(
        &mut self,
        id: &str,
        region: &Region,
    ) -> Result<Summoner, BlitzError> {
        return summoner_endpoint::get_summoner_by_id(
            id,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_match_ids(
        &mut self,
        puuid: &str,
        big_region: &Region,
        options: Option<GetMatchIdsOpts>,
    ) -> Result<Vec<String>, BlitzError> {
        return matches_endpoint::get_match_ids(
            puuid,
            big_region,
            &mut self.api_key,
            self.wait_for_rate_limit,
            options,
        );
    }

    pub fn get_match(&mut self, id: &str, big_region: &Region) -> Result<Match, BlitzError> {
        return matches_endpoint::get_match(
            id,
            big_region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_match_timeline(
        &mut self,
        id: &str,
        big_region: &Region,
    ) -> Result<timeline_types::MatchTimeline, BlitzError> {
        return matches_endpoint::get_match_timeline(
            id,
            big_region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }
}

pub fn new(api_key: String) -> Client {
    let key = ApiKey::new(api_key);

    let client = Client {
        api_key: key,
        wait_for_rate_limit: false,
    };

    return client;
}

pub fn new_custom_rate(api_key: String, l1: u16, l1p: u16, l2: u16, l2p: u16) -> Client {
    let key = ApiKey::new_custom_rate(api_key, l1, l1p, l2, l2p);

    let client = Client {
        api_key: key,
        wait_for_rate_limit: false,
    };

    return client;
}
