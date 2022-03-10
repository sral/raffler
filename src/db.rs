use rocket::fairing::{self, AdHoc};
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{futures, Build, Rocket};

use rocket_db_pools::{sqlx, Connection, Database};

use chrono::prelude::*;

use sqlx::Acquire;

use futures::stream::TryStreamExt;

#[derive(Database)]
#[database("sqlx")]
pub struct Db(sqlx::PgPool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

// API requests. Should be moved out of DB.

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
    disabled_at: Option<NaiveDateTime>,
    reserved_at: Option<NaiveDateTime>,
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
pub struct Location {
    pub id: i64,
    pub name: String,
    deleted_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
}

impl Location {
    pub async fn find_all(mut db: Connection<Db>) -> Result<Vec<Location>> {
        let mut tx = db.begin().await?;
        let locations = sqlx::query_as!(
            Location,
            r#"SELECT * FROM location
                WHERE deleted_at IS NULL"#
        )
        .fetch(&mut tx)
        .try_collect::<Vec<_>>()
        .await?;

        tx.commit().await?;
        Ok(locations)
    }

    pub async fn find_by_id(mut db: Connection<Db>, id: i64) -> Result<Location> {
        let mut tx = db.begin().await?;
        let location = sqlx::query_as!(
            Location,
            r#"SELECT *
                 FROM location
                WHERE deleted_at IS NULL
                  AND id = $1"#,
            id
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(location)
    }

    pub async fn add_location(mut db: Connection<Db>, name: String) -> Result<Location> {
        let mut tx = db.begin().await?;
        let location = sqlx::query_as!(
            Location,
            "INSERT INTO location (name)
                  VALUES ($1)
               RETURNING *",
            name
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(location)
    }

    pub async fn delete_by_id(mut db: Connection<Db>, id: i64) -> Result<Location> {
        let mut tx = db.begin().await?;

        let location = sqlx::query_as!(
            Location,
            r#"UPDATE location
                  SET deleted_at = now()
                WHERE id = $1
            RETURNING *"#,
            id
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(location)
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Game {
    id: i64,
    location_id: i64,
    name: String,
    abbreviation: String,
    disabled_at: Option<NaiveDateTime>,
    reserved_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Note {
    id: i64,
    game_id: i64,
    // Nullable for now, fix once we add players to the mix.
    player_id: Option<i64>,
    note: String,
    deleted_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
}

#[get("/<location_id>/games")]
async fn get_games_by_location_id(
    mut db: Connection<Db>,
    location_id: i64,
) -> Result<Json<Vec<GameResponse>>> {
    let mut tx = db.begin().await?;
    let games = sqlx::query_as!(
        Game,
        r#"SELECT *
             FROM game
            WHERE deleted_at IS NULL
              AND location_id = $1
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
            disabled_at: r.disabled_at,
            // TODO: Fix me
            reserved_at: r.reserved_at,
            reserved_for_minutes: 0,
            // END TODO
            notes: sqlx::query_as!(
                NoteResponse,
                r#"SELECT id, note
                     FROM note
                    WHERE deleted_at IS NULL
                      AND game_id = $1
                 ORDER BY created_at ASC"#,
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
        r#"SELECT *
             FROM game
            WHERE deleted_at IS NULL
              AND location_id = $1
              AND id = $2"#,
        location_id,
        game_id
    )
    .fetch_one(&mut tx)
    .await?;

    let note_response = sqlx::query_as!(
        NoteResponse,
        r#"SELECT id, note
             FROM note
            WHERE deleted_at IS NULL
              AND game_id = $1
         ORDER BY created_at ASC"#,
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
        disabled_at: game.disabled_at,
        reserved_at: game.reserved_at,
        // TODO: Fix me.
        reserved_for_minutes: 0,
        // END TODO
        notes: note_response,
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

    let game = sqlx::query_as!(
        Game,
        r#"INSERT INTO game (location_id, name, abbreviation)
                VALUES ($1, $2, $3)
             RETURNING *"#,
        location_id,
        request.name,
        request.abbreviation
    )
    .fetch_one(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(Created::new("/").body(Json(IdResponse { id: game.id })))
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
              SET name = $1,
                  abbreviation = $2
            WHERE location_id = $3
              AND id = $4"#,
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

#[post("/<location_id>/games/<game_id>/disable")]
async fn disable_game_at_location_by_id(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
) -> Result<Option<()>> {
    // - Potentially needs to mark related data as deleted?
    let mut tx = db.begin().await?;

    let result = sqlx::query!(
        r#"UPDATE game
              SET disabled_at = now()
            WHERE location_id = $1
              AND id = $2"#,
        location_id,
        game_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    // Fix this return value, don't use result?
    Ok((result.rows_affected() == 1).then(|| ()))
}

#[post("/<location_id>/games/<game_id>/enable")]
async fn enable_game_at_location_by_id(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
) -> Result<Option<()>> {
    // - Potentially needs to mark related data as deleted?
    let mut tx = db.begin().await?;

    let result = sqlx::query!(
        r#"UPDATE game
              SET disabled_at = NULL
            WHERE location_id = $1
              AND id = $2"#,
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
              SET deleted_at = now()
            WHERE location_id = $1
              AND id = $2"#,
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
    "/<_>/games/<game_id>/notes",
    format = "application/json",
    data = "<request>"
)]
async fn add_note_for_game_at_location(
    mut db: Connection<Db>,
    game_id: i64,
    request: Json<AddNoteRequest>,
) -> Result<Created<Json<IdResponse>>> {
    let mut tx = db.begin().await?;

    // TODO: PlayerId needs to come from authorization/sessions.

    let note = sqlx::query_as!(
        Note,
        r#"INSERT INTO note (note, game_id)
                VALUES ($1, $2)
             RETURNING *"#,
        request.note,
        game_id,
    )
    .fetch_one(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(Created::new("/").body(Json(IdResponse { id: note.id })))
}

#[delete("/<_>/games/<game_id>/notes/<note_id>")]
async fn delete_note_for_game_by_id(
    mut db: Connection<Db>,
    game_id: i64,
    note_id: i64,
) -> Result<Option<()>> {
    // TODO:
    // - Needs to authorize
    // - Nedes to mark all related data as deleted
    let mut tx = db.begin().await?;

    let result = sqlx::query!(
        r#"UPDATE note
              SET deleted_at = now()
            WHERE game_id = $1
              AND id = $2"#,
        game_id,
        note_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    // Fix this return value, don't use result?
    Ok((result.rows_affected() == 1).then(|| ()))
}

#[post("/<location_id>/games/<game_id>/reservations")]
async fn reserve_game_at_location_by_id(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
) -> Result<Option<()>> {
    let mut tx = db.begin().await?;

    let result = sqlx::query!(
        r#"UPDATE game
              SET reserved_at = now()
            WHERE location_id = $1
              AND id = $2"#,
        location_id,
        game_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    // Fix this return value, don't use result?
    Ok((result.rows_affected() == 1).then(|| ()))
}

#[delete("/<location_id>/games/<game_id>/reservations")]
async fn release_game_at_location_by_id(
    mut db: Connection<Db>,
    location_id: i64,
    game_id: i64,
) -> Result<Option<()>> {
    let mut tx = db.begin().await?;

    let result = sqlx::query!(
        r#"UPDATE game
              SET reserved_at = NULL
            WHERE location_id = $1
              AND id = $2"#,
        location_id,
        game_id
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
