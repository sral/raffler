use rocket::fairing::{self, AdHoc};
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{futures, Build, Rocket};

use rocket_db_pools::{sqlx, Connection, Database};

// use chrono;

use sqlx::Acquire;

use futures::stream::TryStreamExt;

#[derive(Database)]
#[database("sqlx")]
pub struct Db(sqlx::SqlitePool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

// API requests. Should be moved out of DB.

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AddLocationRequest {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AddGameRequest {
    name: String,
    abbreviation: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UpdateGameRequest {
    name: String,
    abbreviation: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AddNoteRequest {
    note: String,
}

// API responses. Should be moved out of DB.

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct IdResponse {
    id: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct LocationResponse {
    id: i64,
    name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct GameResponse {
    id: i64,
    name: String,
    abbreviation: String,
    disabled: bool,
    reserved: bool,
    reserved_for_minutes: i64,
    notes: Vec<NoteResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct NoteResponse {
    id: i64,
    note: String,
    // created_at: String,
}

// Database entites

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Location {
    id: i64,
    name: String,
    deleted: bool,
    // created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Game {
    id: i64,
    location_id: i64,
    name: String,
    abbreviation: String,
    disabled: bool,
    // deleted: bool,
    // created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Note {
    id: i64,
    game_id: i64,
    // player_id: i64,
    note: String,
    // deleted: bool,
    // created_at: chrono::DateTime<chrono::Utc>,
}

#[get("/")]
async fn get_all_locations(mut db: Connection<Db>) -> Result<Json<Vec<LocationResponse>>> {
    let mut tx = db.begin().await?;
    let locations_response = sqlx::query_as!(
        LocationResponse,
        r#"SELECT id, name
             FROM location
            WHERE deleted IS FALSE"#
    )
    .fetch(&mut tx)
    .try_collect::<Vec<_>>()
    .await?;

    tx.commit().await?;
    Ok(Json(locations_response))
}

#[get("/<location_id>")]
async fn get_location_by_id(
    mut db: Connection<Db>,
    location_id: i64,
) -> Result<Json<LocationResponse>> {
    let mut tx = db.begin().await?;
    let locations_response = sqlx::query_as!(
        LocationResponse,
        r#"SELECT id, name
             FROM location
            WHERE deleted IS FALSE
              AND id = ?"#,
        location_id
    )
    .fetch_one(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(Json(locations_response))
}

#[post("/", format = "application/json", data = "<request>")]
async fn add_location(
    mut db: Connection<Db>,
    request: Json<AddLocationRequest>,
) -> Result<Created<Json<IdResponse>>> {
    let mut tx = db.begin().await?;
    let locaiton_id = sqlx::query!("INSERT INTO location (name) VALUES (?)", request.name)
        .execute(&mut tx)
        .await?
        .last_insert_rowid();

    tx.commit().await?;
    Ok(Created::new("/").body(Json(IdResponse { id: locaiton_id })))
}

#[delete("/<location_id>")]
async fn delete_location_by_id(mut db: Connection<Db>, location_id: i64) -> Result<Option<()>> {
    // TODO:
    // - Needs authorization
    // - Neeeds to potentially mark related data as deleted
    let mut tx = db.begin().await?;
    let result = sqlx::query!(
        r#"UPDATE location
              SET deleted = TRUE
            WHERE id = ?"#, location_id)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;

    // Fix this return value, don't use result?
    Ok((result.rows_affected() == 1).then(|| ()))
}

#[get("/<location_id>/games")]
async fn get_games_by_location_id(
    mut db: Connection<Db>,
    location_id: i64,
) -> Result<Json<Vec<GameResponse>>> {
    let mut tx = db.begin().await?;
    let games = sqlx::query_as!(
        Game,
        r#"SELECT id, location_id, name, abbreviation, disabled
             FROM game
            WHERE deleted IS FALSE
              AND location_id = ?
         ORDER BY abbreviation ASC"#,
        location_id
    )
    .fetch(&mut tx)
    .try_collect::<Vec<_>>()
    .await?;

    // Can this query be nested and GameResponse be constructed instead of first
    // consutrcting Game?
    let mut game_response: Vec<GameResponse> = Vec::new();
    for r in games {
        game_response.push(GameResponse {
            id: r.id,
            name: r.name,
            abbreviation: r.abbreviation,
            disabled: r.disabled,
            // TODO: Fix me
            reserved: false,
            reserved_for_minutes: 0,
            // END TODO
            notes: sqlx::query_as!(
                NoteResponse,
                r#"SELECT id, note
                     FROM note
                    WHERE deleted IS FALSE
                      AND game_id = ?"#,
                //ORDER BY created_at DESC"#,
                r.id
            )
            .fetch(&mut *tx)
            .try_collect::<Vec<_>>()
            .await?,
        })
    }

    tx.commit().await?;

    Ok(Json(game_response))
}

#[get("/<location_id>/games/<game_id>")]
async fn get_game_at_location_by_id(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
) -> Result<Json<GameResponse>> {
    let mut tx = db.begin().await?;

    let game = sqlx::query_as!(
        Game,
        r#"SELECT id, location_id, name, abbreviation, disabled
             FROM game
            WHERE deleted IS FALSE
              AND location_id = ?
              AND id = ?"#,
        location_id,
        game_id
    )
    .fetch_one(&mut tx)
    .await?;

    // Can this query be nested and GameResponse be constructed instead of first
    // consutrcting Game?
    let notes = sqlx::query_as!(
        NoteResponse,
        r#"SELECT id, note
             FROM note
            WHERE deleted IS FALSE
              AND game_id = ?"#,
        // ORDER BY created_at DESC"#,
        game.id
    )
    .fetch(&mut *tx)
    .try_collect::<Vec<_>>()
    .await?;

    tx.commit().await?;

    Ok(Json(GameResponse {
        id: game.id,
        name: game.name,
        abbreviation: game.abbreviation,
        disabled: game.disabled,
        // TODO: Fix me
        reserved: false,
        reserved_for_minutes: 0,
        // END TODO
        notes: notes,
    }))
}

#[post(
    "/<location_id>/games",
    format = "application/json",
    data = "<request>"
)]
async fn add_game_at_location(
    mut db: Connection<Db>,
    location_id: i64,
    request: Json<AddGameRequest>,
) -> Result<Created<Json<IdResponse>>> {
    let mut tx = db.begin().await?;

    let game_id = sqlx::query!(
        r#"INSERT INTO game (location_id, name, abbreviation)
                VALUES (?, ?, ?)"#,
        location_id,
        request.name,
        request.abbreviation
    )
    .execute(&mut tx)
    .await?
    .last_insert_rowid();

    tx.commit().await?;
    Ok(Created::new("/").body(Json(IdResponse { id: game_id })))
}

#[put(
    "/<location_id>/games/<game_id>",
    format = "application/json",
    data = "<request>"
)]
async fn update_game_at_location(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
    request: Json<UpdateGameRequest>,
) -> Result<Option<()>> {
    let mut tx = db.begin().await?;

    let result = sqlx::query!(
        r#"UPDATE game
              SET name = ?,
                  abbreviation = ?
            WHERE location_id = ?
              AND id = ?"#,
        request.name,
        request.abbreviation,
        location_id,
        game_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    // Fix this return value, don't use result?
    Ok((result.rows_affected() == 1).then(|| ()))
}

#[get("/<location_id>/games/<game_id>/disable")]
async fn disable_game_at_location_by_id(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
) -> Result<Option<()>> {
    // - Potentially needs to mark related data as deleted?
    let mut tx = db.begin().await?;

    let result = sqlx::query!(
        r#"UPDATE game
              SET disabled = TRUE
            WHERE location_id = ?
              AND id = ?"#,
        location_id,
        game_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    // Fix this return value, don't use result?
    Ok((result.rows_affected() == 1).then(|| ()))
}

#[get("/<location_id>/games/<game_id>/enable")]
async fn enable_game_at_location_by_id(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
) -> Result<Option<()>> {
    // - Potentially needs to mark related data as deleted?
    let mut tx = db.begin().await?;

    let result = sqlx::query!(
        r#"UPDATE game
              SET disabled = FALSE
            WHERE location_id = ?
              AND id = ?"#,
        location_id,
        game_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    // Fix this return value, don't use result?
    Ok((result.rows_affected() == 1).then(|| ()))
}


#[delete("/<location_id>/games/<game_id>")]
async fn delete_game_at_location_by_id(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
) -> Result<Option<()>> {
    // - Potentially needs to mark related data as deleted?
    let mut tx = db.begin().await?;

    let result = sqlx::query!(
        r#"UPDATE game
              SET deleted = TRUE
            WHERE location_id = ?
              AND id = ?"#,
        location_id,
        game_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    // Fix this return value, don't use result?
    Ok((result.rows_affected() == 1).then(|| ()))
}

#[post(
    "/<location_id>/games/<game_id>/notes",
    format = "application/json",
    data = "<request>"
)]
async fn add_note_for_game_at_location(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
    request: Json<AddNoteRequest>,
) -> Result<Created<Json<IdResponse>>> {
    let mut tx = db.begin().await?;

    // TODO: PlayerId needs to come from authorization/sessions.

    let game_id = sqlx::query!(
        r#"INSERT INTO note (note, game_id )
                VALUES (?, ?)"#,
        request.note,
        game_id,
    )
    .execute(&mut tx)
    .await?
    .last_insert_rowid();

    tx.commit().await?;
    Ok(Created::new("/").body(Json(IdResponse { id: game_id })))
}

#[delete("/<location_id>/games/<game_id>/notes/<note_id>")]
async fn delete_note_for_game_by_id(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
    note_id: i64,
) -> Result<Option<()>> {
    // TODO:
    // - Needs to authorize
    // - Nedes to mark all related data as deleted
    let mut tx = db.begin().await?;

    let result = sqlx::query!(
        r#"UPDATE note
              SET deleted = TRUE
            WHERE game_id = ?
              AND id = ?"#,
        game_id,
        note_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    // Fix this return value, don't use result?
    Ok((result.rows_affected() == 1).then(|| ()))
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx migrations", run_migrations))
            .mount(
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
                ],
            )
    })
}
