use crate::helpers::jwt;
use rocket::http::Status;
use serde_json::{json, Value};

#[get("/jwt")]
pub fn get_jwt() -> Result<Value, Status> {
    let token = jwt::jwt_sign("demo");
    Ok(json!({ "token": token }))
}
