use crate::helpers::jwt;
use crate::helpers::mongo_id::MongoId;
use crate::repository::mongodb_repo::{LoginObject, AuthResponse};
use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::{results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};
use struct_helpers::rocket::guard::HelpersGuard;

#[get("/<id>")]
pub async fn get_user(db: &State<MongoRepo>, id: MongoId) -> Result<Json<User>, Status> {
    let user_detail = db.get_user(&id.to_string()).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/", data = "<new_user>")]
pub async fn create_user(
    db: &State<MongoRepo>,
    new_user: HelpersGuard<Json<User>>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = new_user.into_deep_inner();
    println!("{:?}", data);
    let user_detail = db.create_user(User::from(data)).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/", data = "<new_user>")]
pub async fn login(
    db: &State<MongoRepo>,
    new_user: HelpersGuard<Json<User>>,
) -> Result<Json<AuthResponse>, Status> {
    let data = new_user.into_deep_inner();
    println!("{:?}", data);

    let login_object = LoginObject {
        username: data.username.unwrap(),
        password: data.password
    };
    
    let user_detail = db.login(LoginObject::from(login_object)).await;
    match user_detail {
        Ok(auth_response) => Ok(Json(auth_response)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/<id>", data = "<new_user>")]
pub async fn update_user(
    db: &State<MongoRepo>,
    id: MongoId,
    new_user: HelpersGuard<Json<User>>,
) -> Result<Json<User>, Status> {
    let mut data = new_user.into_deep_inner();
    data.remove_id();

    let update_result = match db.update_user(&id.to_string(), data).await {
        Ok(update) => update,
        Err(_) => return Err(Status::InternalServerError)
    };

    if update_result.matched_count == 1 {
        match db.get_user(&id.to_string()).await {
            Ok(user) => return Ok(Json(user)),
            Err(_) => return Err(Status::InternalServerError),
        }
    }

    return Err(Status::NotFound);
}

#[delete("/<id>")]
pub async fn delete_user(db: &State<MongoRepo>, id: MongoId, _auth: jwt::Auth) -> Result<Json<&str>, Status> {
    let result = db.delete_user(&id.to_string()).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/")]
pub async fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}