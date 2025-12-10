mod db;
mod errors;
mod handlers;
mod models;
mod repository;

use crate::db::SqlOrderRepository;
use crate::handlers::{create_order, delete_order, get_order, list_orders, update_status};
use crate::repository::DynRepo;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let server_bind = env::var("SERVER_BIND").unwrap_or_else(|_| "127.0.0.1:8080".into());

    // Create a pool that will work with Postgres or MySQL depending on DATABASE_URL
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");
    // Wrap in Arc
    let arc_pool = Arc::new(pool);
    let repo = Arc::new(SqlOrderRepository::new(arc_pool.clone())) as DynRepo;

    let repo_data = web::Data::new(repo);

    println!("Starting server at http://{}", &server_bind);
    HttpServer::new(move || {
        App::new()
            .app_data(repo_data.clone())
            .route("/orders", web::post().to(create_order))
            .route("/orders", web::get().to(list_orders))
            .route("/orders/{id}", web::get().to(get_order))
            .route("/orders/{id}/status", web::patch().to(update_status))
            .route("/orders/{id}", web::delete().to(delete_order))
    })
    .bind(server_bind)?
    .run()
    .await
}
