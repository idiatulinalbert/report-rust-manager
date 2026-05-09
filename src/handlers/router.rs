use std::sync::Arc;

use axum::routing::{Router, post,get,delete};
use crate::exceptions::AppError;
use crate::core::state::AppState;
use crate::handlers::users::{register_account, get_user_info, logout,delete_user};
use crate::handlers::orders::{create_order, get_order_by_id, get_all_orders, delete_order, get_pagination_orders};
use crate::middlewares::check_user::check_user;
use crate::handlers::fallback::fallback;
use axum::middleware::from_fn_with_state;

pub async fn get_router() -> Result<Router<Arc<AppState>>, AppError>{
    
    let second_router = Router::new()
    .route("/users", post(register_account));
    let state = AppState::new().await.map_err(|e| AppError::IntelServerError { err: e.to_string() })?;
    let app =Router::new()
    .route("/users/me", get(get_user_info))
    .route("/users/me/logout", post(logout))
    .route("/users/me", delete(delete_user))
    .route("/orders", post(create_order))
    .route("/orders/:order_id", get(get_order_by_id))
    .route("/orders", get(get_all_orders))
    .route("/orders", delete(delete_order))
    .route("/orders/pagination", get(get_pagination_orders))
    .merge(second_router)
    .fallback(fallback)
    .layer(from_fn_with_state(state, check_user));

    Ok(app)
}

