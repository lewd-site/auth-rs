use crate::AuthDbConn;
use auth_data::models::users::User;
use auth_data::repositories::users::UserRepository;
use chrono::prelude::*;
use rocket::response::status::{BadRequest, Created};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct UserViewModel {
    uuid: String,
    name: String,
    email: String,
    created_at: NaiveDateTime,
}

impl UserViewModel {
    fn new(user: User) -> UserViewModel {
        UserViewModel {
            uuid: user.uuid,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct UserJson {
    item: UserViewModel,
}

#[derive(Serialize)]
pub struct ErrorJson {
    message: String,
}

#[derive(Deserialize)]
pub struct RegisterJson {
    name: String,
    email: String,
    password: String,
}

#[derive(Responder)]
pub enum RegisterResponse {
    Created(Created<Json<UserJson>>),
    Error(BadRequest<Json<ErrorJson>>),
}

impl RegisterResponse {
    fn created(user: User) -> RegisterResponse {
        let view_model = UserViewModel::new(user);
        let location = format!("/api/v1/users/{}", view_model.uuid);
        let json = Json(UserJson { item: view_model });
        RegisterResponse::Created(Created(location, Some(json)))
    }

    fn error(error: &str) -> RegisterResponse {
        let message = String::from(error);
        let json = Json(ErrorJson { message });
        RegisterResponse::Error(BadRequest(Some(json)))
    }
}

#[post("/", format = "json", data = "<data>")]
pub fn register(data: Json<RegisterJson>, conn: AuthDbConn) -> RegisterResponse {
    let new_user = User::new(&data.name, &data.email, &data.password);
    match UserRepository::create(&*conn, &new_user) {
        Ok(user) => RegisterResponse::created(user),
        Err(error) => RegisterResponse::error(&error),
    }
}
