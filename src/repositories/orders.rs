use sqlx::SqlitePool;
use crate::domain::order::Order;
pub struct OrderRepository {
    pub db: SqlitePool,
}

impl OrderRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
    pub async fn create_order(&self, user_id: String, description: String, amount: f64) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO orders (user_id, description, amount,status_order) VALUES (?, ?, ?, ?)")
            .bind(user_id)
            .bind(description)
            .bind(amount)
            .bind("pending") 
            .execute(&self.db)
            .await?;
        Ok(())
    }

    pub async fn get_order_by_id(&self, order_id: String) -> Result<Order, sqlx::Error> {
        let order = sqlx::query_as::<_, Order>("SELECT  description, amount, status_order FROM orders WHERE id = ?")
            .bind(order_id)
            .fetch_one(&self.db)
            .await?;
        Ok(order)
    }
    pub async fn get_all_orders_by_id(&self, user_id: String) -> Result<Vec<Order>, sqlx::Error> {
        let orders = sqlx::query_as::<_, Order>("SELECT description, amount, status_order FROM orders WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(&self.db)
            .await?;
        Ok(orders)
    }

    pub async fn get_all_pagination_orders(&self,offset:i64,page_size:i64) -> Result<Vec<Order>,sqlx::Error>{
        let orders = sqlx::query_as::<_,Order>("SELECT description, amount, status_order FROM orders LIMIT ? OFFSET ?")
            .bind(page_size)
            .bind(offset)
            .fetch_all(&self.db)
            .await?;
        Ok(orders)
    }

    pub async fn delete_order_by_id(&self, order_id: String) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM orders WHERE id = ?")
            .bind(order_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_order_status_by_id(&self, order_id: String,status:String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE orders SET status_order = ? WHERE id = ?")
            .bind(status)
            .bind(order_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}