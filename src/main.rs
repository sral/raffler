#[macro_use]
extern crate rocket;

mod api;
mod db;

use rocket::fs::{relative, FileServer};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .attach(api::stage())
        .mount("/", FileServer::from(relative!("static")))
}
