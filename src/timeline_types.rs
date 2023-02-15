use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchTimeline {
    pub metadata: Metadata,
    pub info: Info,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "dataVersion")]
    pub data_version: String,
    #[serde(rename = "matchId")]
    pub match_id: String,
    pub participants: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "frameInterval")]
    pub frame_interval: i64,
    pub frames: Vec<Frame>,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub participants: Vec<Participant>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub events: Vec<Event>,
    #[serde(rename = "participantFrames")]
    pub participant_frames: ParticipantFrames,
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "realTimestamp")]
    pub real_timestamp: Option<i64>,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "itemId")]
    pub item_id: Option<i64>,
    #[serde(rename = "participantId")]
    pub participant_id: Option<i64>,
    #[serde(rename = "levelUpType")]
    pub level_up_type: Option<String>,
    #[serde(rename = "skillSlot")]
    pub skill_slot: Option<i64>,
    #[serde(rename = "creatorId")]
    pub creator_id: Option<i64>,
    #[serde(rename = "wardType")]
    pub ward_type: Option<String>,
    #[serde(rename = "afterId")]
    pub after_id: Option<i64>,
    #[serde(rename = "beforeId")]
    pub before_id: Option<i64>,
    #[serde(rename = "goldGain")]
    pub gold_gain: Option<i64>,
    pub level: Option<i64>,
    #[serde(rename = "assistingParticipantIds")]
    #[serde(default)]
    pub assisting_participant_ids: Vec<i64>,
    pub bounty: Option<i64>,
    #[serde(rename = "killStreakLength")]
    pub kill_streak_length: Option<i64>,
    #[serde(rename = "killerId")]
    pub killer_id: Option<i64>,
    pub position: Option<Position>,
    #[serde(rename = "shutdownBounty")]
    pub shutdown_bounty: Option<i64>,
    #[serde(rename = "victimDamageDealt")]
    #[serde(default)]
    pub victim_damage_dealt: Vec<VictimDamageDealt>,
    #[serde(rename = "victimDamageReceived")]
    #[serde(default)]
    pub victim_damage_received: Vec<VictimDamageReceived>,
    #[serde(rename = "victimId")]
    pub victim_id: Option<i64>,
    #[serde(rename = "killType")]
    pub kill_type: Option<String>,
    #[serde(rename = "multiKillLength")]
    pub multi_kill_length: Option<i64>,
    #[serde(rename = "killerTeamId")]
    pub killer_team_id: Option<i64>,
    #[serde(rename = "monsterSubType")]
    pub monster_sub_type: Option<String>,
    #[serde(rename = "monsterType")]
    pub monster_type: Option<String>,
    #[serde(rename = "laneType")]
    pub lane_type: Option<String>,
    #[serde(rename = "teamId")]
    pub team_id: Option<i64>,
    #[serde(rename = "buildingType")]
    pub building_type: Option<String>,
    #[serde(rename = "towerType")]
    pub tower_type: Option<String>,
    #[serde(rename = "gameId")]
    pub game_id: Option<i64>,
    #[serde(rename = "winningTeam")]
    pub winning_team: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VictimDamageDealt {
    pub basic: bool,
    #[serde(rename = "magicDamage")]
    pub magic_damage: i64,
    pub name: String,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    #[serde(rename = "physicalDamage")]
    pub physical_damage: i64,
    #[serde(rename = "spellName")]
    pub spell_name: String,
    #[serde(rename = "spellSlot")]
    pub spell_slot: i64,
    #[serde(rename = "trueDamage")]
    pub true_damage: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VictimDamageReceived {
    pub basic: bool,
    #[serde(rename = "magicDamage")]
    pub magic_damage: i64,
    pub name: String,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    #[serde(rename = "physicalDamage")]
    pub physical_damage: i64,
    #[serde(rename = "spellName")]
    pub spell_name: String,
    #[serde(rename = "spellSlot")]
    pub spell_slot: i64,
    #[serde(rename = "trueDamage")]
    pub true_damage: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipantFrames {
    #[serde(rename = "1")]
    pub n1: N1,
    #[serde(rename = "2")]
    pub n2: N2,
    #[serde(rename = "3")]
    pub n3: N3,
    #[serde(rename = "4")]
    pub n4: N4,
    #[serde(rename = "5")]
    pub n5: N5,
    #[serde(rename = "6")]
    pub n6: N6,
    #[serde(rename = "7")]
    pub n7: N7,
    #[serde(rename = "8")]
    pub n8: N8,
    #[serde(rename = "9")]
    pub n9: N9,
    #[serde(rename = "10")]
    pub n10: N10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N1 {
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats,
    #[serde(rename = "currentGold")]
    pub current_gold: i64,
    #[serde(rename = "damageStats")]
    pub damage_stats: DamageStats,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i64,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i64,
    pub level: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub position: Position2,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i64,
    #[serde(rename = "totalGold")]
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionStats {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: i64,
    #[serde(rename = "abilityPower")]
    pub ability_power: i64,
    pub armor: i64,
    #[serde(rename = "armorPen")]
    pub armor_pen: i64,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i64,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i64,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i64,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i64,
    pub health: i64,
    #[serde(rename = "healthMax")]
    pub health_max: i64,
    #[serde(rename = "healthRegen")]
    pub health_regen: i64,
    pub lifesteal: i64,
    #[serde(rename = "magicPen")]
    pub magic_pen: i64,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i64,
    #[serde(rename = "magicResist")]
    pub magic_resist: i64,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i64,
    pub omnivamp: i64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: i64,
    pub power: i64,
    #[serde(rename = "powerMax")]
    pub power_max: i64,
    #[serde(rename = "powerRegen")]
    pub power_regen: i64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageStats {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i64,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i64,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i64,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i64,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i64,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position2 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N2 {
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats2,
    #[serde(rename = "currentGold")]
    pub current_gold: i64,
    #[serde(rename = "damageStats")]
    pub damage_stats: DamageStats2,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i64,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i64,
    pub level: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub position: Position3,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i64,
    #[serde(rename = "totalGold")]
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionStats2 {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: i64,
    #[serde(rename = "abilityPower")]
    pub ability_power: i64,
    pub armor: i64,
    #[serde(rename = "armorPen")]
    pub armor_pen: i64,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i64,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i64,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i64,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i64,
    pub health: i64,
    #[serde(rename = "healthMax")]
    pub health_max: i64,
    #[serde(rename = "healthRegen")]
    pub health_regen: i64,
    pub lifesteal: i64,
    #[serde(rename = "magicPen")]
    pub magic_pen: i64,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i64,
    #[serde(rename = "magicResist")]
    pub magic_resist: i64,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i64,
    pub omnivamp: i64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: i64,
    pub power: i64,
    #[serde(rename = "powerMax")]
    pub power_max: i64,
    #[serde(rename = "powerRegen")]
    pub power_regen: i64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageStats2 {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i64,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i64,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i64,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i64,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i64,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position3 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N3 {
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats3,
    #[serde(rename = "currentGold")]
    pub current_gold: i64,
    #[serde(rename = "damageStats")]
    pub damage_stats: DamageStats3,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i64,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i64,
    pub level: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub position: Position4,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i64,
    #[serde(rename = "totalGold")]
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionStats3 {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: i64,
    #[serde(rename = "abilityPower")]
    pub ability_power: i64,
    pub armor: i64,
    #[serde(rename = "armorPen")]
    pub armor_pen: i64,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i64,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i64,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i64,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i64,
    pub health: i64,
    #[serde(rename = "healthMax")]
    pub health_max: i64,
    #[serde(rename = "healthRegen")]
    pub health_regen: i64,
    pub lifesteal: i64,
    #[serde(rename = "magicPen")]
    pub magic_pen: i64,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i64,
    #[serde(rename = "magicResist")]
    pub magic_resist: i64,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i64,
    pub omnivamp: i64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: i64,
    pub power: i64,
    #[serde(rename = "powerMax")]
    pub power_max: i64,
    #[serde(rename = "powerRegen")]
    pub power_regen: i64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageStats3 {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i64,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i64,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i64,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i64,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i64,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position4 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N4 {
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats4,
    #[serde(rename = "currentGold")]
    pub current_gold: i64,
    #[serde(rename = "damageStats")]
    pub damage_stats: DamageStats4,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i64,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i64,
    pub level: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub position: Position5,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i64,
    #[serde(rename = "totalGold")]
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionStats4 {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: i64,
    #[serde(rename = "abilityPower")]
    pub ability_power: i64,
    pub armor: i64,
    #[serde(rename = "armorPen")]
    pub armor_pen: i64,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i64,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i64,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i64,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i64,
    pub health: i64,
    #[serde(rename = "healthMax")]
    pub health_max: i64,
    #[serde(rename = "healthRegen")]
    pub health_regen: i64,
    pub lifesteal: i64,
    #[serde(rename = "magicPen")]
    pub magic_pen: i64,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i64,
    #[serde(rename = "magicResist")]
    pub magic_resist: i64,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i64,
    pub omnivamp: i64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: i64,
    pub power: i64,
    #[serde(rename = "powerMax")]
    pub power_max: i64,
    #[serde(rename = "powerRegen")]
    pub power_regen: i64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageStats4 {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i64,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i64,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i64,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i64,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i64,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position5 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N5 {
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats5,
    #[serde(rename = "currentGold")]
    pub current_gold: i64,
    #[serde(rename = "damageStats")]
    pub damage_stats: DamageStats5,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i64,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i64,
    pub level: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub position: Position6,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i64,
    #[serde(rename = "totalGold")]
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionStats5 {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: i64,
    #[serde(rename = "abilityPower")]
    pub ability_power: i64,
    pub armor: i64,
    #[serde(rename = "armorPen")]
    pub armor_pen: i64,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i64,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i64,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i64,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i64,
    pub health: i64,
    #[serde(rename = "healthMax")]
    pub health_max: i64,
    #[serde(rename = "healthRegen")]
    pub health_regen: i64,
    pub lifesteal: i64,
    #[serde(rename = "magicPen")]
    pub magic_pen: i64,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i64,
    #[serde(rename = "magicResist")]
    pub magic_resist: i64,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i64,
    pub omnivamp: i64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: i64,
    pub power: i64,
    #[serde(rename = "powerMax")]
    pub power_max: i64,
    #[serde(rename = "powerRegen")]
    pub power_regen: i64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageStats5 {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i64,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i64,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i64,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i64,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i64,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position6 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N6 {
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats6,
    #[serde(rename = "currentGold")]
    pub current_gold: i64,
    #[serde(rename = "damageStats")]
    pub damage_stats: DamageStats6,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i64,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i64,
    pub level: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub position: Position7,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i64,
    #[serde(rename = "totalGold")]
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionStats6 {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: i64,
    #[serde(rename = "abilityPower")]
    pub ability_power: i64,
    pub armor: i64,
    #[serde(rename = "armorPen")]
    pub armor_pen: i64,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i64,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i64,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i64,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i64,
    pub health: i64,
    #[serde(rename = "healthMax")]
    pub health_max: i64,
    #[serde(rename = "healthRegen")]
    pub health_regen: i64,
    pub lifesteal: i64,
    #[serde(rename = "magicPen")]
    pub magic_pen: i64,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i64,
    #[serde(rename = "magicResist")]
    pub magic_resist: i64,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i64,
    pub omnivamp: i64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: i64,
    pub power: i64,
    #[serde(rename = "powerMax")]
    pub power_max: i64,
    #[serde(rename = "powerRegen")]
    pub power_regen: i64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageStats6 {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i64,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i64,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i64,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i64,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i64,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position7 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N7 {
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats7,
    #[serde(rename = "currentGold")]
    pub current_gold: i64,
    #[serde(rename = "damageStats")]
    pub damage_stats: DamageStats7,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i64,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i64,
    pub level: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub position: Position8,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i64,
    #[serde(rename = "totalGold")]
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionStats7 {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: i64,
    #[serde(rename = "abilityPower")]
    pub ability_power: i64,
    pub armor: i64,
    #[serde(rename = "armorPen")]
    pub armor_pen: i64,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i64,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i64,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i64,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i64,
    pub health: i64,
    #[serde(rename = "healthMax")]
    pub health_max: i64,
    #[serde(rename = "healthRegen")]
    pub health_regen: i64,
    pub lifesteal: i64,
    #[serde(rename = "magicPen")]
    pub magic_pen: i64,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i64,
    #[serde(rename = "magicResist")]
    pub magic_resist: i64,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i64,
    pub omnivamp: i64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: i64,
    pub power: i64,
    #[serde(rename = "powerMax")]
    pub power_max: i64,
    #[serde(rename = "powerRegen")]
    pub power_regen: i64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageStats7 {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i64,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i64,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i64,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i64,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i64,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position8 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N8 {
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats8,
    #[serde(rename = "currentGold")]
    pub current_gold: i64,
    #[serde(rename = "damageStats")]
    pub damage_stats: DamageStats8,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i64,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i64,
    pub level: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub position: Position9,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i64,
    #[serde(rename = "totalGold")]
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionStats8 {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: i64,
    #[serde(rename = "abilityPower")]
    pub ability_power: i64,
    pub armor: i64,
    #[serde(rename = "armorPen")]
    pub armor_pen: i64,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i64,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i64,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i64,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i64,
    pub health: i64,
    #[serde(rename = "healthMax")]
    pub health_max: i64,
    #[serde(rename = "healthRegen")]
    pub health_regen: i64,
    pub lifesteal: i64,
    #[serde(rename = "magicPen")]
    pub magic_pen: i64,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i64,
    #[serde(rename = "magicResist")]
    pub magic_resist: i64,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i64,
    pub omnivamp: i64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: i64,
    pub power: i64,
    #[serde(rename = "powerMax")]
    pub power_max: i64,
    #[serde(rename = "powerRegen")]
    pub power_regen: i64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageStats8 {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i64,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i64,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i64,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i64,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i64,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position9 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N9 {
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats9,
    #[serde(rename = "currentGold")]
    pub current_gold: i64,
    #[serde(rename = "damageStats")]
    pub damage_stats: DamageStats9,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i64,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i64,
    pub level: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub position: Position10,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i64,
    #[serde(rename = "totalGold")]
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionStats9 {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: i64,
    #[serde(rename = "abilityPower")]
    pub ability_power: i64,
    pub armor: i64,
    #[serde(rename = "armorPen")]
    pub armor_pen: i64,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i64,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i64,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i64,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i64,
    pub health: i64,
    #[serde(rename = "healthMax")]
    pub health_max: i64,
    #[serde(rename = "healthRegen")]
    pub health_regen: i64,
    pub lifesteal: i64,
    #[serde(rename = "magicPen")]
    pub magic_pen: i64,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i64,
    #[serde(rename = "magicResist")]
    pub magic_resist: i64,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i64,
    pub omnivamp: i64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: i64,
    pub power: i64,
    #[serde(rename = "powerMax")]
    pub power_max: i64,
    #[serde(rename = "powerRegen")]
    pub power_regen: i64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageStats9 {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i64,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i64,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i64,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i64,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i64,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position10 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N10 {
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats10,
    #[serde(rename = "currentGold")]
    pub current_gold: i64,
    #[serde(rename = "damageStats")]
    pub damage_stats: DamageStats10,
    #[serde(rename = "goldPerSecond")]
    pub gold_per_second: i64,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i64,
    pub level: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub position: Position11,
    #[serde(rename = "timeEnemySpentControlled")]
    pub time_enemy_spent_controlled: i64,
    #[serde(rename = "totalGold")]
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionStats10 {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: i64,
    #[serde(rename = "abilityPower")]
    pub ability_power: i64,
    pub armor: i64,
    #[serde(rename = "armorPen")]
    pub armor_pen: i64,
    #[serde(rename = "armorPenPercent")]
    pub armor_pen_percent: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,
    #[serde(rename = "bonusArmorPenPercent")]
    pub bonus_armor_pen_percent: i64,
    #[serde(rename = "bonusMagicPenPercent")]
    pub bonus_magic_pen_percent: i64,
    #[serde(rename = "ccReduction")]
    pub cc_reduction: i64,
    #[serde(rename = "cooldownReduction")]
    pub cooldown_reduction: i64,
    pub health: i64,
    #[serde(rename = "healthMax")]
    pub health_max: i64,
    #[serde(rename = "healthRegen")]
    pub health_regen: i64,
    pub lifesteal: i64,
    #[serde(rename = "magicPen")]
    pub magic_pen: i64,
    #[serde(rename = "magicPenPercent")]
    pub magic_pen_percent: i64,
    #[serde(rename = "magicResist")]
    pub magic_resist: i64,
    #[serde(rename = "movementSpeed")]
    pub movement_speed: i64,
    pub omnivamp: i64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: i64,
    pub power: i64,
    #[serde(rename = "powerMax")]
    pub power_max: i64,
    #[serde(rename = "powerRegen")]
    pub power_regen: i64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageStats10 {
    #[serde(rename = "magicDamageDone")]
    pub magic_damage_done: i64,
    #[serde(rename = "magicDamageDoneToChampions")]
    pub magic_damage_done_to_champions: i64,
    #[serde(rename = "magicDamageTaken")]
    pub magic_damage_taken: i64,
    #[serde(rename = "physicalDamageDone")]
    pub physical_damage_done: i64,
    #[serde(rename = "physicalDamageDoneToChampions")]
    pub physical_damage_done_to_champions: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "totalDamageDone")]
    pub total_damage_done: i64,
    #[serde(rename = "totalDamageDoneToChampions")]
    pub total_damage_done_to_champions: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "trueDamageDone")]
    pub true_damage_done: i64,
    #[serde(rename = "trueDamageDoneToChampions")]
    pub true_damage_done_to_champions: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position11 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub puuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseHeaders {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Content-Type")]
    pub content_type: String,
    #[serde(rename = "Transfer-Encoding")]
    pub transfer_encoding: String,
    #[serde(rename = "Connection")]
    pub connection: String,
    #[serde(rename = "Vary")]
    pub vary: String,
    #[serde(rename = "X-App-Rate-Limit")]
    pub x_app_rate_limit: String,
    #[serde(rename = "X-App-Rate-Limit-Count")]
    pub x_app_rate_limit_count: String,
    #[serde(rename = "X-Method-Rate-Limit")]
    pub x_method_rate_limit: String,
    #[serde(rename = "X-Method-Rate-Limit-Count")]
    pub x_method_rate_limit_count: String,
    #[serde(rename = "X-Riot-Edge-Trace-Id")]
    pub x_riot_edge_trace_id: String,
    #[serde(rename = "Content-Encoding")]
    pub content_encoding: String,
    #[serde(rename = "Access-Control-Allow-Origin")]
    pub access_control_allow_origin: String,
    #[serde(rename = "Access-Control-Allow-Methods")]
    pub access_control_allow_methods: String,
    #[serde(rename = "Access-Control-Allow-Headers")]
    pub access_control_allow_headers: String,
    #[serde(rename = "Access-Control-Expose-Headers")]
    pub access_control_expose_headers: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    #[serde(rename = "Accept-Charset")]
    pub accept_charset: String,
    #[serde(rename = "Origin")]
    pub origin: String,
    #[serde(rename = "X-Riot-Token")]
    pub x_riot_token: String,
}
