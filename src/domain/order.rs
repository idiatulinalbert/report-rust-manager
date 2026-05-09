use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow,Serialize, Deserialize)]
pub struct Order {
    pub description: String,
    pub amount: f64,
    pub created_at: String,
    pub status_order: String,
}