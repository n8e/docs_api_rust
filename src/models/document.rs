use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use struct_helpers::{to_lower_case_optional, Helpers};

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Helpers)]
pub struct Document {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    #[helper(to_lower_case)]
    pub title: Option<String>,
    pub content: Option<String>,
    pub date_created: Option<String>,
    pub last_modified: Option<String>,
}


// TODO: add "owner_id" and link to User model; default date fields to right now