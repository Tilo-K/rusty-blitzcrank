use crate::region::*;
use crate::summoner_endpoint;
use crate::types::*;

pub struct Client {
    api_key: String,
}

impl Client {
    pub fn get_api_key(&self) -> &str {
        return &self.api_key;
    }

    pub fn get_summoner_by_name(
        &self,
        summoner_name: String,
        region: Region,
    ) -> Result<Summoner, summoner_endpoint::SummonerError> {
        return summoner_endpoint::get_summoner_by_name(summoner_name, region, &self.api_key);
    }

    pub fn get_summoner_by_accountid(
        &self,
        accountid: String,
        region: Region,
    ) -> Result<Summoner, summoner_endpoint::SummonerError> {
        return summoner_endpoint::get_summoner_by_accountid(accountid, region, &self.api_key);
    }
}

pub fn new(api_key: String) -> Client {
    let client = Client { api_key };

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
