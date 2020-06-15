use crate::schema::users;
use chrono::prelude::*;
use pwhash::bcrypt;
use uuid::Uuid;

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
