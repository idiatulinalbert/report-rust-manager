use crate::unit_of_work::UnitOfWork;
use std::sync::Arc;

pub struct AppState {
    pub uow: UnitOfWork,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Arc<AppState>> {
    let uow= UnitOfWork::init().await?;
    Ok(Arc::new(AppState
         { uow }))}
}
