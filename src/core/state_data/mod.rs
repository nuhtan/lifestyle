pub mod calories;
pub mod state;

use std::{fs, io::{BufRead, BufReader, Read}};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Basic {
    pub weight_goal: u32,
    pub rank_goal: u32
}

impl Basic {
    pub fn load() -> Basic {
        let file = fs::File::open("database/calories.txt").unwrap();
        let mut content = String::new();
        BufReader::new(file).read_to_string(&mut content).unwrap();
        let base: Basic = serde_json::from_str(content.as_str()).unwrap();
        base
    }
}