use std::sync::Arc;

use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;
use serde_json::Value;
use tower_cookies::Cookies;
use axum::http::StatusCode;
use crate::dto::orders::UpdateOrderStatusSchema;
use crate::exceptions::AppError;
use crate::core::state::AppState;
use crate::dto::orders::{CreateOrderRequest,GetOrderSchema};
use crate::response::ResponseOrderSchema;
use crate::response::ResponseSchema;
use crate::dto::orders::GetPaginationOrdersSchema;
use axum::extract::Query;

pub async fn create_order(State(state): State<Arc<AppState>>,
                        cookie:Cookies,
                        Json(payload): Json<CreateOrderRequest>) -> Result<(StatusCode,Json<ResponseSchema>,),AppError>{
    let user_session_cookie = cookie.get("session_uuid").ok_or( AppError::Unauthorized)?.value().to_string();
    let user_uuid = state.uow.user_repository.get_user_by_uuid(user_session_cookie).await
        .map_err(|_e| AppError::UserNotFound)?.user_uuid;

    state.uow.orders_repository.create_order(user_uuid, payload.description, payload.amount).await
        .map_err(|e| AppError::IntelServerError { err: e.to_string() })?;

    Ok((StatusCode::OK,Json(ResponseSchema{
        message:"Order created successfully".to_string()
    })))
    
}


pub async fn get_order_by_id(State(state): State<Arc<AppState>>,
                            cookie:Cookies,
                             Path(payload): Path<GetOrderSchema> ) -> Result<(StatusCode,Json<ResponseOrderSchema>),AppError>{
    let _user_session_cookie = cookie.get("session_uuid").ok_or( AppError::Unauthorized)?.value().to_string();
    let order = state.uow.orders_repository.get_order_by_id(payload.order_id).await
        .map_err(|e| AppError::IntelServerError { err: e.to_string() })?;
    Ok((StatusCode::OK,Json(ResponseOrderSchema{
        description: order.description,
        amount: order.amount,
        status_order: order.status_order
    })))
    }

pub async fn get_all_orders(State(state): State<Arc<AppState>>,
                            cookie:Cookies) -> Result<(StatusCode,Json<Value>),AppError>{
    let user_session_cookie = cookie.get("session_uuid").ok_or( AppError::Unauthorized)?.value().to_string();
    let orders = state.uow.orders_repository.get_all_orders_by_id(user_session_cookie).await
        .map_err(|e| AppError::IntelServerError { err: e.to_string() })?;
    let response = serde_json::to_value(orders).map_err(|e| AppError::IntelServerError { err: e.to_string() })?;
    Ok((StatusCode::OK,Json(response)))
}

pub async fn delete_order(State(state): State<Arc<AppState>>,
                            cookie:Cookies,
                             Json(payload): Json<GetOrderSchema> ) -> Result<(StatusCode,Json<ResponseSchema>),AppError>{
    let _user_session_cookie = cookie.get("session_uuid").ok_or( AppError::Unauthorized)?.value().to_string();
    state.uow.orders_repository.delete_order_by_id(payload.order_id).await
        .map_err(|e| AppError::IntelServerError { err: e.to_string() })?;
    Ok((StatusCode::OK,Json(ResponseSchema{
        message:"Order deleted successfully".to_string()
    })))
}


pub async fn update_order_status(State(state): State<Arc<AppState>>,
                            cookie:Cookies,
                             Json(payload): Json<UpdateOrderStatusSchema> ) -> Result<(StatusCode,Json<ResponseSchema>),AppError>{
    let user_session_cookie = cookie.get("session_uuid").ok_or( AppError::Unauthorized)?.value().to_string();
    let get_user = state.uow.user_repository.get_user_by_uuid(user_session_cookie).await
        .map_err(|_e| AppError::UserNotFound)?;

    if get_user.user_role != "admin" {
        return Err(AppError::Forbidden);
    }
    state.uow.orders_repository.update_order_status_by_id(payload.order_id, payload.status).await
        .map_err(|e| AppError::IntelServerError { err: e.to_string() })?;
    Ok((StatusCode::OK,Json(ResponseSchema{
        message:"Order status updated successfully".to_string()
    })))
}

pub async fn get_pagination_orders(State(state): State<Arc<AppState>>,
                            cookie:Cookies,
                             Query(payload): Query<GetPaginationOrdersSchema> ) -> Result<(StatusCode,Json<Value>),AppError>{
    let user_session_cookie = cookie.get("session_uuid").ok_or( AppError::Unauthorized)?.value().to_string();
    let offset = (payload.page - 1) * payload.per_page;
    let orders = state.uow.orders_repository.get_all_pagination_orders(offset,payload.per_page).await
        .map_err(|e| AppError::IntelServerError { err: e.to_string() })?;
    
    let response = serde_json::to_value(orders).map_err(|e| AppError::IntelServerError { err: e.to_string() })?;
    Ok((StatusCode::OK,Json(response)))
}