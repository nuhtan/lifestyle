#[derive(Clone)]
pub struct Calories {
    pub total: u32,
    pub date: (u16, u8, u8),
    pub burn: u32,
    pub food: Vec<(String, u32)>,
}

impl Calories {
    pub fn new(date: (u16, u8, u8)) -> Calories {
        Calories {
            total: 0,
            date,
            burn: 0,
            food: Vec::new(),
        }
    }

    pub fn from(total: u32, date: (u16, u8, u8), burn: u32, food: Vec<(String, u32)>) -> Calories {
        Calories {
            total,
            date,
            burn,
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
