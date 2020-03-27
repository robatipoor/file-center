#![allow(dead_code)]
extern crate actix_web;
extern crate bcrypt;
extern crate chrono;
extern crate derive_new;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate jsonwebtoken;
extern crate log;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate serde_json;
extern crate uuid;

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
    let conn = DataBase::migrate().unwrap().get_connection();
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
            .data(conn.clone())
            .wrap(actix_web::middleware::Logger::default())
            .configure(config)
    })
    .bind(addr)?
    .run()
    .await
}
