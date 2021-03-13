#[derive(Debug, Deserialize)]
pub struct Items(pub Vec<Item>);

use std::fs::File;

impl Items {
    pub fn load(file: File) -> Option<Items> {
        match serde_json::from_reader(file) {
            Ok(items) => Some(items),
            Err(_) => None,
        }
    }

    pub fn sum_value(&self) -> f32 {
        let mut result: f32 = 0.0;

        for item in &self.0 {
            result += item.value;
        }

        result
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Item {
    pub weight: f32,
    pub value: f32,
}