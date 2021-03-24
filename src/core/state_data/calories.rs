use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Calories {
    pub index: u32,
    pub total: u32,
    pub date: (u16, u8, u8),
    pub burn: u32,
    pub end_weight: f32,
    pub food: Vec<(String, u32)>,
}

impl Calories {
    pub fn new(index: u32, date: (u16, u8, u8)) -> Calories {
        Calories {
            index,
            total: 0,
            date,
            burn: 0,
            end_weight: 0.0,
            food: Vec::new(),
        }
    }

    pub fn full(
        index: u32,
        total: u32,
        date: (u16, u8, u8),
        burn: u32,
        end_weight: f32,
        food: Vec<(String, u32)>,
    ) -> Calories {
        Calories {
            index,
            total,
            date,
            burn,
            end_weight,
            food,
        }
    }

    pub fn burn(&mut self, amount: u32) {
        self.burn += amount;
        self.calculate_total();
    }

    pub fn add_food(&mut self, name: String, calories: u32) {
        self.food.push((name, calories));
        self.calculate_total();
    }

    fn calculate_total(&mut self) {
        let mut consume_total = 0;
        for (_item, cal) in &self.food {
            consume_total += cal;
        }
        self.total = consume_total - self.burn;
    }
}
