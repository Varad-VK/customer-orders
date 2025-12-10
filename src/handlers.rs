use crate::models::{CreateOrder, UpdateStatus};
use crate::repository::OrderRepository;
use actix_web::{HttpResponse, web};
use std::sync::Arc;
use uuid::Uuid;

pub type RepoData = Arc<dyn OrderRepository>;

pub async fn create_order(
    repo: web::Data<RepoData>,
    input: web::Json<CreateOrder>,
) -> Result<HttpResponse, crate::errors::ServiceError> {
    let created = repo.create_order(input.into_inner()).await?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_order(
    repo: web::Data<RepoData>,
    path: web::Path<String>,
) -> Result<HttpResponse, crate::errors::ServiceError> {
    let id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| crate::errors::ServiceError::BadRequest("invalid uuid".into()))?;
    let order = repo.get_order(id).await?;
    Ok(HttpResponse::Ok().json(order))
}

pub async fn list_orders(
    repo: web::Data<RepoData>,
) -> Result<HttpResponse, crate::errors::ServiceError> {
    let orders = repo.list_orders().await?;
    Ok(HttpResponse::Ok().json(orders))
}

pub async fn update_status(
    repo: web::Data<RepoData>,
    path: web::Path<String>,
    body: web::Json<UpdateStatus>,
) -> Result<HttpResponse, crate::errors::ServiceError> {
    let id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| crate::errors::ServiceError::BadRequest("invalid uuid".into()))?;
    let updated = repo.update_status(id, body.status.clone()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete_order(
    repo: web::Data<RepoData>,
    path: web::Path<String>,
) -> Result<HttpResponse, crate::errors::ServiceError> {
    let id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| crate::errors::ServiceError::BadRequest("invalid uuid".into()))?;
    repo.delete_order(id).await?;
    Ok(HttpResponse::NoContent().finish())
}
