// use crate::payloads::responses::{ResponseBody, Status};
use crate::{config::constants, utils::jwt::Token};
use actix_identity::RequestIdentity;
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use sqlx::{Pool, SqliteConnection};
use std::pin::Pin;
use std::task::{Context, Poll};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }
    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;
        let identity = RequestIdentity::get_identity(&req);
        if let Some(iden) = identity {
            if Token::decode(iden).is_ok() {
                authenticate_pass = true;
            }
        }
        if authenticate_pass {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                // let resp: ResponseBody<String> =
                //     ResponseBody::new(Status::SUCCESS, constants::MESSAGE_INVALID_TOKEN);
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(constants::MESSAGE_INVALID_TOKEN)
                        .into_body(),
                ))
            })
        }
    }
}
