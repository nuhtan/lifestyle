use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ValorantGame {
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub character: Character,
    pub map: Map,
    pub reportable_teammate: bool,
    pub reportable_round_start: u32,
    pub reportable_on_own_team: bool,
    pub rank_rating_after: u32,
    pub rank_rating_change: u32,
}

impl ValorantGame {
    pub fn new(
        kda: (u32, u32, u32),
        character: Character,
        map: Map,
        rep: bool,
        rep_start: u32,
        rep_team: bool,
        rr_after: u32,
        rr_change: u32,
    ) -> ValorantGame {
        ValorantGame {
            kills: kda.0,
            deaths: kda.1,
            assists: kda.2,
            character,
            map,
            reportable_teammate: rep,
            reportable_round_start: rep_start,
            reportable_on_own_team: rep_team,
            rank_rating_after: rr_after,
            rank_rating_change: rr_change,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum Character {
    Astra,
    Breach,
    Brimstone,
    Cypher,
    Jett,
    Killjoy,
    Omen,
    Phoenix,
    Raze,
    Reyna,
    Sage,
    Skye,
    Sova,
    Viper,
    Yoru,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum Map {
    Ascent,
    Bind,
    Haven,
    Icebox,
    Split,
}
