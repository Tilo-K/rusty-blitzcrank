pub mod apiclient {
    use crate::region::region::*;
    use crate::summoner_endpoint;
    use crate::types::types::*;

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
        ) -> Result<Summoner, summoner_endpoint::summoner::SummonerError> {
            return summoner_endpoint::summoner::get_summoner_by_name(
                summoner_name,
                region,
                &self.api_key,
            );
        }
    }

    pub fn new(api_key: String) -> Client {
        let client = Client { api_key };

        return client;
    }
}

#[cfg(test)]
mod tests {
    use super::apiclient;

    #[test]
    fn create_client() {
        let c = apiclient::new("<UNIT_TEST_API_KEY>".to_owned());
        assert!(c.get_api_key().eq("<UNIT_TEST_API_KEY>"));
    }
}
