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

#[get("/<location_id>")]
async fn get_location_by_id(
    db: Connection<db::Db>,
    location_id: i64,
) -> Result<Json<LocationResponse>> {
    let location = db::Location::find_by_id(db, location_id).await?;

    Ok(Json(LocationResponse {
        id: location.id,
        name: location.name,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .mount("/", FileServer::from(relative!("static")))
        .mount(
            "/v1/locations",
            routes![get_all_locations, get_location_by_id],
        )
}
