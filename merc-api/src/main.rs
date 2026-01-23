use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;

mod context;
mod request_context;
mod routes;

pub use context::Context;
pub use request_context::{RequestContext, RequestContextMiddleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:admin@localhost:5432/main".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("../merc-storage/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let ctx = Context::new(pool);

    println!("Starting server at http://0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ctx.clone()))
            .wrap(RequestContextMiddleware)
            .service(routes::index)
            .service(routes::ingest)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
