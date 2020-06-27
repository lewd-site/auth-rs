use crate::schema::sessions;

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub user_id: i32,
    pub refresh_token: String,
}

#[derive(Identifiable, Queryable)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub refresh_token: String,
}

impl Session {
    pub fn new(user_id: i32, refresh_token: &str) -> NewSession {
        NewSession {
            user_id,
            refresh_token: String::from(refresh_token),
        }
    }
}
