use crate::models::user::UserAuth;
use crate::utils::jwt::Token;
use actix_identity::RequestIdentity;
use actix_web::{
    dev::Payload,
    web::{HttpRequest, HttpResponse},
    Error, FromRequest,
};
use futures::future::{err, ok, Ready};

impl FromRequest for UserAuth {
    type Error = Error;
    type Config = ();
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let identity = RequestIdentity::get_identity(req);
        if let Some(identity) = identity {
            let token: Token = Token::decode(identity).unwrap().claims;
            return ok(UserAuth {
                id: token.sub,
                role_id: token.rol,
            });
        }
        err(HttpResponse::Unauthorized().into())
    }
}
