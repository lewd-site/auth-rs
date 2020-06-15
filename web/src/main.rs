#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket_contrib;

use diesel::pg::PgConnection;
use rocket_contrib::serve::StaticFiles;

#[database("pgsql_chat")]
pub struct ChatDbConn(PgConnection);

fn rocket() -> rocket::Rocket {
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../www");

    rocket::ignite()
        .mount("/", StaticFiles::from(static_dir))
        .attach(ChatDbConn::fairing())
}

fn main() {
    let rocket = rocket();
    rocket.launch();
}
