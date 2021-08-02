use std::convert::TryInto;

use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};

use crate::shared::ApiError;

use super::database::document::{User, UserDraft};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

// impl UserRequest {
//     fn into_draft(self) -> Result<UserDraft, ApiError> {
//         let password = hash(self.password, DEFAULT_COST)?;

//         Ok(UserDraft {
//             username: self.username,
//             password,
//         })
//     }
// }

impl TryInto<UserDraft> for UserRequest {
    type Error = ApiError;

    fn try_into(self) -> Result<UserDraft, Self::Error> {
        let password = hash(self.password, DEFAULT_COST)?;

        Ok(UserDraft {
            username: self.username,
            password,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub username: String,
    pub password: String,
    pub id: String,
}

impl From<User> for UserJson {
    fn from(user: User) -> Self {
        let id = user.id.to_string();

        UserJson {
            username: user.username,
            password: user.password,
            id,
        }
    }
}