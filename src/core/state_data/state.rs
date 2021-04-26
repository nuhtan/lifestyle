use std::{
    fs::{self, File},
    io::{BufRead, BufReader, LineWriter, Write},
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use serde::Serialize;

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
        State::verify_files_exist();
        State {
            calories: Arc::new(Mutex::new(State::load_calories())),
            basics: Arc::new(Mutex::new(Basic::load())),
            progress: Arc::new(Mutex::new(Progress::load())),
            valorant: Arc::new(Mutex::new(State::load_valorant())),
            shopping: Arc::new(Mutex::new(State::load_shopping())),
            running: Arc::new(Mutex::new(true)),
            requests: Arc::new(Mutex::new(StatefulList::new())),
            addr,
            port,
        }
    }

    fn load_calories() -> Vec<Calories> {
        let file = fs::File::open("database/calories.txt").unwrap();
        let mut calories = Vec::new();
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let cal: Calories = serde_json::from_str(line.as_str()).unwrap();
            calories.push(cal);
        }
        calories
    }

    fn load_shopping() -> Vec<ShoppingItem> {
        let file = fs::File::open("database/shopping.txt").unwrap();
        let mut shopping = Vec::new();
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let item: ShoppingItem = serde_json::from_str(line.as_str()).unwrap();
            shopping.push(item);
        }
        shopping
    }

    fn load_valorant() -> Vec<ValorantGame> {
        let file = fs::File::open("database/valorant.txt").unwrap();
        let mut games = Vec::new();
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let game: ValorantGame = serde_json::from_str(line.as_str()).unwrap();
            games.push(game);
        }
        games
    }

    pub fn add_request(&self, req: (SocketAddr, String, String)) {
        let mut requests = self.requests.lock().unwrap();
        requests.add_request(format!("[{}] ({}) {}", req.0, req.2, req.1));
    }

    pub fn save(self) {
        println!("Saving...");
        // Cloned data for working mutexes
        let second = self.clone();
        let third = self.clone();
        let vec_cals = self.calories.lock().unwrap().clone();
        println!(
            "Wrote {} calorie entries to file.",
            self.save_generic(vec_cals.clone())
        );
        let vec_shop = second.shopping.lock().unwrap().clone();
        println!(
            "Wrote {} shopping entries to file.",
            second.save_generic(vec_shop.clone())
        );
        let vec_val = third.valorant.lock().unwrap().clone();
        println!(
            "Wrote {} valorant entries to file.",
            third.save_generic(vec_val.clone())
        );
    }

    pub fn save_generic<T: Serialize>(self, list: Vec<T>) -> usize {
        let file = fs::File::create(format!("database/valorant.txt")).unwrap();
        let mut writer = LineWriter::new(file);
        let len = list.len();
        for item in list.iter() {
            writer
                .write_all(serde_json::to_string(&item.clone()).unwrap().as_bytes())
                .unwrap();
            writer.write_all(b"\n").unwrap();
        }
        return len;
    }

    pub fn verify_files_exist() {
        let files: [&str; 5] = [
            "basic.json",
            "calories.txt",
            "progress.txt",
            "shopping.txt",
            "valorant.txt",
        ];
        let paths: Vec<PathBuf> = files
            .iter()
            .map(|x| Path::new("database/").join(x))
            .collect();
        for path in paths {
            if !path.exists() {
                File::create(path).unwrap();
            }
        }
    }
}
