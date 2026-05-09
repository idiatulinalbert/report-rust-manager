use core::str;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub description: String,
    pub amount: f64,
}


#[derive(Serialize, Deserialize)]
pub struct GetOrderSchema{
    pub order_id:String,

}

#[derive(Serialize, Deserialize)]
pub struct UpdateOrderStatusSchema{
    pub order_id:String,
    pub status:String,
}
#[derive(Serialize, Deserialize)]
pub struct GetPaginationOrdersSchema{
    pub page: i64,
    pub per_page: i64,
}