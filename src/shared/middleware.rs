use std::{sync::RwLock};

use actix_web::{Error, HttpMessage, HttpResponse, dev::{Service, ServiceRequest, ServiceResponse, Transform}, web::{Data}};
use bimap::BiHashMap;
use futures::future::{self, Either, Ready};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


use super::{ApiError, error::{InternalServerError, ValidationError}};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub user_id: ObjectId,
}

impl SessionInfo {
    pub fn new(session_id: String, user_id: ObjectId) -> SessionInfo {
        SessionInfo {
            session_id,
            user_id,
        }
    }
}

pub struct Logging;

impl<S, B> Transform<S> for Logging
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;

    type Response = ServiceResponse<B>;

    type Error = Error;

    type Transform = LoggingMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(LoggingMiddleware { service })
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service for LoggingMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;

    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    //todo: find a way to use async for tokio RwLock
    fn call(&mut self, req: Self::Request) -> Self::Future {    
        match setup_session(&req) {
            Ok(_) => {
                Either::Left(self.service.call(req))
            },
            Err(err) => Either::Right(future::ok(
                req.into_response(HttpResponse::from_error(err.into()).into_body()),
            )),
        }

    }
}

fn setup_session(req: &ServiceRequest) -> Result<(), ApiError> {
    let session_id = req
        .cookie("session_id")
        .ok_or(ApiError::ValidationError(ValidationError::CookieMissing))?;

    let session_list = req
        .app_data::<Data<RwLock<BiHashMap<String, ObjectId>>>>()
        .ok_or(ApiError::InternalServerError(InternalServerError::SessionListMissing))?;

    let session_id = session_id.value();

    let user_id = session_list.read().map_err(|err| ApiError::InternalServerError(InternalServerError::Other(err.to_string())))?;
    let user_id = user_id.get_by_left(session_id).ok_or(ApiError::ValidationError(ValidationError::NotLogged))?;
    
    let session = SessionInfo::new(session_id.to_string(), user_id.clone());
    
    req.extensions_mut().insert(session);

    Ok(())
}
