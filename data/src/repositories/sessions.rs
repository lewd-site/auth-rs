use crate::models::sessions::{NewSession, Session};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct SessionRepository();

impl SessionRepository {
    pub fn get_by_token(conn: &PgConnection, uid: i32, token: &str) -> Option<Session> {
        use crate::schema::sessions::dsl::*;

        let items: Vec<Session> = sessions
            .filter(user_id.eq(uid))
            .filter(refresh_token.eq(token))
            .limit(1)
            .load(conn)
            .unwrap();

        items.into_iter().next()
    }

    pub fn create(conn: &PgConnection, session: &NewSession) -> Session {
        use crate::schema::sessions::dsl::*;

        diesel::insert_into(sessions)
            .values(session)
            .get_result(conn)
            .unwrap()
    }

    pub fn delete(conn: &PgConnection, session: &Session) -> Session {
        use crate::schema::sessions::dsl::*;

        let source = sessions.filter(id.eq(session.id));
        diesel::delete(source).get_result(conn).unwrap()
    }
}
