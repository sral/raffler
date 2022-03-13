use crate::db;

use chrono::prelude::*;

use rocket::fairing::AdHoc;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::{sqlx, Connection};

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddLocationRequest {
    name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LocationResponse {
    id: i64,
    name: String,
}

impl LocationResponse {
    fn from(location: db::Location) -> LocationResponse {
        LocationResponse {
            id: location.id,
            name: location.name,
        }
    }
}

impl LocationResponse {
    fn from_vec(locations: Vec<db::Location>) -> Vec<LocationResponse> {
        locations
            .into_iter()
            .map(|l| LocationResponse {
                id: l.id,
                name: l.name,
            })
            .collect()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddGameRequest {
    name: String,
    abbreviation: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GameResponse {
    id: i64,
    name: String,
    abbreviation: String,
}

impl GameResponse {
    fn from(game: db::Game) -> GameResponse {
        GameResponse {
            id: game.id,
            name: game.name,
            abbreviation: game.abbreviation,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GameWithNotesResponse {
    id: i64,
    name: String,
    abbreviation: String,
    disabled_at: Option<NaiveDateTime>,
    reserved_at: Option<NaiveDateTime>,
    reserved_minutes: i32,
    notes: Vec<NoteResponse>,
}

impl GameWithNotesResponse {
    fn from(game: db::GameWithNotes) -> GameWithNotesResponse {
        GameWithNotesResponse {
            id: game.id,
            name: game.name,
            abbreviation: game.abbreviation,
            disabled_at: game.disabled_at,
            reserved_at: game.reserved_at,
            reserved_minutes: game.reserved_minutes,
            notes: NoteResponse::from_vec(game.notes),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateGameRequest {
    name: String,
    abbreviation: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddNoteRequest {
    note: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NoteResponse {
    id: i64,
    note: String,
}

impl NoteResponse {
    fn from(note: db::Note) -> NoteResponse {
        NoteResponse {
            id: note.id,
            note: note.note,
        }
    }

    fn from_vec(notes: Vec<db::Note>) -> Vec<NoteResponse> {
        notes
            .into_iter()
            .map(|n| NoteResponse {
                id: n.id,
                note: n.note,
            })
            .collect()
    }
}

#[get("/")]
async fn get_all_locations(db: Connection<db::Db>) -> Result<Json<Vec<LocationResponse>>> {
    let locations = db::Location::find_all(db).await?;
    let response = LocationResponse::from_vec(locations);

    Ok(Json(response))
}

#[get("/<location_id>")]
async fn get_location_by_id(
    db: Connection<db::Db>,
    location_id: i64,
) -> Result<Json<LocationResponse>> {
    let location = db::Location::find_by_id(db, location_id).await?;
    let response = LocationResponse::from(location);

    Ok(Json(response))
}

#[post("/", format = "application/json", data = "<request>")]
async fn add_location(
    db: Connection<db::Db>,
    request: Json<AddLocationRequest>,
) -> Result<Created<Json<LocationResponse>>> {
    let location = db::Location::add(db, request.name.to_owned()).await?;
    let response = LocationResponse::from(location);

    Ok(Created::new("/").body(Json(response)))
}

#[delete("/<location_id>")]
async fn delete_location_by_id(db: Connection<db::Db>, location_id: i64) -> Result<Option<()>> {
    let _location = db::Location::delete_by_id(db, location_id).await?;

    Ok(Some(()))
}

#[get("/<location_id>/games")]
async fn get_games_by_location_id(
    db: Connection<db::Db>,
    location_id: i64,
) -> Result<Json<Vec<GameWithNotesResponse>>> {
    let games = db::Game::find_by_location_id(db, location_id).await?;

    let mut response = Vec::new();
    for g in games {
        response.push(GameWithNotesResponse::from(g));
    }

    Ok(Json(response))
}

#[get("/<_>/games/<game_id>")]
async fn get_game_at_location_by_id(
    db: Connection<db::Db>,
    game_id: i64,
) -> Result<Json<GameWithNotesResponse>> {
    let game = db::Game::find_by_id(db, game_id).await?;

    Ok(Json(GameWithNotesResponse::from(game)))
}

#[post(
    "/<location_id>/games",
    format = "application/json",
    data = "<request>"
)]
async fn add_game_at_location(
    db: Connection<db::Db>,
    location_id: i64,
    request: Json<AddGameRequest>,
) -> Result<Created<Json<GameResponse>>> {
    let game = db::Game::add(
        db,
        location_id,
        request.name.to_owned(),
        request.abbreviation.to_owned(),
    )
    .await?;
    let response = GameResponse::from(game);

    Ok(Created::new("/").body(Json(response)))
}

#[put(
    "/<_>/games/<game_id>",
    format = "application/json",
    data = "<request>"
)]
async fn update_game_at_location(
    db: Connection<db::Db>,
    game_id: i64,
    request: Json<UpdateGameRequest>,
) -> Result<Json<GameResponse>> {
    let game = db::Game::update_by_id(
        db,
        game_id,
        request.name.to_owned(),
        request.abbreviation.to_owned(),
    )
    .await?;
    let response = GameResponse::from(game);

    Ok(Json(response))
}

#[post("/<_>/games/<game_id>/disable")]
async fn disable_game_at_location_by_id(
    db: Connection<db::Db>,
    game_id: i64,
) -> Result<Json<GameResponse>> {
    let game = db::Game::disable_by_id(db, game_id).await?;
    let response = GameResponse::from(game);

    Ok(Json(response))
}

#[post("/<_>/games/<game_id>/enable")]
async fn enable_game_at_location_by_id(
    db: Connection<db::Db>,
    game_id: i64,
) -> Result<Json<GameResponse>> {
    let game = db::Game::enable_by_id(db, game_id).await?;
    let response = GameResponse::from(game);

    Ok(Json(response))
}

#[delete("/<_>/games/<game_id>")]
async fn delete_game_at_location_by_id(db: Connection<db::Db>, game_id: i64) -> Result<Option<()>> {
    db::Game::delete_by_id(db, game_id).await?;

    Ok(Some(()))
}

#[post(
    "/<_>/games/<game_id>/notes",
    format = "application/json",
    data = "<request>"
)]
async fn add_note_for_game_at_location(
    db: Connection<db::Db>,
    game_id: i64,
    request: Json<AddNoteRequest>,
) -> Result<Created<Json<NoteResponse>>> {
    let note = db::Note::add_by_game_id(db, game_id, request.note.to_owned()).await?;
    let response = NoteResponse::from(note);

    Ok(Created::new("/").body(Json(response)))
}

#[delete("/<_>/games/<_>/notes/<note_id>")]
async fn delete_note_for_game_by_id(db: Connection<db::Db>, note_id: i64) -> Result<Option<()>> {
    db::Note::delete_by_id(db, note_id).await?;

    Ok(Some(()))
}

#[post("/<_>/games/<game_id>/reservations")]
async fn reserve_game_at_location_by_id(
    db: Connection<db::Db>,
    game_id: i64,
) -> Result<Created<Json<GameResponse>>> {
    let game = db::Game::reserve_by_id(db, game_id).await?;
    let response = GameResponse::from(game);

    Ok(Created::new("/").body(Json(response)))
}

#[delete("/<_>/games/<game_id>/reservations")]
async fn release_game_at_location_by_id(
    db: Connection<db::Db>,
    game_id: i64,
) -> Result<Json<GameResponse>> {
    let game = db::Game::release_reservation_by_id(db, game_id).await?;
    let response = GameResponse::from(game);

    Ok(Json(response))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("API Stage", |rocket| async {
        rocket.mount(
            "/v1/locations",
            routes![
                get_all_locations,
                get_location_by_id,
                add_location,
                delete_location_by_id,
                get_games_by_location_id,
                get_game_at_location_by_id,
                add_game_at_location,
                update_game_at_location,
                disable_game_at_location_by_id,
                enable_game_at_location_by_id,
                delete_game_at_location_by_id,
                add_note_for_game_at_location,
                delete_note_for_game_by_id,
                reserve_game_at_location_by_id,
                release_game_at_location_by_id,
            ],
        )
    })
}