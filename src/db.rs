use crate::errors::ServiceError;
use crate::models::{CreateOrder, Order, OrderStatus};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

use crate::repository::OrderRepository;

pub struct SqlOrderRepository {
    pub pool: Arc<PgPool>,
}

impl SqlOrderRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // Convert DB string -> enum
    fn map_status(s: &str) -> OrderStatus {
        match s {
            "Pending" => OrderStatus::Pending,
            "Processing" => OrderStatus::Processing,
            "Shipped" => OrderStatus::Shipped,
            "Delivered" => OrderStatus::Delivered,
            "Cancelled" => OrderStatus::Cancelled,
            _ => OrderStatus::Pending,
        }
    }

    fn status_to_str(s: &OrderStatus) -> &'static str {
        match s {
            OrderStatus::Pending => "Pending",
            OrderStatus::Processing => "Processing",
            OrderStatus::Shipped => "Shipped",
            OrderStatus::Delivered => "Delivered",
            OrderStatus::Cancelled => "Cancelled",
        }
    }

    fn row_to_order(row: &sqlx::postgres::PgRow) -> Result<Order, ServiceError> {
        let id: Uuid = row.try_get("id")?;
        let customer_name: String = row.try_get("customer_name")?;
        let item: String = row.try_get("item")?;
        let quantity: i32 = row.try_get("quantity")?;
        let price_cents: i64 = row.try_get("price_cents")?;
        let status: String = row.try_get("status")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let updated_at: DateTime<Utc> = row.try_get("updated_at")?;

        Ok(Order {
            id,
            customer_name,
            item,
            quantity,
            price_cents,
            status: Self::map_status(&status),
            created_at,
            updated_at,
        })
    }
}

#[async_trait]
impl OrderRepository for SqlOrderRepository {
    // -----------------------------------------------------
    // CREATE ORDER
    // -----------------------------------------------------
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

        let status = "Pending";

        let query = r#"
            INSERT INTO orders (
                id, customer_name, item, quantity,
                price_cents, status, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#;

        sqlx::query(query)
            .bind(id)
            .bind(&input.customer_name)
            .bind(&input.item)
            .bind(input.quantity)
            .bind(price_cents)
            .bind(status)
            .bind(now)
            .bind(now)
            .execute(self.pool.as_ref())
            .await?;

        Ok(Order {
            id,
            customer_name: input.customer_name,
            item: input.item,
            quantity: input.quantity,
            price_cents,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        })
    }

    // -----------------------------------------------------
    // GET ORDER
    // -----------------------------------------------------
    async fn get_order(&self, id: Uuid) -> Result<Order, ServiceError> {
        let query = r#"
            SELECT id, customer_name, item, quantity,
                   price_cents, status, created_at, updated_at
            FROM orders
            WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(id)
            .fetch_optional(self.pool.as_ref())
            .await?;

        match row {
            Some(row) => Ok(SqlOrderRepository::row_to_order(&row)?),
            None => Err(ServiceError::NotFound),
        }
    }

    // -----------------------------------------------------
    // LIST ORDERS
    // -----------------------------------------------------
    async fn list_orders(&self) -> Result<Vec<Order>, ServiceError> {
        let query = r#"
            SELECT id, customer_name, item, quantity,
                   price_cents, status, created_at, updated_at
            FROM orders
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query(query).fetch_all(self.pool.as_ref()).await?;

        let mut list = Vec::with_capacity(rows.len());
        for row in rows {
            list.push(SqlOrderRepository::row_to_order(&row)?);
        }

        Ok(list)
    }

    // -----------------------------------------------------
    // UPDATE STATUS
    // -----------------------------------------------------
    async fn update_status(&self, id: Uuid, status: OrderStatus) -> Result<Order, ServiceError> {
        let status_str = Self::status_to_str(&status);
        let now = Utc::now();

        let query = r#"
            UPDATE orders
            SET status = $1, updated_at = $2
            WHERE id = $3
        "#;

        let result = sqlx::query(query)
            .bind(status_str)
            .bind(now)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::NotFound);
        }

        self.get_order(id).await
    }

    // -----------------------------------------------------
    // DELETE ORDER
    // -----------------------------------------------------
    async fn delete_order(&self, id: Uuid) -> Result<(), ServiceError> {
        let query = r#"
            DELETE FROM orders WHERE id = $1
        "#;

        let result = sqlx::query(query)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;

        if result.rows_affected() == 0 {
            Err(ServiceError::NotFound)
        } else {
            Ok(())
        }
    }
}
