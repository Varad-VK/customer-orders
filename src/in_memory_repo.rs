use crate::errors::ServiceError;
use crate::models::{CreateOrder, Order, OrderStatus};
use crate::repository::OrderRepository;
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct InMemoryRepo {
    inner: Arc<RwLock<HashMap<Uuid, Order>>>,
}

impl InMemoryRepo {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryRepo {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl OrderRepository for InMemoryRepo {
    async fn create_order(&self, input: CreateOrder) -> Result<Order, ServiceError> {
        if input.quantity <= 0 {
            return Err(ServiceError::BadRequest("quantity must be > 0".into()));
        }
        if input.price < 0.0 {
            return Err(ServiceError::BadRequest("price must be >= 0".into()));
        }

        let id = Uuid::new_v4();
        let now = Utc::now();
        let price_cents = (input.price * 100.0).round() as i64;

        let order = Order {
            id,
            customer_name: input.customer_name,
            item: input.item,
            quantity: input.quantity,
            price_cents,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        };
        let mut map = self.inner.write().unwrap();
        map.insert(id, order.clone());
        Ok(order)
    }

    async fn get_order(&self, id: Uuid) -> Result<Order, ServiceError> {
        let map = self.inner.read().unwrap();
        map.get(&id).cloned().ok_or(ServiceError::NotFound)
    }

    async fn list_orders(&self) -> Result<Vec<Order>, ServiceError> {
        let map = self.inner.read().unwrap();
        let mut v: Vec<_> = map.values().cloned().collect();
        v.sort_by_key(|o| std::cmp::Reverse(o.created_at));
        Ok(v)
    }

    async fn update_status(&self, id: Uuid, status: OrderStatus) -> Result<Order, ServiceError> {
        let mut map = self.inner.write().unwrap();
        let order = map.get_mut(&id).ok_or(ServiceError::NotFound)?;
        order.status = status;
        order.updated_at = Utc::now();
        Ok(order.clone())
    }

    async fn delete_order(&self, id: Uuid) -> Result<(), ServiceError> {
        let mut map = self.inner.write().unwrap();
        if map.remove(&id).is_some() {
            Ok(())
        } else {
            Err(ServiceError::NotFound)
        }
    }
}
