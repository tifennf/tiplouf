use mongodb::{
    bson::{self, doc, oid::ObjectId},
    Collection, Database,
};

use crate::shared::{ApiError, Ressource};

use super::document::{User, UserDraft, UserQuery};

pub struct UserManager {
    collection: Collection,
}

impl UserManager {
    pub fn new(database: &Database) -> UserManager {
        let collection = database.collection("user");

        UserManager { collection }
    }

    pub async fn get_one(&self, query: UserQuery) -> Result<User, ApiError> {
        let filter = match query {
            UserQuery::Id(id) => {
                doc! {
                    "_id": id,
                }
            }
            UserQuery::Username(username) => {
                doc! {
                    "username": username,
                }
            }
        };

        let user = self
            .collection
            .find_one(filter, None)
            .await?
            .ok_or_else(|| ApiError::QueryError(Ressource::User))?;

        Ok(bson::from_document::<User>(user)?)
    }

    pub async fn is_already_taken(&self, username: String) -> Result<bool, ApiError> {
        let filter = doc! {
            "username": username
        };

        let user = self.collection.find_one(filter, None).await?;

        let result = match user {
            Some(_) => true,
            None => false,
        };

        Ok(result)
    }

    pub async fn add_one(&self, user: UserDraft) -> Result<User, ApiError> {
        let doc = bson::to_document(&user)?;

        let result = self.collection.insert_one(doc, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiError::DatabaseError("Id not generated".into()))?;
        let user = user.add_id(id.clone());

        Ok(user)
    }

    pub async fn remove_one(&self, id: ObjectId) -> Result<User, ApiError> {
        let filter = doc! {
            "_id": id.clone(),
        };

        let user = self
            .collection
            .find_one_and_delete(filter, None)
            .await?
            .ok_or_else(|| ApiError::QueryError(Ressource::User))?;

        Ok(bson::from_document::<User>(user)?)
    }
}
