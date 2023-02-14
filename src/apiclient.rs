use crate::matches_endpoint;
use crate::region::*;
use crate::summoner_endpoint;
use crate::types::*;

pub struct Client {
    api_key: String,
    wait_for_rate_limit: bool,
}

impl Client {
    pub fn get_api_key(&self) -> &str {
        return &self.api_key;
    }

    pub fn set_wait_for_rate_limit(&mut self, set_to: bool) {
        self.wait_for_rate_limit = set_to;
    }

    pub fn get_summoner_by_name(
        &self,
        summoner_name: String,
        region: Region,
    ) -> Result<Summoner, BlitzError> {
        return summoner_endpoint::get_summoner_by_name(
            summoner_name,
            region,
            &self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_summoner_by_accountid(
        &self,
        accountid: String,
        region: Region,
    ) -> Result<Summoner, BlitzError> {
        return summoner_endpoint::get_summoner_by_accountid(
            accountid,
            region,
            &self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_summoner_by_puuid(
        &self,
        puuid: String,
        region: Region,
    ) -> Result<Summoner, BlitzError> {
        return summoner_endpoint::get_summoner_by_puuid(
            puuid,
            region,
            &self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_summoner_by_id(&self, id: String, region: Region) -> Result<Summoner, BlitzError> {
        return summoner_endpoint::get_summoner_by_id(
            id,
            region,
            &self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_match_ids(
        &self,
        puuid: &str,
        region: Region,
        options: Option<GetMatchIdsOpts>,
    ) -> Result<Vec<String>, BlitzError> {
        return matches_endpoint::get_match_ids(
            puuid,
            region,
            &self.api_key,
            self.wait_for_rate_limit,
            options,
        );
    }
}

pub fn new(api_key: String) -> Client {
    let client = Client {
        api_key,
        wait_for_rate_limit: false,
    };

    return client;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_client() {
        let c = new("<UNIT_TEST_API_KEY>".to_owned());
        assert!(c.get_api_key().eq("<UNIT_TEST_API_KEY>"));
    }
}
