use sqlx::{postgres::PgPoolOptions, Postgres};

pub type Pool = sqlx::Pool<Postgres>;

pub async fn init_pool(database_url:&str) -> Pool{

    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failde to connect to pool")
}