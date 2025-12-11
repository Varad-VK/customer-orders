use actix_web::{App, test};
use customer_orders::handlers::{
    create_order, delete_order, get_order, list_orders, update_status,
};
use customer_orders::in_memory_repo::InMemoryRepo;
use customer_orders::repository::DynRepo;

use serde_json::json;
use std::sync::Arc;

#[actix_rt::test]
async fn test_create_get_update_delete_order() {
    // In-memory repo → no database needed
    let repo: DynRepo = Arc::new(InMemoryRepo::new());

    // Build test application
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(repo.clone()))
            .route("/orders", actix_web::web::post().to(create_order))
            .route("/orders", actix_web::web::get().to(list_orders))
            .route("/orders/{id}", actix_web::web::get().to(get_order))
            .route(
                "/orders/{id}/status",
                actix_web::web::patch().to(update_status),
            )
            .route("/orders/{id}", actix_web::web::delete().to(delete_order)),
    )
    .await;

    // 1. CREATE ORDER

    let req_body = json!({
        "customer_name": "Varad",
        "item": "iphone",
        "quantity": 1,
        "price": 83990.00
    });

    let req = test::TestRequest::post()
        .uri("/orders")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let created: serde_json::Value = test::read_body_json(resp).await;
    let id = created["id"].as_str().unwrap().to_string();

    // 2. GET ORDER
    let req = test::TestRequest::get()
        .uri(&format!("/orders/{}", id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let fetched: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(fetched["customer_name"], "Varad");
    assert_eq!(fetched["item"], "iphone");

    // 3. UPDATE ORDER STATUS
    let req = test::TestRequest::patch()
        .uri(&format!("/orders/{}/status", id))
        .set_json(&json!({ "status": "Shipped" }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let updated: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(updated["status"], "Shipped");

    // 4. DELETE ORDER
    let req = test::TestRequest::delete()
        .uri(&format!("/orders/{}", id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);

    // 5. GET after deletion → Error 404
    let req = test::TestRequest::get()
        .uri(&format!("/orders/{}", id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
