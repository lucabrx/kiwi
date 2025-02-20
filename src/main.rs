use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use kiwi::db::Db;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct SetParams {
    key: String,
    value: String,
    ttl: Option<u64>,
}

#[derive(Deserialize)]
struct Key {
    key: String,
}

async fn set_item(item: web::Json<SetParams>, db: web::Data<Arc<Db>>) -> impl Responder {
    match db
        .set(&item.key, item.value.clone(), item.ttl.unwrap_or(0))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn get_item(item: web::Json<Key>, db: web::Data<Arc<Db>>) -> impl Responder {
    match db.get(&item.key).await {
        Ok(Some(value)) => HttpResponse::Ok().json(value),
        Ok(None) => HttpResponse::Ok().body("None"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn del_item(item: web::Json<Key>, db: web::Data<Arc<Db>>) -> impl Responder {
    match db.del(&item.key).await {
        Ok(()) => HttpResponse::Ok().json("OK"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Arc::new(Db::new());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/set", web::post().to(set_item))
            .route("/get", web::post().to(get_item))
            .route("/del", web::post().to(del_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
