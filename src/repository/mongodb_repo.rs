use std::{env, error::Error};
extern crate dotenv;
use dotenv::dotenv;
use rocket::{futures::StreamExt};

use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use crate::{models::user_model::User, helpers::jwt};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub struct MongoRepo {
    col: Collection<User>,
}

pub struct LoginObject {
    username: String,
    password: String,
}

pub struct AuthResponse {
    user: User,
    token: String,
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
        // hash password before saving
        let password = new_user.password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password(password, &salt).unwrap();

        let new_doc = User {
            id: None,
            firstname: new_user.firstname,
            lastname: new_user.lastname,
            username: new_user.username,
            email: new_user.email,
            password: password_hash.to_string(),
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

    pub async fn login(&self, credentials: LoginObject) -> Result<AuthResponse, Box<dyn Error>> {
        // get user by username: if not found return 404
        let filter = doc! {"username": credentials.username};
        let resp = match self
            .col
            .find_one(filter, None).await {
                Ok(u) => {
                    let password_hash = u.unwrap().password;
                    let parsed_hash = PasswordHash::new(&password_hash).unwrap();

                    match Argon2::default().verify_password(credentials.password.as_bytes(), &parsed_hash) {
                        Ok(data) => {
                            let signed_string = jwt::jwt_sign(u.unwrap().email.to_owned());

                            return Ok(AuthResponse {
                                user: u.unwrap(),
                                token: signed_string
                            })
                        },
                        Err(e) => {
                            print!("Login Error: Passwords do not match: {}", e);
                            panic!("Login Error: Passwords do not match: {}", e)
                        }
                    }
                },
                Err(e) => {
                    print!("User does not exist: {}", e);
                    panic!("User does not exist: {}", e)
                },
            };
        Ok(resp.unwrap())
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
        let mut doc = to_document(&new_user).unwrap();
        doc.remove("_id");

        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! { "$set": doc };
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