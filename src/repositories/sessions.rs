use sqlx::SqlitePool;

pub struct SessionRepository{
    pub db:SqlitePool,}

impl SessionRepository {
    pub async fn delete_session(&self,session_uuid:String) -> Result<(),sqlx::Error>{
        sqlx::query("DELETE FROM sessions WHERE session_uuid = ?")
            .bind(session_uuid)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    pub async fn create_session(&self, user_uuid: String, session_uuid: String) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO sessions (user_uuid, session_uuid) VALUES (?, ?)")
            .bind(user_uuid)
            .bind(session_uuid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}