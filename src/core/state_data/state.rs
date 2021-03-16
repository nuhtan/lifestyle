use std::sync::{Arc, Mutex};

use super::calories::Calories;

#[derive(Clone)]
pub struct State {
    calories: Arc<Mutex<Vec<Calories>>>
}

impl State {
    pub fn new() -> State {
        State {
            calories: Arc::new(Mutex::new(Vec::new()))
        }
    }
}