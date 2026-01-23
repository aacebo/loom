use actix_web::{HttpResponse, post, web};
use serde::Deserialize;

use crate::Context;

#[derive(Deserialize)]
pub struct IngestPayload {
    pub text: String,
}

#[post("/ingest")]
pub async fn ingest(ctx: web::Data<Context>, payload: web::Json<IngestPayload>) -> HttpResponse {
    let _ctx = ctx.into_inner();
    let _text = payload.into_inner().text;

    // TODO: implement ingest logic

    HttpResponse::Ok().finish()
}
