use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub enum BlitzError {
    RateLimited,
    SummonerNotFound,
    BadJson,
    BadStatuscode(u16),
    RequestError(Option<String>),
    Forbidden,
    NotFound,
    InvalidRegion,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Summoner {
    pub id: String,

    #[serde(rename = "accountId")]
    pub account_id: String,

    pub puuid: String,
    pub name: String,

    #[serde(rename = "profileIconId")]
    pub profile_icon_id: u32,

    #[serde(rename = "revisionDate")]
    pub revision_date: i64,

    #[serde(rename = "summonerLevel")]
    pub summoner_level: u32,
}

pub enum GameType {
    RANKED,
    NORMAL,
    TOURNEY,
    TUTORIAL,
}
pub struct GetMatchIdsOpts {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub queue: Option<i16>,
    pub game_type: Option<GameType>,
    pub start: Option<i16>,
    pub count: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    #[serde(rename = "metadata")]
    pub metadata: Metadata,

    #[serde(rename = "info")]
    pub info: Info,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "gameCreation")]
    pub game_creation: i64,

    #[serde(rename = "gameDuration")]
    pub game_duration: i64,

    #[serde(rename = "gameEndTimestamp")]
    pub game_end_timestamp: i64,

    #[serde(rename = "gameId")]
    pub game_id: i64,

    #[serde(rename = "gameMode")]
    pub game_mode: String,

    #[serde(rename = "gameName")]
    pub game_name: String,

    #[serde(rename = "gameStartTimestamp")]
    pub game_start_timestamp: i64,

    #[serde(rename = "gameType")]
    pub game_type: String,

    #[serde(rename = "gameVersion")]
    pub game_version: String,

    #[serde(rename = "mapId")]
    pub map_id: i64,

    #[serde(rename = "participants")]
    pub participants: Vec<Participant>,

    #[serde(rename = "platformId")]
    pub platform_id: String,

    #[serde(rename = "queueId")]
    pub queue_id: i64,

    #[serde(rename = "teams")]
    pub teams: Vec<Team>,

    #[serde(rename = "tournamentCode")]
    pub tournament_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "allInPings")]
    pub all_in_pings: i64,

    #[serde(rename = "assistMePings")]
    pub assist_me_pings: i64,

    #[serde(rename = "assists")]
    pub assists: i64,

    #[serde(rename = "baitPings")]
    pub bait_pings: i64,

    #[serde(rename = "baronKills")]
    pub baron_kills: i64,

    #[serde(rename = "basicPings")]
    pub basic_pings: i64,

    #[serde(rename = "bountyLevel")]
    pub bounty_level: i64,

    #[serde(rename = "challenges")]
    pub challenges: HashMap<String, f64>,

    #[serde(rename = "champExperience")]
    pub champ_experience: i64,

    #[serde(rename = "champLevel")]
    pub champ_level: i64,

    #[serde(rename = "championId")]
    pub champion_id: i64,

    #[serde(rename = "championName")]
    pub champion_name: String,

    #[serde(rename = "championTransform")]
    pub champion_transform: i64,

    #[serde(rename = "commandPings")]
    pub command_pings: i64,

    #[serde(rename = "consumablesPurchased")]
    pub consumables_purchased: i64,

    #[serde(rename = "damageDealtToBuildings")]
    pub damage_dealt_to_buildings: i64,

    #[serde(rename = "damageDealtToObjectives")]
    pub damage_dealt_to_objectives: i64,

    #[serde(rename = "damageDealtToTurrets")]
    pub damage_dealt_to_turrets: i64,

    #[serde(rename = "damageSelfMitigated")]
    pub damage_self_mitigated: i64,

    #[serde(rename = "dangerPings")]
    pub danger_pings: i64,

    #[serde(rename = "deaths")]
    pub deaths: i64,

    #[serde(rename = "detectorWardsPlaced")]
    pub detector_wards_placed: i64,

    #[serde(rename = "doubleKills")]
    pub double_kills: i64,

    #[serde(rename = "dragonKills")]
    pub dragon_kills: i64,

    #[serde(rename = "eligibleForProgression")]
    pub eligible_for_progression: bool,

    #[serde(rename = "enemyMissingPings")]
    pub enemy_missing_pings: i64,

    #[serde(rename = "enemyVisionPings")]
    pub enemy_vision_pings: i64,

    #[serde(rename = "firstBloodAssist")]
    pub first_blood_assist: bool,

    #[serde(rename = "firstBloodKill")]
    pub first_blood_kill: bool,

    #[serde(rename = "firstTowerAssist")]
    pub first_tower_assist: bool,

    #[serde(rename = "firstTowerKill")]
    pub first_tower_kill: bool,

    #[serde(rename = "gameEndedInEarlySurrender")]
    pub game_ended_in_early_surrender: bool,

    #[serde(rename = "gameEndedInSurrender")]
    pub game_ended_in_surrender: bool,

    #[serde(rename = "getBackPings")]
    pub get_back_pings: i64,

    #[serde(rename = "goldEarned")]
    pub gold_earned: i64,

    #[serde(rename = "goldSpent")]
    pub gold_spent: i64,

    #[serde(rename = "holdPings")]
    pub hold_pings: i64,

    #[serde(rename = "individualPosition")]
    pub individual_position: String,

    #[serde(rename = "inhibitorKills")]
    pub inhibitor_kills: i64,

    #[serde(rename = "inhibitorTakedowns")]
    pub inhibitor_takedowns: i64,

    #[serde(rename = "inhibitorsLost")]
    pub inhibitors_lost: i64,

    #[serde(rename = "item0")]
    pub item0: i64,

    #[serde(rename = "item1")]
    pub item1: i64,

    #[serde(rename = "item2")]
    pub item2: i64,

    #[serde(rename = "item3")]
    pub item3: i64,

    #[serde(rename = "item4")]
    pub item4: i64,

    #[serde(rename = "item5")]
    pub item5: i64,

    #[serde(rename = "item6")]
    pub item6: i64,

    #[serde(rename = "itemsPurchased")]
    pub items_purchased: i64,

    #[serde(rename = "killingSprees")]
    pub killing_sprees: i64,

    #[serde(rename = "kills")]
    pub kills: i64,

    #[serde(rename = "lane")]
    pub lane: String,

    #[serde(rename = "largestCriticalStrike")]
    pub largest_critical_strike: i64,

    #[serde(rename = "largestKillingSpree")]
    pub largest_killing_spree: i64,

    #[serde(rename = "largestMultiKill")]
    pub largest_multi_kill: i64,

    #[serde(rename = "longestTimeSpentLiving")]
    pub longest_time_spent_living: i64,

    #[serde(rename = "magicDamageDealt")]
    pub magic_damage_dealt: i64,

    #[serde(rename = "magicDamageDealtToChampions")]
    pub magic_damage_dealt_to_champions: i64,

    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,

    #[serde(rename = "needVisionPings")]
    pub need_vision_pings: i64,

    #[serde(rename = "neutralMinionsKilled")]
    pub neutral_minions_killed: i64,

    #[serde(rename = "nexusKills")]
    pub nexus_kills: i64,

    #[serde(rename = "nexusLost")]
    pub nexus_lost: i64,

    #[serde(rename = "nexusTakedowns")]
    pub nexus_takedowns: i64,

    #[serde(rename = "objectivesStolen")]
    pub objectives_stolen: i64,

    #[serde(rename = "objectivesStolenAssists")]
    pub objectives_stolen_assists: i64,

    #[serde(rename = "onMyWayPings")]
    pub on_my_way_pings: i64,

    #[serde(rename = "participantId")]
    pub participant_id: i64,

    #[serde(rename = "pentaKills")]
    pub penta_kills: i64,

    #[serde(rename = "perks")]
    pub perks: Perks,

    #[serde(rename = "physicalDamageDealt")]
    pub physical_damage_dealt: i64,

    #[serde(rename = "physicalDamageDealtToChampions")]
    pub physical_damage_dealt_to_champions: i64,

    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,

    #[serde(rename = "profileIcon")]
    pub profile_icon: i64,

    #[serde(rename = "pushPings")]
    pub push_pings: i64,

    #[serde(rename = "puuid")]
    pub puuid: String,

    #[serde(rename = "quadraKills")]
    pub quadra_kills: i64,

    #[serde(rename = "riotIdName")]
    pub riot_id_name: String,

    #[serde(rename = "riotIdTagline")]
    pub riot_id_tagline: String,

    #[serde(rename = "role")]
    pub role: String,

    #[serde(rename = "sightWardsBoughtInGame")]
    pub sight_wards_bought_in_game: i64,

    #[serde(rename = "spell1Casts")]
    pub spell1_casts: i64,

    #[serde(rename = "spell2Casts")]
    pub spell2_casts: i64,

    #[serde(rename = "spell3Casts")]
    pub spell3_casts: i64,

    #[serde(rename = "spell4Casts")]
    pub spell4_casts: i64,

    #[serde(rename = "summoner1Casts")]
    pub summoner1_casts: i64,

    #[serde(rename = "summoner1Id")]
    pub summoner1_id: i64,

    #[serde(rename = "summoner2Casts")]
    pub summoner2_casts: i64,

    #[serde(rename = "summoner2Id")]
    pub summoner2_id: i64,

    #[serde(rename = "summonerId")]
    pub summoner_id: String,

    #[serde(rename = "summonerLevel")]
    pub summoner_level: i64,

    #[serde(rename = "summonerName")]
    pub summoner_name: String,

    #[serde(rename = "teamEarlySurrendered")]
    pub team_early_surrendered: bool,

    #[serde(rename = "teamId")]
    pub team_id: i64,

    #[serde(rename = "teamPosition")]
    pub team_position: String,

    #[serde(rename = "timeCCingOthers")]
    pub time_c_cing_others: i64,

    #[serde(rename = "timePlayed")]
    pub time_played: i64,

    #[serde(rename = "totalDamageDealt")]
    pub total_damage_dealt: i64,

    #[serde(rename = "totalDamageDealtToChampions")]
    pub total_damage_dealt_to_champions: i64,

    #[serde(rename = "totalDamageShieldedOnTeammates")]
    pub total_damage_shielded_on_teammates: i64,

    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,

    #[serde(rename = "totalHeal")]
    pub total_heal: i64,

    #[serde(rename = "totalHealsOnTeammates")]
    pub total_heals_on_teammates: i64,

    #[serde(rename = "totalMinionsKilled")]
    pub total_minions_killed: i64,

    #[serde(rename = "totalTimeCCDealt")]
    pub total_time_cc_dealt: i64,

    #[serde(rename = "totalTimeSpentDead")]
    pub total_time_spent_dead: i64,

    #[serde(rename = "totalUnitsHealed")]
    pub total_units_healed: i64,

    #[serde(rename = "tripleKills")]
    pub triple_kills: i64,

    #[serde(rename = "trueDamageDealt")]
    pub true_damage_dealt: i64,

    #[serde(rename = "trueDamageDealtToChampions")]
    pub true_damage_dealt_to_champions: i64,

    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,

    #[serde(rename = "turretKills")]
    pub turret_kills: i64,

    #[serde(rename = "turretTakedowns")]
    pub turret_takedowns: i64,

    #[serde(rename = "turretsLost")]
    pub turrets_lost: i64,

    #[serde(rename = "unrealKills")]
    pub unreal_kills: i64,

    #[serde(rename = "visionClearedPings")]
    pub vision_cleared_pings: i64,

    #[serde(rename = "visionScore")]
    pub vision_score: i64,

    #[serde(rename = "visionWardsBoughtInGame")]
    pub vision_wards_bought_in_game: i64,

    #[serde(rename = "wardsKilled")]
    pub wards_killed: i64,

    #[serde(rename = "wardsPlaced")]
    pub wards_placed: i64,

    #[serde(rename = "win")]
    pub win: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Perks {
    #[serde(rename = "statPerks")]
    pub stat_perks: StatPerks,

    #[serde(rename = "styles")]
    pub styles: Vec<Style>,
}

#[derive(Serialize, Deserialize)]
pub struct StatPerks {
    #[serde(rename = "defense")]
    pub defense: i64,

    #[serde(rename = "flex")]
    pub flex: i64,

    #[serde(rename = "offense")]
    pub offense: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Style {
    #[serde(rename = "description")]
    pub description: Description,

    #[serde(rename = "selections")]
    pub selections: Vec<Selection>,

    #[serde(rename = "style")]
    pub style: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Selection {
    #[serde(rename = "perk")]
    pub perk: i64,

    #[serde(rename = "var1")]
    pub var1: i64,

    #[serde(rename = "var2")]
    pub var2: i64,

    #[serde(rename = "var3")]
    pub var3: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "bans")]
    pub bans: Vec<Ban>,

    #[serde(rename = "objectives")]
    pub objectives: Objectives,

    #[serde(rename = "teamId")]
    pub team_id: i64,

    #[serde(rename = "win")]
    pub win: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Ban {
    #[serde(rename = "championId")]
    pub champion_id: i64,

    #[serde(rename = "pickTurn")]
    pub pick_turn: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Objectives {
    #[serde(rename = "baron")]
    pub baron: Baron,

    #[serde(rename = "champion")]
    pub champion: Baron,

    #[serde(rename = "dragon")]
    pub dragon: Baron,

    #[serde(rename = "inhibitor")]
    pub inhibitor: Baron,

    #[serde(rename = "riftHerald")]
    pub rift_herald: Baron,

    #[serde(rename = "tower")]
    pub tower: Baron,
}

#[derive(Serialize, Deserialize)]
pub struct Baron {
    #[serde(rename = "first")]
    pub first: bool,

    #[serde(rename = "kills")]
    pub kills: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "dataVersion")]
    pub data_version: String,

    #[serde(rename = "matchId")]
    pub match_id: String,

    #[serde(rename = "participants")]
    pub participants: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Description {
    #[serde(rename = "primaryStyle")]
    PrimaryStyle,

    #[serde(rename = "subStyle")]
    SubStyle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChampionMastery {
    #[serde(rename = "championId")]
    champion_id: i64,

    #[serde(rename = "championLevel")]
    champion_level: i64,

    #[serde(rename = "championPoints")]
    champion_points: i64,

    #[serde(rename = "lastPlayTime")]
    last_play_time: i64,

    #[serde(rename = "championPointsSinceLastLevel")]
    champion_points_since_last_level: i64,

    #[serde(rename = "championPointsUntilNextLevel")]
    champion_points_until_next_level: i64,

    #[serde(rename = "chestGranted")]
    chest_granted: bool,

    #[serde(rename = "tokensEarned")]
    tokens_earned: i64,

    #[serde(rename = "summonerId")]
    summoner_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeagueEntry {
    #[serde(rename = "leagueId")]
    pub league_id: String,
    #[serde(rename = "queueType")]
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "leaguePoints")]
    pub league_points: i64,
    pub wins: i64,
    pub losses: i64,
    pub veteran: bool,
    pub inactive: bool,
    #[serde(rename = "freshBlood")]
    pub fresh_blood: bool,
    #[serde(rename = "hotStreak")]
    pub hot_streak: bool,
}

pub enum Division {
    I,
    II,
    III,
    IV,
}

pub enum Tier {
    DIAMOND,
    PLATINUM,
    GOLD,
    SILVER,
    BRONZE,
    IRON,
}
#[allow(non_camel_case_types)]
pub enum Queue {
    RANKED_SOLO_5x5,
    RANKED_FLEX_SR,
    RANKED_FLEX_TT, // This is technically a part of the API but really outdated. R.I.P. Twisted Treeline
}

impl fmt::Display for Division {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Division::I => write!(f, "I"),
            Division::II => write!(f, "II"),
            Division::III => write!(f, "III"),
            Division::IV => write!(f, "IV"),
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tier::DIAMOND => write!(f, "DIAMOND"),
            Tier::PLATINUM => write!(f, "PLATINUM"),
            Tier::GOLD => write!(f, "GOLD"),
            Tier::SILVER => write!(f, "SILVER"),
            Tier::BRONZE => write!(f, "BRONZE"),
            Tier::IRON => write!(f, "IRON"),
        }
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Queue::RANKED_SOLO_5x5 => write!(f, "RANKED_SOLO_5x5"),
            Queue::RANKED_FLEX_SR => write!(f, "RANKED_FLEX_SR"),
            Queue::RANKED_FLEX_TT => write!(f, "RANKED_FLEX_TT"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeagueList {
    pub tier: String,
    #[serde(rename = "leagueId")]
    pub league_id: String,
    pub queue: String,
    pub name: String,
    pub entries: Vec<Entry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "leaguePoints")]
    pub league_points: i64,
    pub rank: String,
    pub wins: i64,
    pub losses: i64,
    pub veteran: bool,
    pub inactive: bool,
    #[serde(rename = "freshBlood")]
    pub fresh_blood: bool,
    #[serde(rename = "hotStreak")]
    pub hot_streak: bool,
    #[serde(rename = "miniSeries")]
    pub mini_series: Option<MiniSeries>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiniSeries {
    pub target: i64,
    pub wins: i64,
    pub losses: i64,
    pub progress: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentGameInfo {
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "mapId")]
    pub map_id: i64,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameType")]
    pub game_type: String,
    #[serde(rename = "gameQueueConfigId")]
    pub game_queue_config_id: i64,
    pub participants: Vec<CurrGameParticipant>,
    pub observers: Observers,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "bannedChampions")]
    pub banned_champions: Vec<BannedChampion>,
    #[serde(rename = "gameStartTime")]
    pub game_start_time: i64,
    #[serde(rename = "gameLength")]
    pub game_length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrGameParticipant {
    #[serde(rename = "teamId")]
    pub team_id: i64,
    #[serde(rename = "spell1Id")]
    pub spell1id: i64,
    #[serde(rename = "spell2Id")]
    pub spell2id: i64,
    #[serde(rename = "championId")]
    pub champion_id: i64,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    pub bot: bool,
    #[serde(rename = "summonerId")]
    pub summoner_id: String,
    #[serde(rename = "gameCustomizationObjects")]
    pub game_customization_objects: Vec<Value>,
    pub perks: PerksArr,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerksArr {
    #[serde(rename = "perkIds")]
    pub perk_ids: Vec<i64>,
    #[serde(rename = "perkStyle")]
    pub perk_style: i64,
    #[serde(rename = "perkSubStyle")]
    pub perk_sub_style: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Observers {
    #[serde(rename = "encryptionKey")]
    pub encryption_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BannedChampion {
    #[serde(rename = "championId")]
    pub champion_id: i64,
    #[serde(rename = "teamId")]
    pub team_id: i64,
    #[serde(rename = "pickTurn")]
    pub pick_turn: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeaturedGames {
    #[serde(rename = "gameList")]
    pub game_list: Vec<GameList>,
    #[serde(rename = "clientRefreshInterval")]
    pub client_refresh_interval: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameList {
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "mapId")]
    pub map_id: i64,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameType")]
    pub game_type: String,
    #[serde(rename = "gameQueueConfigId")]
    pub game_queue_config_id: i64,
    pub participants: Vec<FeaturedParticipant>,
    pub observers: FeaturedObservers,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "bannedChampions")]
    pub banned_champions: Vec<Value>,
    #[serde(rename = "gameStartTime")]
    pub game_start_time: i64,
    #[serde(rename = "gameLength")]
    pub game_length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeaturedParticipant {
    #[serde(rename = "teamId")]
    pub team_id: i64,
    #[serde(rename = "spell1Id")]
    pub spell1id: i64,
    #[serde(rename = "spell2Id")]
    pub spell2id: i64,
    #[serde(rename = "championId")]
    pub champion_id: i64,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    pub bot: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeaturedObservers {
    #[serde(rename = "encryptionKey")]
    pub encryption_key: String,
}
