use crate::AuthDbConn;
use auth_data::repositories::users::UserRepository;
use rocket::response::status::{BadRequest, Created};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TokenJson {
    access_token: String,
}

#[derive(Serialize)]
pub struct ErrorJson {
    message: String,
}

#[derive(Deserialize)]
pub struct CreateTokenJson {
    name: Option<String>,
    email: Option<String>,
    password: String,
}

#[derive(Responder)]
pub enum TokenResponse {
    Created(Created<Json<TokenJson>>),
    Error(BadRequest<Json<ErrorJson>>),
}

impl TokenResponse {
    fn created(token: &str) -> TokenResponse {
        let location = String::from("/api/v1/tokens");
        let json = Json(TokenJson {
            access_token: String::from(token),
        });
        TokenResponse::Created(Created(location, Some(json)))
    }

    fn error(error: &str) -> TokenResponse {
        let message = String::from(error);
        let json = Json(ErrorJson { message });
        TokenResponse::Error(BadRequest(Some(json)))
    }
}

#[post("/", format = "json", data = "<data>")]
pub fn create_access_token(data: Json<CreateTokenJson>, conn: AuthDbConn) -> TokenResponse {
    let user = match (&data.name, &data.email) {
        (Some(name), _) => UserRepository::get_by_name(&conn, &name),
        (_, Some(email)) => UserRepository::get_by_email(&conn, &email),
        _ => return TokenResponse::error("Username or email required"),
    };

    if let Some(user) = user {
        if user.verify_password(&data.password) {
            TokenResponse::created(&user.create_access_token())
        } else {
            TokenResponse::error("Incorrect password")
        }
    } else {
        TokenResponse::error("User not found")
    }
}
