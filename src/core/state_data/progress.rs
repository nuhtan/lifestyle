use std::{fs, io::{BufRead, BufReader}};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Progress {
    pub in_progress: Vec<ToDo>
}

impl Progress {
    pub fn load() -> Progress {
        let file = fs::File::open("database/progress.txt").unwrap();
        let mut progress = Vec::new();
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let pro: ToDo = serde_json::from_str(line.as_str()).unwrap();
            progress.push(pro);
        }
        Progress {
            in_progress: progress
        }
    }

    pub fn save() {

    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum ToDo {
    Bug(bool, String),
    Feature(bool, String),
}