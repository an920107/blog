use std::{
    fmt::Display,
    future::{self, Ready},
};

use actix_session::SessionExt;
use actix_web::{FromRequest, HttpRequest, dev::Payload};

use crate::framework::web::constants::SESSION_KEY_USER_ID;

pub struct UserId(i32);

impl UserId {
    pub fn get(&self) -> i32 {
        self.0
    }
}

impl FromRequest for UserId {
    type Error = UnauthorizedError;
    type Future = Ready<Result<Self, UnauthorizedError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let user_id_result = req.get_session().get::<i32>(SESSION_KEY_USER_ID);

        let user_id = match user_id_result {
            Ok(id) => id,
            _ => return future::ready(Err(UnauthorizedError)),
        };

        match user_id {
            Some(id) => future::ready(Ok(UserId(id))),
            None => future::ready(Err(UnauthorizedError)),
        }
    }
}

#[derive(Debug)]
pub struct UnauthorizedError;

impl Display for UnauthorizedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unauthorized access")
    }
}

impl actix_web::ResponseError for UnauthorizedError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Unauthorized().finish()
    }
}
