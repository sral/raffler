#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};

mod db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .mount("/", FileServer::from(relative!("static")))
}
