use log::info;
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, Postgres, PgPool};
use std::env;

use super::utils::{run_database_migrations, log_existing_tables};

pub async fn connect() -> PgPool {
    let database_url = env::var("POSTGRES_URL").expect("could not load POSTGRES_URL from environment");

    ensure_database(&database_url).await;
    let db_pool = get_database_connection(&database_url).await;

    run_database_migrations(&db_pool).await;

    log_existing_tables(&db_pool).await;

    db_pool
}

async fn ensure_database(database_url: &String) {
    if !Postgres::database_exists(database_url).await.unwrap_or(false) {
        info!("existing database not found, creating database now");

        Postgres::create_database(database_url)
            .await
            .expect("database could not be created and does not exist");
    }

    info!("existing database found");
}

async fn get_database_connection(database_url: &String) -> PgPool {
    let db_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("could not connect to database");

    info!("database connection established");

    db_pool
}
