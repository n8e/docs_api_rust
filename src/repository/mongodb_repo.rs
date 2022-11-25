use std::{env, error::Error};
extern crate dotenv;
use dotenv::dotenv;
use rocket::futures::StreamExt;

use mongodb::{
    bson::{doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();

        let client_uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("You must set the MONGO_URI environment var!")
        };

        let client = Client::with_uri_str(client_uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");

        MongoRepo { col }
    }
}

impl MongoRepo {
    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Box<dyn Error>> {
        let new_doc = User {
            id: None,
            firstname: new_user.firstname,
            lastname: new_user.lastname,
            username: new_user.username,
            email: new_user.email,
            password: new_user.password,
            role: new_user.role
        };
        let user = match self
            .col
            .insert_one(new_doc, None).await {
                Ok(u) => u,
                Err(e) => {
                    print!("Error creating user: {}", e);
                    panic!("Error creating user: {}", e)
                }
            };
        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await 
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "firstname": new_user.firstname,
                    "lastname": new_user.lastname,
                    "username": new_user.username,
                    "email": new_user.email,
                    "password": new_user.password,
                    "role": new_user.role
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let users = match self.col.find(None, None).await {
                Ok(cursors) => cursors.map(|doc| doc.unwrap()).collect().await,
                Err(_e) => {
                    println!("Error getting list of users");
                    Vec::new()
                }
            };

        Ok(users)
    }
}