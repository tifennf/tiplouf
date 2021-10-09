use std::{convert::TryInto, sync::RwLock};

use actix_web::{cookie::Cookie, dev::HttpResponseBuilder, http::StatusCode, web, HttpResponse};
use bcrypt::verify;
use bimap::BiHashMap;
use mongodb::{bson::oid::ObjectId, Database};
// use tokio::sync::RwLock;

use crate::{
    shared::{
        error::{InternalServerError, ValidationError},
        ApiError, ApiResponse, ApiSuccess,
    },
    user::database::{document::UserQuery, manager::UserManager},
};

use super::{
    database::document::UserDraft,
    schema::{SessionCheck, UserRequest},
};

use actix_web::Result;

use nanoid::nanoid;

type SessionList = web::Data<RwLock<BiHashMap<String, ObjectId>>>;

pub async fn register(
    database: web::Data<Database>,
    body: web::Json<UserRequest>,
) -> Result<HttpResponse> {
    let user: UserDraft = body.0.try_into()?;
    let manager = UserManager::new(&database);

    let username_taken = manager.is_already_taken(user.username.clone()).await?;

    if username_taken {
        Err(ApiError::ValidationError(ValidationError::UsernameTaken).into())
    } else {
        manager.add_one(user).await?;

        Ok(ApiResponse::success("", StatusCode::CREATED))
    }
}

pub async fn login(
    database: web::Data<Database>,
    session_list: SessionList,
    body: web::Json<UserRequest>,
) -> Result<HttpResponse> {
    let submit_password = body.0.password.clone();

    let user: UserDraft = body.0.try_into()?;
    let manager = UserManager::new(&database);

    let user = manager.get_one(UserQuery::Username(user.username)).await?;

    let password_match = verify(submit_password, &user.password).map_err(|err| {
        ApiError::InternalServerError(InternalServerError::Other(err.to_string()))
    })?;

    if !password_match {
        return Err(ApiError::ValidationError(ValidationError::UserIdentifier).into());
    }

    let session_id = nanoid!();

    let mut guard = session_list.write().map_err(|err| {
        ApiError::InternalServerError(InternalServerError::Other(err.to_string()))
    })?;

    guard.insert(session_id.clone(), user.id);

    let mut session_id = Cookie::new("session_id".to_string(), session_id);
    session_id.set_path("/");
    let res = HttpResponseBuilder::new(StatusCode::ACCEPTED)
        .header("Set-Cookie", session_id.to_string())
        .json(ApiSuccess::default());

    Ok(res)
}

pub async fn check_session(
    session_list: SessionList,
    body: web::Json<SessionCheck>,
) -> Result<HttpResponse> {
    let check = body.0;

    let guard = session_list.try_read().map_err(|err| {
        ApiError::InternalServerError(InternalServerError::Other(err.to_string()))
    })?;

    if !guard.contains_left(&check.session_id) {
        Err(ApiError::ValidationError(ValidationError::NotLogged).into())
    } else {
        Ok(ApiResponse::success(
            serde_json::Value::Null,
            StatusCode::ACCEPTED,
        ))
    }
}
