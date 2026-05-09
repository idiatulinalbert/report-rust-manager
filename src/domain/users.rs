use sqlx::FromRow;

#[derive(FromRow)]
pub struct UsersDomain{
    pub username:String,
    pub user_uuid:String,
    pub password_hash:String,
    pub user_role:String,
}