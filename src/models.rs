use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    #[default]
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub customer_name: String,
    pub item: String,
    pub quantity: i32,
    pub price_cents: i64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub customer_name: String,
    pub item: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatus {
    pub status: OrderStatus,
}
