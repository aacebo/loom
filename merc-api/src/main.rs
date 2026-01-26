use actix_web::{App, HttpServer, web};
use merc_events::{Key, MemoryAction};
use sqlx::postgres::PgPoolOptions;

mod config;
mod context;
mod request_context;
mod routes;

pub use config::Config;
pub use context::Context;
pub use request_context::{RequestContext, RequestContextMiddleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("../merc-storage/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let producer = merc_events::new(&config.rabbitmq_url)
        .with_app_id("merc[api]")
        .with_queue(Key::memory(MemoryAction::Create))
        .with_queue(Key::memory(MemoryAction::Update))
        .connect()
        .await
        .expect("error while connecting to rabbitmq")
        .produce();

    let ctx = Context::new(pool, producer);
    println!("Starting server at http://0.0.0.0:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ctx.clone()))
            .wrap(RequestContextMiddleware)
            .service(routes::index)
            .service(routes::ingest)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
