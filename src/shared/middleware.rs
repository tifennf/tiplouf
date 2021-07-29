use std::pin::Pin;

use actix_web::{Error, HttpMessage, dev::{Service, Transform, ServiceRequest, ServiceResponse}, http, web::Data};
use dashmap::DashMap;
use futures::{Future, TryFutureExt, future::{self, Either, Ready}};
use mongodb::bson::oid::ObjectId;

use super::{ApiError, ApiResponse};


pub struct SayHi;

impl<S, B> Transform<S> for SayHi 
where
	S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
    type Request = ServiceRequest;

    type Response = ServiceResponse<B>;

    type Error = Error;

    type Transform = SayHiMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(SayHiMiddleware { service })
    }
}



pub struct SayHiMiddleware<S>{
	service: S,
}

impl<S, B> Service for SayHiMiddleware<S> 
where
	S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,

{
    type Request = ServiceRequest;

    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let state = req.app_data::<Data<DashMap<String, ObjectId>>>().unwrap();

        let session_id = req.cookie("session_id");

        match session_id {
            Some(cookie) if state.contains_key(cookie.value()) => {
                println!("Login OK");

                Either::Left(self.service.call(req))
            },
            Some(_) | None =>{
                println!("Login FAIL");
                Either::Right(future::ok(req.into_response(
                    ApiResponse::fail(Some(ApiError::ValidationError("You are not logged in".into())), http::StatusCode::UNAUTHORIZED).into_body()
                )))
            }
        }

    }
}