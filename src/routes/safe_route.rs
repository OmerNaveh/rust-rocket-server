use rocket::http::Status;
use crate::middlewares::guards::ApplicationToken;

#[get("/secret")]
pub fn get_secret(application_token: ApplicationToken) -> Result<String,Status> {
    if application_token.0 == "YOUR_APPLICATION_TOKEN" {
         return Ok("This is a secret".to_string());
    }
    Err(Status::Unauthorized)
}