use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub enum UserQuery {
    Id(ObjectId),
    Username(String),
}

//UserDraft has bcrypt password
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDraft {
    pub username: String,
    pub password: String,
}

impl UserDraft {
    pub fn add_id(self, id: ObjectId) -> User {
        User {
            username: self.username,
            password: self.password,
            id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

