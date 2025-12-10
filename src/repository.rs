use crate::models::{CreateOrder, Order, OrderStatus};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait OrderRepository: Send + Sync + 'static {
    async fn create_order(&self, input: CreateOrder) -> Result<Order, crate::errors::ServiceError>;
    async fn get_order(&self, id: Uuid) -> Result<Order, crate::errors::ServiceError>;
    async fn list_orders(&self) -> Result<Vec<Order>, crate::errors::ServiceError>;
    async fn update_status(
        &self,
        id: Uuid,
        status: OrderStatus,
    ) -> Result<Order, crate::errors::ServiceError>;
    async fn delete_order(&self, id: Uuid) -> Result<(), crate::errors::ServiceError>;
}

pub type DynRepo = Arc<dyn OrderRepository>;
