# Order API (Rust · Actix-Web · Postgres · SQLx)

A lightweight REST API for managing customer orders, built with **Rust**, **Actix-web**, **PostgreSQL**, and **SQLx 0.8**.  
Includes a Postgres-backed repository and a fully in-memory repository for tests.

## Features
- Endpoints:
  - `POST /orders` — create order  
  - `GET /orders/{id}` — get order by ID  
  - `GET /orders` — list all orders  
  - `PATCH /orders/{id}/status` — update order status  
  - `DELETE /orders/{id}` — delete order
- JSON Responses
- Repository trait with:
  - **PgPool implementation**
  - **In-memory implementation (for tests)**
- Tests require **no database**

## Prerequisites
- Rust (latest stable)
- Docker (for running Postgres)
- SQLx CLI:
  cargo install sqlx-cli --no-default-features --features postgres

## Setup (Quick Start)

### 1. Start Postgres
```docker-compose up -d```

### 2. Create `.env`
DATABASE_URL=postgres://postgres:postgres@localhost:5432/orders_db
SERVER_BIND=127.0.0.1:8080

### 3. Apply migrations
```sqlx migrate run```

### 4. Run API server
```cargo run```

## Testing
Runs fully in-memory (no DB needed):

```cargo test```
