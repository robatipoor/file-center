use actix_web::web;

use crate::api::account_controller::*;
use crate::api::file_controller::*;
use crate::api::*;

pub fn config(service_config: &mut web::ServiceConfig) {
    service_config.service(
        web::scope("api")
            .service(web::resource("ping").route(web::get().to(ping)))
            .service(
                web::scope("file")
                    .service(web::resource("serv/{filename:.*}").route(web::get().to(serv_file)))
                    .service(
                        web::resource("save")
                            .route(web::get().to(index))
                            .route(web::post().to(save_file)),
                    ),
            )
            .service(
                web::scope("auth")
                    .service(web::resource("login").route(web::post().to(login)))
                    .service(web::resource("register").route(web::post().to(register))), // .service(web::resource("update").route(web::post().to(update_account))),
            ),
    );
}
