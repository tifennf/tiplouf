use std::pin::Pin;

use actix_web::{Error, dev::{Service, Transform, ServiceRequest, ServiceResponse}};
use futures::{Future, future::{self, Ready}};


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

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

		let fut = self.service.call(req);

		Box::pin(async move {
			let res = fut.await?;

			println!("Hi from response");
			Ok(res)
		})
    }
}