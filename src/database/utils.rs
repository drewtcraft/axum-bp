use log::info;
use sqlx::{PgPool, Row};

pub async fn run_database_migrations(db_pool: &PgPool) {
    info!("running migration checks");

    let migrations_dir = std::path::Path::new("migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations_dir)
        .await
        .expect("could not create migrator")
        .run(db_pool)
        .await
        .expect("failed to run migration");

    info!("migration: {:?}", migration_results);
}

pub async fn log_existing_tables(db_pool: &PgPool) {
    let result = sqlx::query(
        "SELECT table_name
FROM information_schema.tables
WHERE table_schema = 'public' AND table_type = 'BASE TABLE';
",
    )
    .fetch_all(db_pool)
    .await;

    if let Ok(result) = result {
        for (idx, row) in result.iter().enumerate() {
            info!("[{}]: {:?}", idx, row.get::<String, &str>("table_name"));
        }
    }
}
