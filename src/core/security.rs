use bcrypt::{hash,verify,DEFAULT_COST};

use crate::exceptions::AppError;



pub async fn hash_password(password:&String) -> Result<String,AppError>{
    let hashed =  hash(password, DEFAULT_COST).
    map_err(|e| AppError::IntelServerError { err: (e.to_string()) })?;
    Ok(hashed)
}
pub async fn verifiry_password(password:&String,hash:&String) -> Result<bool,AppError>{
    let res_verifiry = verify(password, hash)
    .map_err(|_e|AppError::BadPassword)?;
    Ok(res_verifiry)
}