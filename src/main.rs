#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};

// mod api;
mod db;

// use rocket_db_pools::{sqlx, Connection, Database};

// #[get("/")]
// async fn list(mut db: Connection<db::Db>) -> String {
//     let locations = db::Db::frob(db);
//     let r = sqlx::query!("SELECT name FROM location WHERE deleted IS NULL")
//         .fetch_one(&mut *db)
//         .await;

//     match r {
//         Ok(r) => r.name,
//         Err(_) => "frob".to_string()
//     }
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .mount("/", FileServer::from(relative!("static")))
}
