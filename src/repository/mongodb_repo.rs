use std::{env, error::Error};
extern crate dotenv;
// use chrono::Utc;
use dotenv::dotenv;
use rocket::{futures::StreamExt};
use serde::{Serialize, Deserialize};

use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use crate::{models::{user::User, document::Document}, helpers::jwt};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub struct MongoRepo {
    user_col: Collection<User>,
    document_col: Collection<Document>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    username: String,
    firstname: String,
    lastname: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginObject {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    user: UserResponse,
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

        let document_col: Collection<Document> = db.collection("Document");
        let user_col: Collection<User> = db.collection("User");

        MongoRepo { document_col, user_col }
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
            .user_col
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
            .user_col
            .find_one(filter, None).await {
                Ok(u) => {
                    let user = u.as_ref().unwrap();
                    let password_hash = user.password.to_string();
                    let parsed_hash = PasswordHash::new(&password_hash).unwrap();

                    match Argon2::default().verify_password(credentials.password.as_bytes(), &parsed_hash) {
                        Ok(_data) => {
                            let derived_user = user;
                            let user_response = UserResponse {
                                firstname: derived_user.firstname.as_ref().unwrap().to_string(),
                                lastname: derived_user.lastname.as_ref().unwrap().to_string(),
                                username: derived_user.username.as_ref().unwrap().to_string(),
                                email: derived_user.email.as_ref().unwrap().to_string(),
                            };
                            let user_email = user_response.email.as_str();
                            let signed_string = jwt::jwt_sign(user_email);

                            Ok(AuthResponse {
                                user: user_response,
                                token: signed_string
                            })
                        },
                        Err(_) => Err("Login Error: Passwords do not match".to_string())
                    }
                },
                Err(_) => Err("User does not exist".to_string()),
            };
        Ok(resp.unwrap())
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user_col
            .find_one(filter, None)
            .await 
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User, Box<dyn Error>> {
        let filter = doc! {"email": email};
        let user_detail = self
            .user_col
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
            .user_col
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
            .user_col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let users = match self.user_col.find(None, None).await {
                Ok(cursors) => cursors.map(|doc| doc.unwrap()).collect().await,
                Err(_e) => {
                    println!("Error getting list of users");
                    Vec::new()
                }
            };

        Ok(users)
    }


    /**
     * Documents
    */

    pub async fn create_document(&self, new_document: Document) -> Result<InsertOneResult, Box<dyn Error>> {
        let document = match self
            .document_col
            .insert_one(new_document, None).await {
                Ok(u) => u,
                Err(e) => {
                    print!("Error creating document: {}", e);
                    panic!("Error creating document: {}", e)
                }
            };
        Ok(document)
    }

    pub async fn get_document(&self, id: &String) -> Result<Document, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let doc_detail = self
            .document_col
            .find_one(filter, None)
            .await 
            .ok()
            .expect("Error getting document detail");
        Ok(doc_detail.unwrap())
    }

    pub async fn update_document(&self, id: &String, new_document: Document) -> Result<UpdateResult, Box<dyn Error>> {
        let mut doc = to_document(&new_document).unwrap();
        doc.remove("_id");

        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! { "$set": doc };
        let updated_doc = self
            .document_col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating document");
        Ok(updated_doc)
    }

    pub async fn delete_document(&self, id: &String) -> Result<DeleteResult, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let doc_detail = self
            .document_col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting document");
        Ok(doc_detail)
    }
}