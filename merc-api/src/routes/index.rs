use actix_web::{HttpResponse, get, web};
use serde::Serialize;

use crate::Context;

#[derive(Serialize)]
struct IndexResponse {
    start_time: String,
}

#[get("/")]
pub async fn index(ctx: web::Data<Context>) -> HttpResponse {
    HttpResponse::Ok().json(IndexResponse {
        start_time: ctx.start_time().to_rfc3339(),
    })
}
