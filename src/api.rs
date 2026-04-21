use chrono::prelude::*;

use crate::db;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;

// API error type
pub(crate) enum ApiError {
    NotFound(String),
    Conflict(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}

fn map_not_found(message: &str) -> impl FnOnce(sqlx::Error) -> ApiError {
    let msg = message.to_string();
    |e| match e {
        sqlx::Error::RowNotFound => ApiError::NotFound(msg),
        _ => {
            tracing::error!("Database error: {e}");
            ApiError::Internal("Internal server error".into())
        }
    }
}

fn map_conflict(message: &str) -> impl FnOnce(sqlx::Error) -> ApiError {
    let msg = message.to_string();
    |e| match e {
        sqlx::Error::RowNotFound => ApiError::Conflict(msg),
        _ => {
            tracing::error!("Database error: {e}");
            ApiError::Internal("Internal server error".into())
        }
    }
}

fn map_internal(e: sqlx::Error) -> ApiError {
    tracing::error!("Database error: {e}");
    ApiError::Internal("Internal server error".into())
}

fn map_db_error(not_found_msg: &str) -> impl FnOnce(db::DbError) -> ApiError {
    let msg = not_found_msg.to_string();
    move |e| match e {
        db::DbError::NotFound => ApiError::NotFound(msg),
        db::DbError::Disabled => ApiError::Conflict("Game is disabled".into()),
        db::DbError::Db(e) => {
            tracing::error!("Database error: {e}");
            ApiError::Internal("Internal server error".into())
        }
    }
}

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
    created_at: NaiveDateTime,
}

impl From<db::Note> for NoteResponse {
    fn from(note: db::Note) -> Self {
        NoteResponse {
            id: note.id,
            note: note.note,
            created_at: note.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ReservationStatsResponse {
    game_id: i64,
    reservation_count: i64,
    reserved_minutes: i64,
    average_reserved_minutes: f64,
    median_reserved_minutes: f64,
}

impl From<db::ReservationStats> for ReservationStatsResponse {
    fn from(stats: db::ReservationStats) -> Self {
        ReservationStatsResponse {
            game_id: stats.game_id,
            reservation_count: stats.reservation_count,
            reserved_minutes: stats.reserved_minutes,
            average_reserved_minutes: stats.average_reserved_minutes,
            median_reserved_minutes: stats.median_reserved_minutes,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GameResponse {
    id: i64,
    name: String,
    abbreviation: String,
}

impl From<db::Game> for GameResponse {
    fn from(game: db::Game) -> Self {
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
    disabled_at: Option<NaiveDateTime>,
    reserved_at: Option<NaiveDateTime>,
    reserved_minutes: i32,
    notes: Vec<NoteResponse>,
}

impl From<db::GameWithNotes> for GameWithNotesResponse {
    fn from(game_with_notes: db::GameWithNotes) -> Self {
        GameWithNotesResponse {
            id: game_with_notes.id,
            name: game_with_notes.name,
            abbreviation: game_with_notes.abbreviation,
            disabled_at: game_with_notes.disabled_at,
            reserved_at: game_with_notes.reserved_at,
            reserved_minutes: game_with_notes.reserved_minutes,
            notes: game_with_notes.notes.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LocationResponse {
    id: i64,
    name: String,
}

impl From<db::Location> for LocationResponse {
    fn from(location: db::Location) -> Self {
        LocationResponse {
            id: location.id,
            name: location.name,
        }
    }
}

pub async fn get_all_locations(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<LocationResponse>>, ApiError> {
    let locations = db::Location::find_all(&pool).await.map_err(map_internal)?;
    Ok(Json(locations.into_iter().map(Into::into).collect()))
}

pub async fn get_location_by_id(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
) -> Result<Json<LocationResponse>, ApiError> {
    let location = db::Location::find_by_id(&pool, location_id)
        .await
        .map_err(map_not_found("Location not found"))?;
    Ok(Json(location.into()))
}

pub async fn post_add_location(
    State(pool): State<PgPool>,
    Json(payload): Json<LocationRequest>,
) -> Result<(StatusCode, Json<LocationResponse>), ApiError> {
    let location = db::Location::add(&pool, payload.name)
        .await
        .map_err(map_internal)?;
    Ok((StatusCode::CREATED, Json(location.into())))
}

pub async fn delete_location_by_id(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
) -> Result<Json<LocationResponse>, ApiError> {
    let location = db::Location::delete_by_id(&pool, location_id)
        .await
        .map_err(map_not_found("Location not found"))?;
    Ok(Json(location.into()))
}

pub async fn get_games_by_location_id(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
) -> Result<Json<Vec<GameWithNotesResponse>>, ApiError> {
    let games_with_notes = db::Game::find_by_location_id(&pool, location_id)
        .await
        .map_err(map_internal)?;
    Ok(Json(games_with_notes.into_iter().map(Into::into).collect()))
}

pub async fn get_game_by_id(
    State(pool): State<PgPool>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameWithNotesResponse>, ApiError> {
    let game_with_notes = db::Game::find_by_id(&pool, game_id)
        .await
        .map_err(map_not_found("Game not found"))?;
    Ok(Json(game_with_notes.into()))
}

pub async fn post_add_game_at_location(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
    Json(payload): Json<GameRequest>,
) -> Result<(StatusCode, Json<GameResponse>), ApiError> {
    let game = db::Game::add(&pool, location_id, payload.name, payload.abbreviation)
        .await
        .map_err(map_not_found("Location not found"))?;
    Ok((StatusCode::CREATED, Json(game.into())))
}

pub async fn put_update_game(
    State(pool): State<PgPool>,
    Path(game_id): Path<i64>,
    Json(payload): Json<GameRequest>,
) -> Result<Json<GameResponse>, ApiError> {
    let game = db::Game::update_by_id(&pool, game_id, payload.name, payload.abbreviation)
        .await
        .map_err(map_not_found("Game not found"))?;
    Ok(Json(game.into()))
}

pub async fn post_reserve_random_game_at_location(
    State(pool): State<PgPool>,
    Path(location_id): Path<i64>,
) -> Result<Json<GameResponse>, ApiError> {
    let game = db::Game::reserve_random_by_location_id(&pool, location_id)
        .await
        .map_err(map_conflict("No available games at this location"))?;
    Ok(Json(game.into()))
}

pub async fn delete_game_reservation_by_id(
    State(pool): State<PgPool>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameResponse>, ApiError> {
    let game = db::Game::release_reservation_by_id(&pool, game_id)
        .await
        .map_err(map_db_error("Game not found"))?;
    Ok(Json(game.into()))
}

pub async fn post_disable_game_by_id(
    State(pool): State<PgPool>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameResponse>, ApiError> {
    let game = db::Game::disable_by_id(&pool, game_id)
        .await
        .map_err(map_db_error("Game not found"))?;
    Ok(Json(game.into()))
}

pub async fn post_enable_game_by_id(
    State(pool): State<PgPool>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameResponse>, ApiError> {
    let game = db::Game::enable_by_id(&pool, game_id)
        .await
        .map_err(map_db_error("Game not found"))?;
    Ok(Json(game.into()))
}

pub async fn delete_game_by_id(
    State(pool): State<PgPool>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameResponse>, ApiError> {
    let game = db::Game::delete_by_id(&pool, game_id)
        .await
        .map_err(map_not_found("Game not found"))?;
    Ok(Json(game.into()))
}

pub async fn post_add_note_for_game(
    State(pool): State<PgPool>,
    Path(game_id): Path<i64>,
    Json(payload): Json<NoteRequest>,
) -> Result<(StatusCode, Json<NoteResponse>), ApiError> {
    let note = db::Note::add_by_game_id(&pool, payload.note, game_id)
        .await
        .map_err(map_not_found("Game not found"))?;
    Ok((StatusCode::CREATED, Json(note.into())))
}

pub async fn delete_note_for_game_by_id(
    State(pool): State<PgPool>,
    Path((game_id, note_id)): Path<(i64, i64)>,
) -> Result<Json<NoteResponse>, ApiError> {
    let note = db::Note::delete_by_id(&pool, game_id, note_id)
        .await
        .map_err(map_not_found("Note not found"))?;
    Ok(Json(note.into()))
}

pub async fn post_reserve_game_by_id(
    State(pool): State<PgPool>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameResponse>, ApiError> {
    let game = db::Game::reserve_by_id(&pool, game_id)
        .await
        .map_err(map_db_error("Game not found"))?;
    Ok(Json(game.into()))
}

pub async fn get_game_reservation_stats(
    State(pool): State<PgPool>,
    Path(game_id): Path<i64>,
) -> Result<Json<ReservationStatsResponse>, ApiError> {
    let stats = db::ReservationStats::get_reservations_stats_by_game_id(&pool, game_id)
        .await
        .map_err(map_not_found("Game not found"))?;
    Ok(Json(stats.into()))
}
