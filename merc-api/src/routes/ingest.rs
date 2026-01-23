use actix_web::{HttpResponse, post, web};
use serde::Deserialize;

use crate::RequestContext;

#[derive(Deserialize)]
struct IngestPath {
    pub scope_id: String,
}

#[derive(Deserialize)]
struct IngestChatPayload {
    pub text: String,
}

#[post("/chats/{scope_id}/ingest")]
pub async fn ingest(
    ctx: RequestContext,
    path: web::Path<IngestPath>,
    payload: web::Json<IngestChatPayload>,
) -> HttpResponse {
    let _ctx = ctx.context();
    let _scope_id = path.into_inner().scope_id;
    let _text = payload.into_inner().text;

    // TODO: implement ingest logic

    HttpResponse::Ok().finish()
}
