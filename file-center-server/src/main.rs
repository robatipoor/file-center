#![allow(dead_code)]
extern crate actix_cors;
extern crate actix_web;

extern crate bcrypt;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate envy;
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
#[macro_use]
extern crate lazy_static;

mod config;
mod errors;
mod extractors;
mod handlers;
mod middlewares;
mod models;
mod payloads;
mod routers;
mod services;
mod utils;

use crate::services::get_identity_service;
use actix_cors::Cors;
use actix_web::{http::header, middleware, App, HttpServer};
use config::CONFIG;
use dotenv::dotenv;
use log::info;
use models::DataBase;
use routers::router::router;
use std::default::Default;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let db = DataBase::auto_ddl_generate().await.unwrap();
    let pool = db.get_conn_pool().await;
    info!("Start Server Address : {}", CONFIG.address_server);
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
            .wrap(get_identity_service())
            .wrap(middleware::Logger::default())
            .configure(router)
    })
    .bind(CONFIG.address_server.as_str())?
    .run()
    .await
}
