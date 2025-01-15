use std::time::Duration;

use once_cell::sync::OnceCell;
use sqlx::{PgPool, postgres::PgPoolOptions};

static POSTGRES: OnceCell<PgPool> = OnceCell::new();

pub async fn init_pg_pool() {
    let db_connection_str = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Failed to create Postgres connection pool.");

    POSTGRES.set(pool).unwrap();
}

#[inline]
pub fn get_pg_pool() -> &'static PgPool {
    POSTGRES.get().unwrap()
}
