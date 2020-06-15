use crate::models::users::User;
use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use std::fs;

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

pub struct AccessToken(String);

impl AccessToken {
    pub fn new(user: &User) -> AccessToken {
        let header = Header::new(Algorithm::RS256);

        let now = Utc::now();
        let claims = Claims {
            user_uuid: user.uuid.clone(),
            user_name: user.name.clone(),
            user_email: user.email.clone(),
            iat: now,
            nbf: now,
            exp: now + Duration::hours(1),
        };

        AccessToken(jsonwebtoken::encode(&header, &claims, &*ENCODING_KEY).unwrap())
    }

    pub fn validate(&self) -> bool {
        let validation = Validation {
            algorithms: vec![Algorithm::RS256],
            validate_nbf: true,
            validate_exp: true,
            leeway: 60,
            aud: None,
            iss: None,
            sub: None,
        };

        match jsonwebtoken::decode::<Claims>(&self.0, &*DECODING_KEY, &validation) {
            Ok(_) => true,
            _ => false,
        }
    }
}

impl Display for AccessToken {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

pub struct RefreshToken(String);

impl RefreshToken {
    pub fn new() -> RefreshToken {
        let token = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        RefreshToken(token)
    }
}

impl Display for RefreshToken {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}
