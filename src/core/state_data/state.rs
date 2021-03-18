use std::{fs, io::{BufRead, BufReader, BufWriter, LineWriter, Write}, net::SocketAddr, sync::{Arc, Mutex}};

use crate::core::StatefulList;

use super::calories::Calories;

#[derive(Clone)]
pub struct State {
    pub calories: Arc<Mutex<Vec<Calories>>>,
    pub running: Arc<Mutex<bool>>,
    pub requests: Arc<Mutex<StatefulList<String>>>,
    pub addr: [u8; 4],
    pub port: u16,
}

impl State {
    pub fn new(addr: [u8; 4], port: u16) -> State {
        fn load_cals() -> Vec<Calories> {
            let file = fs::File::open("database/calories.txt").unwrap();
            let mut cals = Vec::new();
            for line in BufReader::new(file).lines() {
                let line = line.unwrap();
                let cal: Calories = serde_json::from_str(line.as_str()).unwrap();
                cals.push(cal);
            };
            return cals;
        }

        State {
            calories: Arc::new(Mutex::new(load_cals())),
            running: Arc::new(Mutex::new(true)),
            requests: Arc::new(Mutex::new(StatefulList::new())),
            addr,
            port,
        }
    }

    pub fn add_request(&self, req: (SocketAddr, String)) {
        let mut reqs = self.requests.lock().unwrap();
        reqs.add_request(format!("[{}]: {}", req.0, req.1));
    }

    pub fn save(&self) {
        println!("Saving...");
        let file = fs::File::create("database/calories.txt").unwrap();
        let mut writer = LineWriter::new(file);
        let cals = self.calories.lock().unwrap().clone();
        println!("{}", cals.len());
        for cal in cals {
            writer.write_all(serde_json::to_string(&cal.clone()).unwrap().as_bytes()).unwrap();
            writer.write_all(b"\n").unwrap();
        }
    }
}
