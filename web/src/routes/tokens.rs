use crate::AuthDbConn;
use auth_data::models::sessions::NewSession;
use auth_data::models::tokens::{AccessToken, RefreshToken};
use auth_data::repositories::sessions::SessionRepository;
use auth_data::repositories::users::UserRepository;
use rocket::response::status::{BadRequest, Created};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TokenJson {
    access_token: String,
    refresh_token: String,
}

#[derive(Serialize)]
pub struct ErrorJson {
    message: String,
}

#[derive(Deserialize)]
pub struct CreateTokenJson {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    refresh_token: Option<String>,
}

#[derive(Responder)]
pub enum TokenResponse {
    Created(Created<Json<TokenJson>>),
    Error(BadRequest<Json<ErrorJson>>),
}

impl TokenResponse {
    fn created(access_token: AccessToken, refresh_token: RefreshToken) -> TokenResponse {
        let location = String::from("/api/v1/tokens");
        let json = Json(TokenJson {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
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
pub fn create_token(data: Json<CreateTokenJson>, conn: AuthDbConn) -> TokenResponse {
    let user = match (&data.name, &data.email) {
        (Some(name), None) => UserRepository::get_by_name(&conn, &name),
        (None, Some(email)) => UserRepository::get_by_email(&conn, &email),
        _ => return TokenResponse::error("Either name or email required"),
    };

    match user {
        Some(user) => match (&data.password, &data.refresh_token) {
            (Some(password), None) => {
                if user.verify_password(&password) {
                    let access_token = AccessToken::new(&user);
                    let refresh_token = RefreshToken::new();
                    SessionRepository::create(
                        &conn,
                        &NewSession {
                            user_id: user.id,
                            refresh_token: refresh_token.to_string(),
                        },
                    );

                    TokenResponse::created(access_token, refresh_token)
                } else {
                    TokenResponse::error("Incorrect password")
                }
            }
            (None, Some(refresh_token)) => {
                match SessionRepository::get_by_token(&conn, user.id, &refresh_token) {
                    Some(session) => {
                        SessionRepository::delete(&conn, &session);

                        let access_token = AccessToken::new(&user);
                        let refresh_token = RefreshToken::new();
                        SessionRepository::create(
                            &conn,
                            &NewSession {
                                user_id: user.id,
                                refresh_token: refresh_token.to_string(),
                            },
                        );

                        TokenResponse::created(access_token, refresh_token)
                    }
                    None => TokenResponse::error("Incorrect refresh token"),
                }
            }
            _ => TokenResponse::error("Either password or refresh_token required"),
        },
        None => TokenResponse::error("User not found"),
    }
}
