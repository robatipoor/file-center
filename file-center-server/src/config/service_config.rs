use actix_web::web;

use crate::api::account_controller::*;
use crate::api::file_controller::*;
use crate::api::*;
use crate::middlewares::authentication;

pub fn config(service_config: &mut web::ServiceConfig) {
    service_config.service(
        web::scope("api")
            .service(web::resource("ping").route(web::get().to(ping)))
            .service(
                web::scope("auth")
                    .service(web::resource("login").route(web::post().to(login)))
                    .service(web::resource("register").route(web::post().to(register))), // .service(web::resource("update").route(web::post().to(update_account))),
            )
            .service(
                web::resource("health")
                    .wrap(authentication::Authentication)
                    .route(web::get().to(health)),
            )
            .service(
                web::scope("file")
                    .wrap(authentication::Authentication)
                    .service(web::resource("download/{filename:.*}").route(web::get().to(download_file)))
                    .service(
                        web::resource("upload")
                            .route(web::get().to(help_upload_file))
                            .route(web::post().to(upload_file)),
                    ),
            ),
    );
}
