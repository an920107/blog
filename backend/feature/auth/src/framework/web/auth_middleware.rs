use std::future::{self, Ready};

use actix_session::SessionExt;
use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, error::ErrorUnauthorized};

use crate::framework::web::constants::SESSION_KEY_USER_ID;

pub struct UserId(i32);

impl UserId {
    pub fn get(&self) -> i32 {
        self.0
    }
}

impl FromRequest for UserId {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let user_id_result = req.get_session().get::<i32>(SESSION_KEY_USER_ID);

        let user_id = match user_id_result {
            Ok(id) => id,
            _ => return future::ready(Err(ErrorUnauthorized(""))),
        };

        match user_id {
            Some(id) => future::ready(Ok(UserId(id))),
            None => future::ready(Err(ErrorUnauthorized(""))),
        }
    }
}
