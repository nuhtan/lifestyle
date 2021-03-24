use std::{
    fs,
    io::{BufRead, BufReader, LineWriter, Write},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use super::{super::StatefulList, Basic, calories::Calories};

#[derive(Clone)]
pub struct State {
    pub calories: Arc<Mutex<Vec<Calories>>>,
    pub basics: Arc<Mutex<Basic>>,
    pub running: Arc<Mutex<bool>>,
    pub requests: Arc<Mutex<StatefulList<String>>>,
    pub addr: [u8; 4],
    pub port: u16,
}

impl State {
    pub fn new(self, addr: [u8; 4], port: u16) -> State {
        State {
            calories: Arc::new(Mutex::new(self.load_calories())),
            basics: Arc::new(Mutex::new(Basic::load())),
            running: Arc::new(Mutex::new(true)),
            requests: Arc::new(Mutex::new(StatefulList::new())),
            addr,
            port,
        }
    }
    
    fn load_calories(self) -> Vec<Calories> {
        let file = fs::File::open("database/calories.txt").unwrap();
        let mut calories = Vec::new();
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let cal: Calories = serde_json::from_str(line.as_str()).unwrap();
            calories.push(cal);
        }
        return calories;
    }

    pub fn add_request(&self, req: (SocketAddr, String, String)) {
        let mut requests = self.requests.lock().unwrap();
        requests.add_request(format!("[{}] ({}) {}", req.0, req.2, req.1));
    }

    pub fn save(&self) {
        println!("Saving...");
        let file = fs::File::create("database/calories.txt").unwrap();
        let mut writer = LineWriter::new(file);
        let calories = self.calories.lock().unwrap().clone();
        println!("{}", calories.len());
        for cal in calories {
            writer
                .write_all(serde_json::to_string(&cal.clone()).unwrap().as_bytes())
                .unwrap();
            writer.write_all(b"\n").unwrap();
        }
    }
}
