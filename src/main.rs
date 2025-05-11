use actix_web::{web, App, HttpServer};
use db::conn::Pool;
use dotenvy::dotenv;
mod handlers;
use std::env;
mod db;
mod engine;
mod middleware;
mod models;
mod routes;
mod utils;

use sqlx::postgres::PgPoolOptions;

struct AppState {
    db: Pool,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // let pg_url = env::var("DATABASE_URL");
    let pg_url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(e) => "couldn't interpret {e}".to_string(),
    };

    let pool = db::conn::init_pool(&pg_url).await;

    HttpServer::new(move || {
        App::new()
            //  .app_Data() shove the database data from top to bottom
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(routes::auth_routes::config) // Remove .service(web::scope("/api"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
