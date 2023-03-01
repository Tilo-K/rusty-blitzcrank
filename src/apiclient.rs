use crate::api_key::*;
use crate::endpoints::league;
use crate::endpoints::mastery;
use crate::endpoints::matches;
use crate::endpoints::spectator;
use crate::endpoints::status;
use crate::endpoints::summoner;
use crate::region::*;
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
        return summoner::get_summoner_by_name(
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
        return summoner::get_summoner_by_accountid(
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
        return summoner::get_summoner_by_puuid(
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
        return summoner::get_summoner_by_id(
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
        return matches::get_match_ids(
            puuid,
            big_region,
            &mut self.api_key,
            self.wait_for_rate_limit,
            options,
        );
    }

    pub fn get_match(&mut self, id: &str, big_region: &Region) -> Result<Match, BlitzError> {
        return matches::get_match(id, big_region, &mut self.api_key, self.wait_for_rate_limit);
    }

    pub fn get_match_timeline(
        &mut self,
        id: &str,
        big_region: &Region,
    ) -> Result<timeline_types::MatchTimeline, BlitzError> {
        return matches::get_match_timeline(
            id,
            big_region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_champion_masteries(
        &mut self,
        id: &str,
        region: &Region,
    ) -> Result<Vec<ChampionMastery>, BlitzError> {
        return mastery::get_champion_masteries(
            id,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_champion_mastery_for_champion(
        &mut self,
        id: &str,
        champion_id: u16,
        region: &Region,
    ) -> Result<ChampionMastery, BlitzError> {
        return mastery::get_champion_mastery_for_champion(
            id,
            champion_id,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_champion_mastery_top(
        &mut self,
        id: &str,
        count: Option<u16>,
        region: &Region,
    ) -> Result<Vec<ChampionMastery>, BlitzError> {
        return mastery::get_champion_mastery_top(
            id,
            count,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_champion_mastery_score(
        &mut self,
        id: &str,
        region: &Region,
    ) -> Result<u64, BlitzError> {
        return mastery::get_champion_mastery_score(
            id,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_league_entries(
        &mut self,
        id: &str,
        region: &Region,
    ) -> Result<Vec<LeagueEntry>, BlitzError> {
        return league::get_league_entries(id, region, &mut self.api_key, self.wait_for_rate_limit);
    }

    pub fn get_league_entries_for_division(
        &mut self,
        division: Division,
        queue: Queue,
        tier: Tier,
        page: Option<u16>,
        region: &Region,
    ) -> Result<Vec<LeagueEntry>, BlitzError> {
        return league::get_league_entries_for_division(
            division,
            queue,
            tier,
            page,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_grandmasterleagues(
        &mut self,
        queue: Queue,
        region: &Region,
    ) -> Result<Vec<LeagueEntry>, BlitzError> {
        return league::get_grandmasterleagues(
            queue,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_challengerleagues(
        &mut self,
        queue: Queue,
        region: &Region,
    ) -> Result<Vec<LeagueEntry>, BlitzError> {
        return league::get_challengerleagues(
            queue,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_masterleagues(
        &mut self,
        queue: Queue,
        region: &Region,
    ) -> Result<Vec<LeagueEntry>, BlitzError> {
        return league::get_masterleagues(
            queue,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_league(
        &mut self,
        leagueid: &String,
        region: &Region,
    ) -> Result<LeagueList, BlitzError> {
        return league::get_league(
            leagueid,
            region,
            &mut self.api_key,
            self.wait_for_rate_limit,
        );
    }

    pub fn get_active_game(
        &mut self,
        id: &str,
        region: &Region,
    ) -> Result<CurrentGameInfo, BlitzError> {
        return spectator::get_active_game(id, region, &mut self.api_key, self.wait_for_rate_limit);
    }

    pub fn get_featured_games(&mut self, region: &Region) -> Result<FeaturedGames, BlitzError> {
        return spectator::get_featured_games(region, &mut self.api_key, self.wait_for_rate_limit);
    }

    pub fn get_platform_data(&mut self, region: &Region) -> Result<PlatformData, BlitzError> {
        return status::get_platform_data(region, &mut self.api_key, self.wait_for_rate_limit);
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
