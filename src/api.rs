use crate::db;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    response::Json,
};
use serde::Deserialize;
use sqlx::postgres::PgPool;

// use chrono::prelude::*;

#[derive(Debug, Deserialize)]
pub struct AddLocationRequest {
    name: String,
}

// #[derive(Debug, Clone, Serialize)]
// pub struct LocationResponse {
//     id: i64,
//     name: String,
// }

// impl LocationResponse {
//     fn from(location: db::Location) -> LocationResponse {
//         LocationResponse {
//             id: location.id,
//             name: location.name,
//         }
//     }
// }

// impl LocationResponse {
//     fn from_vec(locations: Vec<db::Location>) -> Vec<LocationResponse> {
//         locations
//             .into_iter()
//             .map(|l| LocationResponse {
//                 id: l.id,
//                 name: l.name,
//             })
//             .collect()
//     }
// }

// #[derive(Debug, Clone, Deserialize)]
// pub struct AddGameRequest {
//     name: String,
//     abbreviation: String,
// }

// #[derive(Debug, Clone, Serialize)]
// pub struct GameResponse {
//     id: i64,
//     name: String,
//     abbreviation: String,
// }

// impl GameResponse {
//     fn from(game: db::Game) -> GameResponse {
//         GameResponse {
//             id: game.id,
//             name: game.name,
//             abbreviation: game.abbreviation,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize)]
// pub struct GameWithNotesResponse {
//     id: i64,
//     name: String,
//     abbreviation: String,
//     disabled_at: Option<NaiveDateTime>,
//     reserved_at: Option<NaiveDateTime>,
//     reserved_minutes: i32,
//     notes: Vec<NoteResponse>,
// }

// impl GameWithNotesResponse {
//     fn from(game: db::GameWithNotes) -> GameWithNotesResponse {
//         GameWithNotesResponse {
//             id: game.id,
//             name: game.name,
//             abbreviation: game.abbreviation,
//             disabled_at: game.disabled_at,
//             reserved_at: game.reserved_at,
//             reserved_minutes: game.reserved_minutes,
//             notes: NoteResponse::from_vec(game.notes),
//         }
//     }
// }

// #[derive(Debug, Clone, Deserialize)]
// pub struct UpdateGameRequest {
//     name: String,
//     abbreviation: String,
// }

// #[derive(Debug, Clone, Deserialize)]
// pub struct AddNoteRequest {
//     note: String,
// }

// #[derive(Debug, Clone, Serialize)]
// pub struct NoteResponse {
//     id: i64,
//     note: String,
// }

// impl NoteResponse {
//     fn from(note: db::Note) -> NoteResponse {
//         NoteResponse {
//             id: note.id,
//             note: note.note,
//         }
//     }

//     fn from_vec(notes: Vec<db::Note>) -> Vec<NoteResponse> {
//         notes
//             .into_iter()
//             .map(|n| NoteResponse {
//                 id: n.id,
//                 note: n.note,
//             })
//             .collect()
//     }
// }

pub async fn get_all_locations(State(pool): State<PgPool>) -> impl IntoResponse {
    let locations = db::Location::find_all(&pool).await;

    match locations {
        Ok(locations) => Ok(Json(locations)),
        Err(_e) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_location_by_id(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
) -> impl IntoResponse {
    let location = db::Location::find_by_id(&pool, location_id).await;

    match location {
        Ok(location) => Ok(Json(location)),
        Err(_e) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn post_add_location(
    State(pool): State<PgPool>,
    Json(payload): Json<AddLocationRequest>,
) -> impl IntoResponse {
    let location = db::Location::add(&pool, payload.name).await;

    match location {
        Ok(location) => Ok(Json(location)),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn delete_location_by_id(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
) -> impl IntoResponse {
    let location = db::Location::delete_by_id(&pool, location_id).await;

    match location {
        Ok(location) => Ok(Json(location)),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

// #[get("/<location_id>/games")]
// async fn get_games_by_location_id(
//     db: Connection<db::Db>,
//     location_id: i64,
// ) -> Result<Json<Vec<GameWithNotesResponse>>, Status> {
//     let games = db::Game::find_by_location_id(db, location_id).await;
//     match games {
//         Ok(games) => {
//             let mut response = Vec::new();
//             for g in games {
//                 response.push(GameWithNotesResponse::from(g));
//             }
//             Ok(Json(response))
//         }
//         _ => Err(Status::NotFound),
//     }
// }

// async fn get_game_at_location_by_id(
//     db: Connection<db::Db>,
//     location_id: i64,
//     game_id: i64,
// ) -> Result<Json<GameWithNotesResponse>, Status> {
//     let game = db::Game::find_by_id(db, game_id, location_id).await;

//     match game {
//         Ok(game) => Ok(Json(GameWithNotesResponse::from(game))),
//         _ => Err(Status::NotFound),
//     }
// }

// async fn add_game_at_location(
//     db: Connection<db::Db>,
//     location_id: i64,
//     request: Json<AddGameRequest>,
// ) -> Result<Created<Json<GameResponse>>, Status> {
//     let game = db::Game::add(
//         db,
//         location_id,
//         request.name.to_owned(),
//         request.abbreviation.to_owned(),
//     )
//     .await;

//     match game {
//         Ok(game) => {
//             let path = format!("/v1/locations/{}/games/{}", location_id, game.id);
//             let response = GameResponse::from(game);
//             Ok(Created::new(path).body(Json(response)))
//         }
//         _ => Err(Status::BadRequest),
//     }
// }

// async fn update_game_at_location(
//     db: Connection<db::Db>,
//     game_id: i64,
//     location_id: i64,
//     request: Json<UpdateGameRequest>,
// ) -> Result<Json<GameResponse>, Status> {
//     let game = db::Game::update_by_id(
//         db,
//         game_id,
//         location_id,
//         request.name.to_owned(),
//         request.abbreviation.to_owned(),
//     )
//     .await;

//     match game {
//         Ok(game) => Ok(Json(GameResponse::from(game))),
//         _ => Err(Status::BadRequest),
//     }
// }

// async fn disable_game_at_location_by_id(
//     db: Connection<db::Db>,
//     location_id: i64,
//     game_id: i64,
// ) -> Result<Json<GameResponse>, Status> {
//     let game = db::Game::disable_by_id(db, game_id, location_id).await;

//     match game {
//         Ok(game) => Ok(Json(GameResponse::from(game))),
//         _ => Err(Status::BadRequest),
//     }
// }

// async fn enable_game_at_location_by_id(
//     db: Connection<db::Db>,
//     game_id: i64,
//     location_id: i64,
// ) -> Result<Json<GameResponse>, Status> {
//     let game = db::Game::enable_by_id(db, game_id, location_id).await;

//     match game {
//         Ok(game) => Ok(Json(GameResponse::from(game))),
//         _ => Err(Status::BadRequest),
//     }
// }

// async fn delete_game_at_location_by_id(
//     db: Connection<db::Db>,
//     game_id: i64,
//     location_id: i64,
// ) -> Result<Option<()>, Status> {
//     let game = db::Game::delete_by_id(db, game_id, location_id).await;

//     match game {
//         Ok(_game) => Ok(Some(())),
//         _ => Err(Status::BadRequest),
//     }
// }

// async fn add_note_for_game_at_location(
//     db: Connection<db::Db>,
//     location_id: i64,
//     game_id: i64,
//     request: Json<AddNoteRequest>,
// ) -> Result<Created<Json<NoteResponse>>, Status> {
//     // TODO API weirdeness: location id is not verified/used.
//     let note = db::Note::add_by_game_id(db, game_id, request.note.to_owned()).await;

//     match note {
//         Ok(note) => {
//             let path = format!(
//                 "/v1/locations/{}/games/{}/notes/{}",
//                 location_id, game_id, note.id
//             );
//             let response = NoteResponse::from(note);
//             Ok(Created::new(path).body(Json(response)))
//         }
//         _ => Err(Status::BadRequest),
//     }
// }

// async fn delete_note_for_game_by_id(
//     db: Connection<db::Db>,
//     note_id: i64,
// ) -> Result<Option<()>, Status> {
//     // TODO API weirdeness: location id is not verified/used.

//     let note = db::Note::delete_by_id(db, note_id).await;

//     match note {
//         Ok(_note) => Ok(Some(())),
//         _ => Err(Status::BadRequest),
//     }
// }

// async fn reserve_game_at_location_by_id(
//     db: Connection<db::Db>,
//     game_id: i64,
//     location_id: i64,
// ) -> Result<Created<Json<GameResponse>>, Status> {
//     let game = db::Game::reserve_by_id(db, game_id, location_id).await;

//     match game {
//         Ok(game) => {
//             let path = format!("/v1/locations/{}/games/{}", location_id, game.id);
//             let response = GameResponse::from(game);
//             Ok(Created::new(path).body(Json(response)))
//         }
//         _ => Err(Status::BadRequest),
//     }
// }

// async fn reserve_random_game_at_location(
//     db: Connection<db::Db>,
//     location_id: i64,
// ) -> Result<Created<Json<GameResponse>>, Status> {
//     let game = db::Game::reserve_random_by_location_id(db, location_id).await;

//     match game {
//         Ok(game) => {
//             let path = format!("/v1/locations/{}/games/{}", location_id, game.id);
//             let response = GameResponse::from(game);
//             Ok(Created::new(path).body(Json(response)))
//         }
//         _ => Err(Status::NotFound),
//     }
// }

// async fn release_game_at_location_by_id(
//     db: Connection<db::Db>,
//     game_id: i64,
//     location_id: i64,
// ) -> Result<Option<()>, Status> {
//     let game = db::Game::release_reservation_by_id(db, game_id, location_id).await;

//     match game {
//         Ok(_game) => Ok(Some(())),
//         _ => Err(Status::BadRequest),
//     }
// }
