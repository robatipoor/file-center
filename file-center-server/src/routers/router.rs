use actix_web::web;

use crate::handlers::access::*;
use crate::handlers::account::*;
use crate::handlers::file::*;
use crate::handlers::{health, ping};
use crate::middlewares::auth;

pub fn router(service_config: &mut web::ServiceConfig) {
    service_config.service(
        web::scope("api")
            .service(web::resource("manual").route(web::get().to(manual_upload_file)))
            .service(web::resource("ping").route(web::get().to(ping)))
            .service(
                web::scope("auth")
                    .service(web::resource("login").route(web::post().to(login)))
                    .service(web::resource("logout").route(web::post().to(logout)))
                    .service(web::resource("register").route(web::post().to(register))), // .service(web::resource("update").route(web::post().to(update_account))),
            )
            .service(
                web::resource("health")
                    .wrap(auth::Authentication)
                    .route(web::get().to(health)),
            )
            .service(
                web::scope("file")
                    .wrap(auth::Authentication)
                    .service(
                        web::resource("download/{linkID:.*}").route(web::get().to(download_file)),
                    )
                    .service(
                        web::resource("upload")
                            .route(web::get().to(manual_upload_file))
                            .route(web::post().to(upload_file)),
                    )
                    .service(web::resource("list").route(web::get().to(list_file)))
                    .service(
                        web::resource("access")
                            .route(web::post().to(add_access))
                            // .route(web::delete().to(remove_access)),
                    ),
            ),
    );
}
