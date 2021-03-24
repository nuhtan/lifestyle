use std::{
    fs,
    io::{BufRead, BufReader, LineWriter, Write},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use super::{
    super::StatefulList, calories::Calories, progress::Progress, shopping_item::ShoppingItem,
    valorant_game::ValorantGame, Basic,
};

#[derive(Clone)]
pub struct State {
    pub calories: Arc<Mutex<Vec<Calories>>>,
    pub basics: Arc<Mutex<Basic>>,
    pub progress: Arc<Mutex<Progress>>,
    pub valorant: Arc<Mutex<Vec<ValorantGame>>>,
    pub shopping: Arc<Mutex<Vec<ShoppingItem>>>,
    pub running: Arc<Mutex<bool>>,
    pub requests: Arc<Mutex<StatefulList<String>>>,
    pub addr: [u8; 4],
    pub port: u16,
}

impl State {
    pub fn new(addr: [u8; 4], port: u16) -> State {
        State {
            calories: Arc::new(Mutex::new(State::load_vectored("calories.txt"))),
            basics: Arc::new(Mutex::new(Basic::load())),
            progress: Arc::new(Mutex::new(Progress::load())),
            valorant: Arc::new(Mutex::new(State::load_vectored("valorant.txt"))),
            shopping: Arc::new(Mutex::new(State::load_vectored("shopping.txt"))),
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
        calories
    }

    pub fn load_vectored<'a, T: Clone + Deserialize<'a> + Serialize>(file: &str) -> Vec<T> {
        let file = fs::File::open(format!("database/{}", file)).unwrap();
        let mut vector = Vec::new();
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let item: T = serde_json::from_str(line.as_str()).unwrap();
            vector.push(item);
        }
        vector
    }

    pub fn add_request(&self, req: (SocketAddr, String, String)) {
        let mut requests = self.requests.lock().unwrap();
        requests.add_request(format!("[{}] ({}) {}", req.0, req.2, req.1));
    }

    pub fn save(&self) {
        println!("Saving...");
        println!(
            "Wrote {} calorie entries to file.",
            self.save_vectored(*self.calories.lock().unwrap(), "calories.txt")
        );
        println!(
            "Wrote {} shopping entries to file.",
            self.save_vectored(*self.shopping.lock().unwrap(), "shopping.txt")
        );
        println!(
            "Wrote {} valorant entries to file.",
            self.save_vectored(*self.valorant.lock().unwrap(), "valorant.txt")
        );
    }

    fn save_vectored<'a, T: Clone + Deserialize<'a> + Serialize>(
        self,
        vector: Vec<T>,
        file: &str,
    ) -> usize {
        let file = fs::File::create(format!("database/{}", file)).unwrap();
        let mut writer = LineWriter::new(file);
        let len = vector.len();
        for item in vector {
            writer
                .write_all(serde_json::to_string(&item.clone()).unwrap().as_bytes())
                .unwrap();
            writer.write_all(b"\n").unwrap();
        }
        len
    }
}
