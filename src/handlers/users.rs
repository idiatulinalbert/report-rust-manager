
use std::sync::Arc;

use crate::{core::state::AppState, dto::users::LoginDTO};
use crate::response::{ResponseDomainModel, ResponseSchema};
use crate::core::security::hash_password;
use crate::exceptions::AppError;
use axum::extract::{Json, State};
use chrono::Utc;
use tower_cookies::{ Cookies, Cookie};
use uuid::Uuid;
use axum::http::StatusCode;

pub async fn register_account(State(state):State<Arc<AppState>>, 
                            cookie: Cookies,
                            Json(data): Json<LoginDTO>) -> 
                            Result<(StatusCode,Json<ResponseSchema>,),AppError>{
    let create_uuid = Uuid::new_v4().to_string();
    let user_session_cookie = cookie.get("session_uuid");
    if user_session_cookie.is_some(){
        return Err(AppError::UserAlreadyExists);
    }
    let time = Utc::now();
    let hashed_password = hash_password(&data.password).await?;

    state.uow.user_repository.add_user(data.username.to_string(), hashed_password, time, create_uuid.clone()).await
    .map_err(|e| AppError::IntelServerError { err:e.to_string()  })?;

    state.uow.session_repository.create_session(create_uuid.clone(), create_uuid).await
        .map_err(|e| AppError::IntelServerError { err:e.to_string() })?;

    let result = ResponseSchema {
        message: "User created successfully".to_string(),
    };
    Ok((StatusCode::OK,Json(result)))
}

pub async fn get_user_info(State(state):State<Arc<AppState>>,
                                            cookie: Cookies)  -> Result<(StatusCode,Json<ResponseDomainModel>), AppError> {
    let user_session_cookie = cookie.get("session_uuid");
    if user_session_cookie.is_none(){
        return Err(AppError::Unauthorized);
    }
    let user = state.uow.user_repository.get_user_by_uuid(user_session_cookie.unwrap().value().to_string()).await.
    map_err(|_e| AppError::UserNotFound)?;
    Ok((StatusCode::OK,Json(ResponseDomainModel{
        username: user.username,
    })))

}

pub async fn logout(State(state):State<Arc<AppState>>,cookie: Cookies) -> Result<(StatusCode,Json<ResponseSchema>), AppError>{
    let user_session_cookie = cookie.get("session_uuid");
    if user_session_cookie.is_none(){
        return Err(AppError::Unauthorized);
    }
    state.uow.session_repository.delete_session(user_session_cookie.unwrap().value().to_string()).await
        .map_err(|e| AppError::IntelServerError { err:e.to_string() })?;
    cookie.remove(Cookie::new("session_uuid",""));
    let result = ResponseSchema {
        message: "User logged out successfully".to_string(),
    };
    Ok((StatusCode::OK,Json(result)))
}


pub async fn delete_user(State(state):State<Arc<AppState>>, cookie: Cookies) -> Result<(StatusCode,Json<ResponseSchema>), AppError>{
    let user_session_cookie = cookie.get("session_uuid");
    if user_session_cookie.is_none(){
        return Err(AppError::Unauthorized);
    }
    state.uow.user_repository.delete_user(user_session_cookie.unwrap().value().to_string()).await
        .map_err(|e| AppError::IntelServerError { err:e.to_string() })?;
    cookie.remove(Cookie::new("session_uuid",""));
    let result = ResponseSchema {
        message: "User deleted successfully".to_string(),
    };
    Ok((StatusCode::OK,Json(result)))
}