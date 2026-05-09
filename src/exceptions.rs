use thiserror::{Error};
use axum::{Json, response::IntoResponse};
use axum::http::StatusCode;
use crate::response::ResponseSchema;


#[derive(Error,Debug)]
pub enum AppError {
    #[error("Не известная ошибка")]
    IntelServerError {err:String},
    #[error("Не верный пароль")]
    BadPassword,
    #[error("Пользователь не найден")]
    UserNotFound,
    #[error("Пользователь уже существует")]
    UserAlreadyExists,
    #[error("Пользователь не авторизован")]
    Unauthorized,
    #[error("Доступ запрещен")]
    Forbidden,
}

impl IntoResponse for  AppError{
    fn into_response(self) -> axum::response::Response{

        let error_response = ResponseSchema{
            message:"Не известная ошибка".to_string(),
        };

        let bad_pass = ResponseSchema{
            message:"Не правильный пароль".to_string()
        };
        let user_not_found = ResponseSchema{
            message:"Пользователь не найден".to_string()
        };
        match self {
            Self::IntelServerError{err: _} => (StatusCode::INTERNAL_SERVER_ERROR,Json(error_response)).into_response(),
            Self::BadPassword => (StatusCode::FORBIDDEN,Json(bad_pass)).into_response(),
            Self::UserNotFound => (StatusCode::NOT_FOUND,Json(user_not_found)).into_response(),
            Self::UserAlreadyExists => (StatusCode::CONFLICT,Json(ResponseSchema{
                message:"Пользователь уже существует".to_string()
            })).into_response(),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED,Json(ResponseSchema{
                message:"Пользователь не авторизован".to_string()
            })).into_response(),
            Self::Forbidden => (StatusCode::FORBIDDEN,Json(ResponseSchema{
                message:"Доступ запрещен".to_string()
            })).into_response()
        }
    }
}