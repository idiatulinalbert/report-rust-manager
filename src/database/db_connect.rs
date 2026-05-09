use sqlx::sqlite::SqlitePool;

pub async fn connect_to_db() -> anyhow::Result<SqlitePool>{
    let db_conn = SqlitePool::connect("sqlite://main.db").await.unwrap();
    Ok(db_conn)
}