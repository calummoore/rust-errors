use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::sync::Arc;

mod db;
mod errors;
mod indexer;

use crate::db::Db;
use crate::errors::HTTPError;

#[derive(Serialize)]
struct GetOutput {
    id: String,
    value: String,
}

struct RouteState {
    db: Arc<Db>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/{id}")]
async fn get(
    state: web::Data<RouteState>,
    path: web::Path<String>,
) -> Result<web::Json<GetOutput>, HTTPError> {
    let db = Arc::clone(&state.db);
    let id = path.into_inner();
    let value = db.get(&id).await?;

    Ok(web::Json(GetOutput {
        id,
        value: value.to_string(),
    }))
}

#[post("/{id}/{value}")]
async fn set(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut db = Db::new();
    db.set("hello".into(), "world".into()).await;

    let db = Arc::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(RouteState {
                db: Arc::clone(&db),
            }))
            .service(hello)
            .service(get)
            .service(set)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
