use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ShoppingItem {
    pub name: String,
    pub url: String,
    pub cost: u32,
    pub progress: u32,
}

impl ShoppingItem {
    pub fn new(name: String, url: String, cost: u32, progress: u32) -> ShoppingItem {
        ShoppingItem {
            name,
            url,
            cost,
            progress,
        }
    }
}
