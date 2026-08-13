use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, dev::Payload};
use std::future::{Ready, ready};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct CurrentUser {
    pub id: Uuid,
}

impl FromRequest for CurrentUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    // Parameter payload untuk sementara ini tidak di gunakan dahulu
    // kedepannya akan ada adjustment untuk role & permission
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let user_id = req.extensions().get::<Uuid>().copied();

        match user_id {
            Some(id) => ready(Ok(CurrentUser { id })),
            None => ready(Err(actix_web::error::ErrorUnauthorized(
                "User not authenticated",
            ))),
        }
    }
}
