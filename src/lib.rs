#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate mongodb;
extern crate r2d2;
extern crate r2d2_mongodb;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use rocket::{Request, Rocket};
pub mod users;
mod mongo_connection;

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(400)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

pub fn rocket() -> Rocket {
    dotenv().ok();
    rocket::ignite()
        .register(catchers![internal_error, not_found])
        .manage(mongo_connection::init_pool())
        .mount(
            "/users",
            routes![
                users::handler::all,
                users::handler::get,
                users::handler::post,
                users::handler::put,
                users::handler::delete,
                users::handler::delete_all
            ],
        )
}