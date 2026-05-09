use axum::middleware::Next;
use axum::extract::Request;
use axum::response::Response;
use tower_cookies::Cookies;
use crate::exceptions::AppError;


pub async fn check_user(cookie:Cookies,
    req:Request,
    next:Next) -> Result<Response, AppError>{
    let response = next.run(req).await;
    let sessions_cookie = cookie.get("session_uuid");
    if sessions_cookie.is_none(){
        return Err(AppError::Unauthorized);
    }
    Ok(response)
}