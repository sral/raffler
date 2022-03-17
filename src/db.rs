use chrono::prelude::*;

use rocket::fairing::{self, AdHoc};
use rocket::serde::Serialize;
use rocket::{futures, Build, Rocket};
use rocket_db_pools::{sqlx, Connection, Database};

use sqlx::Acquire;

use futures::stream::TryStreamExt;

#[derive(Database)]
#[database("sqlx")]
pub struct Db(sqlx::PgPool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

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

    pub async fn add(mut db: Connection<Db>, name: String) -> Result<Location> {
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

// Bah! This needs to be fixed. Can we pass around a transaction and compose functions
// in API layer and not do this shit?
#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GameWithNotes {
    pub id: i64,
    location_id: i64,
    pub name: String,
    pub abbreviation: String,
    pub disabled_at: Option<NaiveDateTime>,
    pub reserved_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    pub notes: Vec<Note>,
    pub reserved_minutes: i32,
}

impl GameWithNotes {
    pub fn build(game: Game, notes: Vec<Note>) -> GameWithNotes {
        GameWithNotes {
            id: game.id,
            location_id: game.location_id,
            name: game.name,
            abbreviation: game.abbreviation,
            disabled_at: game.disabled_at,
            reserved_at: game.reserved_at,
            deleted_at: game.deleted_at,
            created_at: game.created_at,
            notes: notes,
            reserved_minutes: game.reserved_minutes,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub id: i64,
    location_id: i64,
    pub name: String,
    pub abbreviation: String,
    pub disabled_at: Option<NaiveDateTime>,
    pub reserved_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    pub reserved_minutes: i32,
}

impl Game {
    pub async fn find_by_id(
        mut db: Connection<Db>,
        id: i64,
        location_id: i64,
    ) -> Result<GameWithNotes> {
        let mut tx = db.begin().await?;
        let game = sqlx::query_as!(
            Game,
            r#"SELECT *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                WHERE deleted_at IS NULL
                  AND id = $1
                  AND location_id = $2"#,
            id,
            location_id
        )
        .fetch_one(&mut *tx)
        .await?;

        let notes = sqlx::query_as!(
            Note,
            r#"SELECT *
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
        Ok(GameWithNotes::build(game, notes))
    }

    pub async fn find_by_location_id(
        mut db: Connection<Db>,
        id: i64,
    ) -> Result<Vec<GameWithNotes>> {
        let mut tx = db.begin().await?;
        let games = sqlx::query_as!(
            Game,
            r#"SELECT *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                WHERE deleted_at IS NULL
                  AND location_id = $1
             ORDER BY abbreviation ASC"#,
            id
        )
        .fetch(&mut tx)
        .try_collect::<Vec<_>>()
        .await?;

        let mut games_with_notes = Vec::new();
        for g in games {
            let notes = sqlx::query_as!(
                Note,
                r#"SELECT *
                     FROM note
                    WHERE deleted_at IS NULL
                      AND game_id = $1
                 ORDER BY created_at ASC"#,
                g.id
            )
            .fetch(&mut *tx)
            .try_collect::<Vec<_>>()
            .await?;
            games_with_notes.push(GameWithNotes::build(g, notes));
        }

        tx.commit().await?;
        Ok(games_with_notes)
    }

    pub async fn add(
        mut db: Connection<Db>,
        location_id: i64,
        name: String,
        abbreviation: String,
    ) -> Result<Game> {
        let mut tx = db.begin().await?;
        let game = sqlx::query_as!(
            Game,
            r#"INSERT INTO game (location_id, name, abbreviation)
                    VALUES ($1, $2, $3)
                 RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            location_id,
            name,
            abbreviation
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn update_by_id(
        mut db: Connection<Db>,
        id: i64,
        location_id: i64,
        name: String,
        abbreviation: String,
    ) -> Result<Game> {
        let mut tx = db.begin().await?;
        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET name = $1,
                      abbreviation = $2
                WHERE id = $3
                  AND location_id = $4
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            name,
            abbreviation,
            id,
            location_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn disable_by_id(mut db: Connection<Db>, id: i64, location_id: i64) -> Result<Game> {
        let mut tx = db.begin().await?;
        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET disabled_at = now()
                WHERE id = $1
                  AND location_id = $2
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
            location_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn enable_by_id(mut db: Connection<Db>, id: i64, location_id: i64) -> Result<Game> {
        let mut tx = db.begin().await?;
        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET disabled_at = NULL
                WHERE id = $1
                  AND location_id = $2
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
            location_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn delete_by_id(mut db: Connection<Db>, id: i64, location_id: i64) -> Result<Game> {
        // - Potentially needs to mark related data as deleted?
        let mut tx = db.begin().await?;
        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET deleted_at = now()
                WHERE id = $1
                  AND location_id = $2
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
            location_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn reserve_by_id(mut db: Connection<Db>, id: i64, location_id: i64) -> Result<Game> {
        let mut tx = db.begin().await?;
        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET reserved_at = now()
                WHERE id = $1
                  AND location_id = $2
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
            location_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn release_reservation_by_id(
        mut db: Connection<Db>,
        id: i64,
        location_id: i64,
    ) -> Result<Game> {
        let mut tx = db.begin().await?;
        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET reserved_at = NULL
                WHERE id = $1
                  AND location_id = $2
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
            location_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Note {
    pub id: i64,
    game_id: i64,
    // Nullable for now, fix once we add players to the mix.
    player_id: Option<i64>,
    pub note: String,
    deleted_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
}

impl Note {
    pub async fn add_by_game_id(mut db: Connection<Db>, id: i64, note: String) -> Result<Note> {
        // TODO: PlayerId needs to come from authorization/sessions.
        let mut tx = db.begin().await?;
        let note = sqlx::query_as!(
            Note,
            r#"INSERT INTO note (note, game_id)
                    VALUES ($1, $2)
                 RETURNING *"#,
            note,
            id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(note)
    }

    pub async fn delete_by_id(mut db: Connection<Db>, id: i64) -> Result<Note> {
        let mut tx = db.begin().await?;
        let note = sqlx::query_as!(
            Note,
            r#"UPDATE note
                  SET deleted_at = now()
                WHERE id = $1
            RETURNING *"#,
            id
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(note)
    }

    // TODO: Ideally we'd like to compose functions in the API layer and use this rather than doing
    // queries above when quering for games but I haven't been able to figure out how to do that yet.

    // pub async fn find_by_game_id(mut db: Connection<Db>, id: i64) -> Result<Vec<Note>> {
    //     let mut tx = db.begin().await?;
    //     let notes = sqlx::query_as!(
    //         Note,
    //         r#"SELECT *
    //              FROM note
    //             WHERE deleted_at IS NULL
    //               AND game_id = $1
    //          ORDER BY created_at ASC"#,
    //         id
    //     )
    //     .fetch(&mut tx)
    //     .try_collect::<Vec<_>>()
    //     .await?;

    //     Ok(notes)
    // }
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
    })
}
