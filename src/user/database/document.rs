use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::user::schema::UserJson;

pub enum UserQuery {
    Id(ObjectId),
    Identifier(UserDraft),
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

impl Into<UserJson> for User {
    fn into(self) -> UserJson {
        let id = self.id.to_string();

        UserJson {
            username: self.username,
            password: self.password,
            id,
        }
    }
}


