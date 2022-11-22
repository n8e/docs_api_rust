use crate::users;
use crate::mongo_connection::Conn;
use users::User;
use mongodb::{doc, error::Error, oid::ObjectId};
use rocket::http::Status;
use rocket_contrib::json::Json;

fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/")]
pub fn all(connection: Conn) -> Result<Json<Vec<User>>, Status> {
    match users::repository::all(&connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}

#[get("/<id>")]
pub fn get(id: String, connection: Conn) -> Result<Json<User>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match users::repository::get(res, &connection) {
            Ok(res) => Ok(Json(res.unwrap())),
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[post("/", format = "application/json", data = "<users>")]
pub fn post(users: Json<User>, connection: Conn) -> Result<Json<ObjectId>, Status> {
    match users::repository::insert(users.into_inner(), &connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}

#[put("/<id>", format = "application/json", data = "<users>")]
pub fn put(id: String, users: Json<User>, connection: Conn) -> Result<Json<User>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match users::repository::update(res, users.into_inner(), &connection) {
            Ok(res) => Ok(Json(res)),
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[delete("/<id>")]
pub fn delete(id: String, connection: Conn) -> Result<Json<String>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match users::repository::delete(res, &connection) {
            Ok(_) => Ok(Json(id)),
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[delete("/")]
pub fn delete_all(connection: Conn) -> Result<Json<bool>, Status> {
    match users::repository::delete_all(&connection) {
        Ok(_) => Ok(Json(true)),
        Err(err) => Err(error_status(err)),
    }
}