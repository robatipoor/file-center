use crate::models::user::User;
use crate::{config::constants, payloads::responses::ResponseBody, utils::jwt::Token};
use actix_service::{Service, Transform};
use actix_web::http::header::HeaderMap;
use actix_web::HttpRequest;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use log::*;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

type PoolSqliteData = web::Data<Pool<SqliteConnectionManager>>;

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
        if let Some(token) = get_token_from_header(req.headers()) {
            if let Ok(claims) = Token::decode(token) {
                info!("authenticate pass for {:?}", claims);
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
                let resp: ResponseBody<String> =
                    ResponseBody::new(false, constants::MESSAGE_INVALID_TOKEN.to_owned(), None);
                Ok(req.into_response(HttpResponse::Unauthorized().json(resp).into_body()))
            })
        }
    }
}

pub fn get_user_id_from_request(pool: PoolSqliteData, req: HttpRequest) -> Result<i32, String> {
    get_claims_from_request(req)
        .and_then(|token| User::find_id(&pool.get().unwrap(), &*token.sub).ok())
        .ok_or("User Not Authorization".to_owned())
}

pub fn get_claims_from_request(req: HttpRequest) -> Option<Token> {
    if let Some(token) = get_token_from_header(req.headers()) {
        if let Ok(claims) = Token::decode(token) {
            info!("claims => {:?}", claims);
            return Some(claims.claims);
        }
    }
    None
}

pub fn get_token_from_header(header: &HeaderMap) -> Option<String> {
    if let Some(authen_header) = header.get(constants::AUTHORIZATION) {
        info!("Parsing authorization header...");
        if let Ok(authen_str) = authen_header.to_str() {
            if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                info!("Parsing token...");
                let token = authen_str[6..authen_str.len()].trim().to_string();
                info!("token... {}", token);
                return Some(token);
            }
        }
    }
    return None;
}
