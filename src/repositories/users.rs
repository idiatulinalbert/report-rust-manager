
use sqlx::SqlitePool;
use chrono::Utc;
use chrono::DateTime;
use crate::domain::users::*;

pub struct UserRepository{
    pub db:SqlitePool,

}


impl UserRepository {
    pub async fn add_user(&self,username:String,password:String,
                            time:DateTime<Utc>,user_uuid:String) -> Result<(),sqlx::Error>{
        sqlx::query("INSERT INTO users (username,password,created_at,user_uuid) VALUES (?,?,?,?)")  
            .bind(username)
            .bind(password)
            .bind(time.to_string())
            .bind(user_uuid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn get_user_by_name(&self,user_name:String) -> Result<UsersDomain,sqlx::Error>{
        let data = sqlx::query_as::<_,UsersDomain>("SELECT username,user_uuid,password_hash,user_role FROM users WHERE username = ?")
            .bind(user_name)
            .fetch_one(&self.db)
            .await?;
        Ok(data)
        
    }

    pub async fn get_user_by_uuid(&self,user_uuid:String) -> Result<UsersDomain,sqlx::Error>{
        let data = sqlx::query_as::<_,UsersDomain>("SELECT username,user_uuid,password_hash,user_role FROM users WHERE user_uuid = ?")
            .bind(user_uuid)
            .fetch_one(&self.db)
            .await?;
        Ok(data)
    }
    pub async fn delete_user(&self,user_uuid:String) -> Result<(),sqlx::Error>{
        sqlx::query("DELETE FROM users WHERE user_uuid = ?")
            .bind(user_uuid)
            .execute(&self.db)
            .await?;
        Ok(())
}
}
pub struct UserRepositoryError {
    pub message: String,
}


