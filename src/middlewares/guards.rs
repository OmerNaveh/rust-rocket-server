use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest,Outcome};

pub struct ApplicationToken(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApplicationToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        match request.headers().get_one("application_token") {
            Some(token) if !token.is_empty() => Outcome::Success(ApplicationToken(token.to_string())),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}