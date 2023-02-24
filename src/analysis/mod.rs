use std::collections::HashMap;

use crate::types::*;

#[derive(Clone)]
pub struct ChampStats {
    pub wins: i64,
    pub losses: i64,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
}

impl ChampStats {
    pub fn empty() -> ChampStats {
        return ChampStats {
            wins: 0,
            losses: 0,
            kills: 0,
            deaths: 0,
            assists: 0,
        };
    }
}

pub fn get_player_stats_for_matches(matches: &Vec<Match>, summoner_name: &str) -> Vec<Participant> {
    let mut stats: Vec<Participant> = Vec::with_capacity(matches.len());

    for m in matches {
        for p in &m.info.participants {
            if p.summoner_name.eq(summoner_name) {
                stats.push(p.clone());
                break;
            }
        }
    }

    return stats;
}

pub fn get_champion_stats(player_stats: &Vec<Participant>) -> HashMap<String, ChampStats> {
    let mut map = HashMap::new();

    for stat in player_stats {
        let champ = &stat.champion_name;
        let mut stats = map.get(champ).unwrap_or(&ChampStats::empty()).clone();

        if stat.win {
            stats.wins += 1;
        } else {
            stats.losses += 1;
        }

        stats.kills += stat.kills;
        stats.deaths += stat.deaths;
        stats.assists += stat.assists;

        map.insert(champ.to_owned(), stats);
    }

    return map;
}
