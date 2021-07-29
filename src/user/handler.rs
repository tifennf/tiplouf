use std::convert::TryInto;

use actix_web::{HttpResponse, cookie::Cookie, dev::HttpResponseBuilder, http::{self, StatusCode}, web};
use bcrypt::{DEFAULT_COST, verify};
use dashmap::DashMap;
use mongodb::{Database, bson::oid::ObjectId};

use crate::{shared::{ApiError, ApiResponse, ApiSuccess}, user::database::{document::UserQuery, manager::UserManager}};

use super::{database::document::{User, UserDraft}, schema::UserRequest};

use actix_web::Result;

use nanoid::nanoid;


pub async fn register(database: web::Data<Database>, body: web::Json<UserRequest>) -> Result<HttpResponse> {
	let user: UserDraft = body.0.try_into()?;
	let manager = UserManager::new(&database);

	let username_taken = manager.is_already_taken(user.username.clone()).await?;

	if username_taken {
		Err(ApiError::ValidationError("Username already taken".into()).into())
	} else {
		manager.add_one(user).await?;
		
		Ok(ApiResponse::success("", StatusCode::CREATED))
	}
	
}

pub async fn login(database: web::Data<Database>, session_list: web::Data<DashMap<String, ObjectId>>, body: web::Json<UserRequest>) -> Result<HttpResponse> {

	let submit_password = body.0.password.clone();

	let user: UserDraft = body.0.try_into()?;
	let manager = UserManager::new(&database);

	let user = manager.get_one(UserQuery::Username(user.username)).await?;
	
	let password_match = verify(submit_password, &user.password).map_err(|err| ApiError::InternalServerError(err.to_string()))?;

	if !password_match {
		return Err(ApiError::ValidationError("Username or password does not match, try again".into()).into());
	}

	let session_id = nanoid!();
	session_list.insert(session_id.clone(), user.id);

	let session_id = Cookie::new("session_id".to_string(), session_id);
	let res = HttpResponseBuilder::new(StatusCode::ACCEPTED).header(http::header::SET_COOKIE, session_id.to_string()).json(ApiSuccess::default());

	Ok(res)
}