use serde::{Deserialize, Serialize};




#[derive(Serialize,Deserialize)]
pub struct ResponseDomainModel{
    pub username:String,
}

#[derive(Serialize,Deserialize)]
pub struct ResponseSchema{
    pub message:String,
}   


#[derive(Serialize,Deserialize)]
pub struct ResponseOrderSchema{
    pub description:String,
    pub amount:f64,
    pub status_order:String,
}