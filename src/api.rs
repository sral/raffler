use crate::db;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

// API requests
#[derive(Debug, Deserialize)]
pub struct LocationRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct GameRequest {
    name: String,
    abbreviation: String,
}

#[derive(Debug, Deserialize)]
pub struct NoteRequest {
    note: String,
}

// API responses
#[derive(Debug, Serialize)]
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
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct GameWithNotesResponse {
    id: i64,
    name: String,
    abbreviation: String,
    notes: Vec<NoteResponse>,
}
impl GameWithNotesResponse {
    fn from(game_with_notes: db::GameWithNotes) -> GameWithNotesResponse {
        GameWithNotesResponse {
            id: game_with_notes.id,
            name: game_with_notes.name,
            abbreviation: game_with_notes.abbreviation,
            notes: game_with_notes
                .notes
                .into_iter()
                .map(|n| NoteResponse::from(n))
                .collect(),
        }
    }

    fn from_vec(games_with_notes: Vec<db::GameWithNotes>) -> Vec<GameWithNotesResponse> {
        games_with_notes
            .into_iter()
            .map(|g| GameWithNotesResponse::from(g))
            .collect()
    }
}

#[derive(Debug, Serialize)]
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

pub async fn get_all_locations(State(pool): State<PgPool>) -> impl IntoResponse {
    let locations = db::Location::find_all(&pool).await;

    match locations {
        Ok(locations) => Ok(Json(LocationResponse::from_vec(locations))),
        Err(_e) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_location_by_id(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
) -> impl IntoResponse {
    let location = db::Location::find_by_id(&pool, location_id).await;

    match location {
        Ok(location) => Ok(Json(LocationResponse::from(location))),
        Err(_e) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn post_add_location(
    State(pool): State<PgPool>,
    Json(payload): Json<LocationRequest>,
) -> impl IntoResponse {
    let location = db::Location::add(&pool, payload.name).await;

    match location {
        Ok(location) => Ok(Json(LocationResponse::from(location))),
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
        Ok(location) => Ok(Json(LocationResponse::from(location))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn get_games_by_location_id(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
) -> impl IntoResponse {
    let games_with_notes = db::Game::find_by_location_id(&pool, location_id).await;
    match games_with_notes {
        Ok(games_with_notes) => Ok(Json(GameWithNotesResponse::from_vec(games_with_notes))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::NOT_FOUND);
        }
    }
}

pub async fn get_game_at_location_by_id(
    State(pool): State<PgPool>,
    Path((location_id, game_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let game_with_notes = db::Game::find_by_id(&pool, game_id, location_id).await;

    match game_with_notes {
        Ok(game_with_notes) => Ok(Json(GameWithNotesResponse::from(game_with_notes))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::NOT_FOUND);
        }
    }
}

pub async fn post_add_game_at_location(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
    Json(payload): Json<GameRequest>,
) -> impl IntoResponse {
    let game = db::Game::add(&pool, location_id, payload.name, payload.abbreviation).await;

    match game {
        Ok(game) => Ok(Json(GameResponse::from(game))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn put_update_game_at_location(
    State(pool): State<PgPool>,
    Path((location_id, game_id)): Path<(i64, i64)>,
    Json(payload): Json<GameRequest>,
) -> impl IntoResponse {
    let game = db::Game::update_by_id(
        &pool,
        location_id,
        game_id,
        payload.name,
        payload.abbreviation,
    )
    .await;

    match game {
        Ok(game) => Ok(Json(GameResponse::from(game))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn post_reserve_random_game_at_location(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
) -> impl IntoResponse {
    let game = db::Game::reserve_random_by_location_id(&pool, location_id).await;

    match game {
        Ok(game) => Ok(Json(GameResponse::from(game))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn delete_game_reservation_at_location_by_id(
    State(pool): State<PgPool>,
    Path((location_id, game_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let game = db::Game::release_reservation_by_id(&pool, location_id, game_id).await;

    match game {
        Ok(game) => Ok(Json(GameResponse::from(game))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn post_disable_game_at_location_by_id(
    State(pool): State<PgPool>,
    Path((location_id, game_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let game = db::Game::disable_by_id(&pool, location_id, game_id).await;

    match game {
        Ok(game) => Ok(Json(GameResponse::from(game))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn post_enable_game_at_location_by_id(
    State(pool): State<PgPool>,
    Path((location_id, game_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let game = db::Game::enable_by_id(&pool, location_id, game_id).await;

    match game {
        Ok(game) => Ok(Json(GameResponse::from(game))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn delete_game_at_location_by_id(
    State(pool): State<PgPool>,
    Path((location_id, game_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let game = db::Game::delete_by_id(&pool, location_id, game_id).await;

    match game {
        Ok(game) => Ok(Json(GameResponse::from(game))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn post_add_note_for_game_at_location(
    State(pool): State<PgPool>,
    Path((_location_id, game_id)): Path<(i64, i64)>,
    Json(payload): Json<NoteRequest>,
) -> impl IntoResponse {
    // TODO API weirdeness: location id is not verified/used.
    let note = db::Note::add_by_game_id(&pool, payload.note, game_id).await;

    match note {
        Ok(note) => Ok(Json(NoteResponse::from(note))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn delete_note_for_game_by_id(
    State(pool): State<PgPool>,
    Path((_location_id, game_id, note_id)): Path<(i64, i64, i64)>,
) -> impl IntoResponse {
    // TODO API weirdeness: location_id is not verified/used.
    let note = db::Note::delete_by_id(&pool, game_id, note_id).await;

    match note {
        Ok(note) => Ok(Json(NoteResponse::from(note))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn post_reserve_game_at_location_by_id(
    State(pool): State<PgPool>,
    Path((location_id, game_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let game = db::Game::reserve_by_id(&pool, location_id, game_id).await;

    match game {
        Ok(game) => Ok(Json(GameResponse::from(game))),
        Err(e) => {
            tracing::debug!("Error {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}
