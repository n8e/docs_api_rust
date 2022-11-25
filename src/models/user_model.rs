use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use struct_helpers::{to_lower_case, to_lower_case_optional, Helpers};

#[derive(Serialize, Deserialize, Debug)]
pub enum RoleEnum {
    User,
    Administrator,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Helpers)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    #[helper(to_lower_case)]
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<RoleEnum>,
}

#[derive(Debug, Serialize, Deserialize, Helpers)]
pub struct UserName {
    #[helper(to_lower_case)]
    pub username: String,
}

impl User {
    pub fn remove_id(&mut self) {
        self.id = None;
    }
}

impl From<UserName> for User {
    fn from(u: UserName) -> Self {
        User {
            username: u.username.into(),
            ..Default::default()
        }
    }
}
