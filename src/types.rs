use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub price: f64,
}
