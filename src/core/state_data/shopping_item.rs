use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ShoppingItem {
    pub name: String,
    pub url: String,
    pub cost: u32,
    pub purchased: bool,
}

impl ShoppingItem {
    pub fn new(name: String, url: String, cost: u32, purchased: bool) -> ShoppingItem {
        ShoppingItem {
            name,
            url,
            cost,
            purchased,
        }
    }
}
