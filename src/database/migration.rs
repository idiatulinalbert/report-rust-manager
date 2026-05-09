use anyhow::Ok;
use sqlx::{SqlitePool};


pub async fn run_migration(db:&SqlitePool) -> anyhow::Result<bool>{
    sqlx::migrate!("src/migrations").
    run(db).await.unwrap();
    Ok(true)
}