// add the modules
mod api;
mod helpers;
mod models;
mod repository;

#[cfg(test)]
mod tests;

#[macro_use]
extern crate rocket;

use api::auth::get_jwt;
use api::user::{
    create_user,
    get_user,
    update_user,
    delete_user,
    get_all_users
};
use repository::mongodb_repo::MongoRepo;

use rocket::{get, http::Status, serde::json::Json, Build, Rocket};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let db = MongoRepo::init().await;

    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount("/users", routes![create_user, get_user, update_user, delete_user, get_all_users])
        .mount("/auth", routes![get_jwt])
}