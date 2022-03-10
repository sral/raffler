#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::serde::{json::Json, Serialize};
use rocket_db_pools::{sqlx, Connection};

mod db;

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct LocationResponse {
    id: i64,
    name: String,
}

#[get("/")]
async fn get_all_locations(db: Connection<db::Db>) -> Result<Json<Vec<LocationResponse>>> {
    let locations = db::Location::find_all(db).await?;

    let resp = locations
        .into_iter()
        .map(|l| LocationResponse {
            id: l.id,
            name: l.name,
        })
        .collect();

    Ok(Json(resp))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/v1/locations", routes![get_all_locations,])
}
