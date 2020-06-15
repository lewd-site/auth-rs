use crate::schema::users;
use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;

lazy_static! {
    static ref ENCODING_KEY_RAW: Vec<u8> =
        fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../private.pem")).unwrap();
    static ref DECODING_KEY_RAW: Vec<u8> =
        fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../public.pem")).unwrap();
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_rsa_pem(&ENCODING_KEY_RAW).unwrap();
    static ref DECODING_KEY: DecodingKey<'static> =
        DecodingKey::from_rsa_pem(&DECODING_KEY_RAW).unwrap();
}

#[derive(Serialize, Deserialize)]
struct Claims {
    user_uuid: String,
    user_name: String,
    user_email: String,
    iat: DateTime<Utc>,
    nbf: DateTime<Utc>,
    exp: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn new(name: &str, email: &str, password: &str) -> NewUser {
        NewUser {
            uuid: Uuid::new_v4().to_string(),
            name: String::from(name),
            email: String::from(email),
            password: bcrypt::hash(password).unwrap(),
            created_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password)
    }

    pub fn create_access_token(&self) -> String {
        let header = Header::new(Algorithm::RS256);

        let now = Utc::now();
        let claims = Claims {
            user_uuid: self.uuid.clone(),
            user_name: self.name.clone(),
            user_email: self.email.clone(),
            iat: now,
            nbf: now,
            exp: now + Duration::days(1),
        };

        jsonwebtoken::encode(&header, &claims, &*ENCODING_KEY).unwrap()
    }

    pub fn validate_access_token(access_token: &str) -> bool {
        let validation = Validation {
            algorithms: vec![Algorithm::RS256],
            validate_nbf: true,
            validate_exp: true,
            leeway: 60,
            aud: None,
            iss: None,
            sub: None,
        };

        match jsonwebtoken::decode::<Claims>(&access_token, &*DECODING_KEY, &validation) {
            Ok(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::User;

    #[test]
    fn verify_password_with_correct_password_should_return_true() {
        let new_user = User::new("test", "test@example.com", "password");
        let user = User {
            id: 0,
            uuid: new_user.uuid,
            name: new_user.name,
            email: new_user.email,
            password: new_user.password,
            created_at: new_user.created_at,
        };

        let result = user.verify_password("password");
        assert_eq!(true, result);
    }

    #[test]
    fn verify_password_with_incorrect_password_should_return_false() {
        let new_user = User::new("test", "test@example.com", "password");
        let user = User {
            id: 0,
            uuid: new_user.uuid,
            name: new_user.name,
            email: new_user.email,
            password: new_user.password,
            created_at: new_user.created_at,
        };

        let result = user.verify_password("something");
        assert_eq!(false, result);
    }
}
