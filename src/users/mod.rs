#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
pub mod repository;
use mongodb::bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub age: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableUser {
    pub name: Option<String>,
    pub color: Option<String>,
    pub age: Option<i32>,
}

impl InsertableUser {
    fn from_user(users: User) -> InsertableUser {
        InsertableUser {
            name: users.name,
            color: users.color,
            age: users.age,
        }
    }
}