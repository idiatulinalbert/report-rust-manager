use axum::{routing::Router};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use std::net::SocketAddr;
use first_prject::{database, handlers::router::get_router,core::state::AppState};


#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let app = Router::new()
    .merge(get_router().await?)
    .layer(CookieManagerLayer::new())
    .with_state(AppState::new().await?);
    let socker = SocketAddr::from(([127,0,0,0],2000));
    let db = database::db_connect::connect_to_db().await?;
    database::migration::run_migration(&db).await?;
    let tcp = TcpListener::bind(socker).await.unwrap();
    axum::serve(tcp,app).await.unwrap();
    anyhow::Ok(())
}