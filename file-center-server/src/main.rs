#![allow(dead_code)]
extern crate actix_web;
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

mod api;
mod config;
mod errors;
mod middlewares;
mod models;
mod payloads;
mod services;
mod utils;

use crate::config::service_config::config;
use crate::models::DataBase;
use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use log::info;
use std::default::Default;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    env_logger::init();
    let port = env::var("PORT").unwrap();
    let addr = format!("127.0.0.1:{}", port);
    let db = DataBase::migrate().await.unwrap();
    let pool = db.get_conn_pool().await;
    info!("Start Server {}", addr);
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .configure(config)
    })
    .bind(addr)?
    .run()
    .await
}
