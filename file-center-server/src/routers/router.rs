use actix_web::web;

use crate::handlers::api::access::*;
use crate::handlers::api::account::*;
use crate::handlers::api::file::*;
use crate::handlers::api::*;
use crate::handlers::*;
use crate::middlewares::auth;

pub fn router(service_config: &mut web::ServiceConfig) {
    service_config.service(
        web::scope("/")
            .service(web::resource("").route(web::get().to(home_page)))
            .service(web::resource("login").route(web::get().to(login_page)))
            .service(web::resource("register").route(web::get().to(register_page)))
            .service(web::resource("list").route(web::get().to(list_file_page)))
            .service(web::resource("upload").route(web::get().to(upload_page)))
            .service(web::resource("access/{linkID:.*}").route(web::get().to(access_page)))
            .service(
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
                                web::resource("download/{linkID:.*}")
                                    .route(web::get().to(download_file)),
                            )
                            .service(
                                web::resource("upload")
                                    .route(web::get().to(manual_upload_file))
                                    .route(web::post().to(upload_file)),
                            )
                            .service(web::resource("list").route(web::get().to(list_file)))
                            .service(
                                web::resource("access")
                                    .route(web::post().to(add_or_update_access))
                                    .route(web::delete().to(remove_access)),
                            ),
                    ),
            ),
    );
}
