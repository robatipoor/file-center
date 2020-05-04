#![allow(dead_code)]
extern crate actix_web;
extern crate actix_cors;

extern crate bcrypt;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate jsonwebtoken;
extern crate log;
extern crate serde_json;
extern crate sqlx;
extern crate strum;
extern crate uuid;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate anyhow;

mod config;
mod errors;
mod handlers;
mod middlewares;
mod extractors;
mod models;
mod payloads;
mod routers;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::{http::header, middleware, App, HttpServer};
use log::info;
use models::DataBase;
use routers::router::router;
use std::default::Default;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    env_logger::init();
    let port = env::var("PORT").unwrap();
    let addr = format!("0.0.0.0:{}", port);
    let db = DataBase::migrate().await.unwrap();
    let pool = db.get_conn_pool().await;
    info!("Start Server {}", addr);
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(router)
    })
    .bind(addr)?
    .run()
    .await
}
