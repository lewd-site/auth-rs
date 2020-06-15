#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod routes;

use diesel::pg::PgConnection;
use rocket_contrib::serve::StaticFiles;
use routes::{tokens, users};

#[database("pgsql_auth")]
pub struct AuthDbConn(PgConnection);

fn rocket() -> rocket::Rocket {
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../www");

    rocket::ignite()
        .mount("/api/v1/tokens", routes![tokens::create_access_token])
        .mount("/api/v1/users", routes![users::register])
        .mount("/", StaticFiles::from(static_dir))
        .attach(AuthDbConn::fairing())
}

fn main() {
    let rocket = rocket();
    rocket.launch();
}
