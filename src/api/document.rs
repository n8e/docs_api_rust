use crate::helpers::jwt;
use crate::helpers::mongo_id::MongoId;
use crate::{models::document::Document, repository::mongodb_repo::MongoRepo};
use mongodb::bson::oid::ObjectId;
use mongodb::{results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};
use struct_helpers::rocket::guard::HelpersGuard;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use struct_helpers::{Helpers};

async fn get_owner_id(db: &State<MongoRepo>, email: String) -> Option<ObjectId> {
    let owner_details = db.get_user_by_email(email).await;
    let owner = owner_details.unwrap();

    return owner.id;
}

#[derive(Debug, Default, Serialize, Deserialize, Helpers)]
pub struct NewDocument {
    title: Option<String>,
    content: Option<String>
}


#[derive(Debug, Default, Serialize, Deserialize, Helpers)]
pub struct DocumentCreateObject {
    title: Option<String>,
    content: Option<String>,
    date_created: DateTime<Utc>,
    last_modified: DateTime<Utc>,
}

#[get("/<id>")]
pub async fn get_document(db: &State<MongoRepo>, id: MongoId) -> Result<Json<Document>, Status> {
    let doc_detail = db.get_document(&id.to_string()).await;
    match doc_detail {
        Ok(document) => Ok(Json(document)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/", data = "<new_document>")]
pub async fn create_document(
    db: &State<MongoRepo>,
    new_document: HelpersGuard<Json<Document>>,
    _auth: jwt::AuthObject,
) -> Result<Json<InsertOneResult>, Status> {
    // get owner_id from auth user
    let owner_id = get_owner_id(db, _auth.user).await;
    let data = new_document.into_deep_inner();
    let new_doc = Document {
        id: data.id,
        owner_id: owner_id,
        title: data.title,
        content: data.content,
        date_created: Utc::now().with_second(1),
        last_modified: Utc::now().with_second(1),
    };
    let doc_detail = db.create_document(Document::from(new_doc)).await;

    return match doc_detail {
        Ok(document) => Ok(Json(document)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/<id>", data = "<new_document>")]
pub async fn update_document(
    db: &State<MongoRepo>,
    id: MongoId,
    new_document: HelpersGuard<Json<Document>>,
) -> Result<Json<Document>, Status> {
    let mut data = new_document.into_deep_inner();
    data.remove_id();

    let update_result = match db.update_document(&id.to_string(), data).await {
        Ok(update) => update,
        Err(_) => return Err(Status::InternalServerError)
    };

    if update_result.matched_count == 1 {
        match db.get_document(&id.to_string()).await {
            Ok(document) => return Ok(Json(document)),
            Err(_) => return Err(Status::InternalServerError),
        }
    }

    return Err(Status::NotFound);
}

#[delete("/<id>")]
pub async fn delete_document(db: &State<MongoRepo>, id: MongoId) -> Result<Json<&str>, Status> {
    let result = db.delete_document(&id.to_string()).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Document successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
