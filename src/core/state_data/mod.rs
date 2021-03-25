pub mod calories;
pub mod progress;
pub mod shopping_item;
pub mod state;
pub mod valorant_game;

use std::{fs, io::{BufReader, Read}};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Basic {
    pub weight_goal: f32,
    pub weight_start: f32,
    pub rank_goal: u32
}

impl Basic {
    pub fn load() -> Basic {
        let file = fs::File::open("database/basic.json").unwrap();
        let mut content = String::new();
        BufReader::new(file).read_to_string(&mut content).unwrap();
        let base: Basic = serde_json::from_str(content.as_str()).unwrap();
        base
    }
}