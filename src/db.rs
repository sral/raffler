use std::collections::HashMap;

use chrono::prelude::*;

use serde::Serialize;
use sqlx::PgPool;

type Result<T> = std::result::Result<T, sqlx::Error>;

#[derive(Debug)]
pub enum DbError {
    NotFound,
    Disabled,
    Db(sqlx::Error),
}

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => DbError::NotFound,
            other => DbError::Db(other),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    deleted_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
}

impl Location {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Location>> {
        let locations = sqlx::query_as!(
            Location,
            r#"SELECT * FROM location
                WHERE deleted_at IS NULL"#
        )
        .fetch_all(pool)
        .await?;

        Ok(locations)
    }

    pub async fn add(pool: &PgPool, name: String) -> Result<Location> {
        let location = sqlx::query_as!(
            Location,
            "INSERT INTO location (name)
                  VALUES ($1)
            RETURNING *",
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(location)
    }

    pub async fn delete_by_id(pool: &PgPool, id: i64) -> Result<Location> {
        let mut tx = pool.begin().await?;

        let location = sqlx::query_as!(
            Location,
            r#"UPDATE location
                  SET deleted_at = now()
                WHERE deleted_at IS NULL
                  AND id = $1
            RETURNING *"#,
            id
        )
        .fetch_one(&mut *tx)
        .await?;

        let _result = sqlx::query!(
            r#"UPDATE game
                  SET deleted_at = now()
                WHERE deleted_at IS NULL
                  AND location_id = $1"#,
            id
        )
        .execute(&mut *tx)
        .await?;

        let _result = sqlx::query!(
            r#"UPDATE note
                  SET deleted_at = now()
                 FROM game
                WHERE game.id=note.game_id
                  AND note.deleted_at IS NULL
                  AND game.deleted_at IS NULL
                  AND game.location_id = $1"#,
            id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(location)
    }
}

#[derive(Debug, Serialize)]
pub struct GameWithNotes {
    pub id: i64,
    location_id: i64,
    pub name: String,
    pub abbreviation: String,
    pub disabled_at: Option<NaiveDateTime>,
    pub reserved_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
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
            notes,
            reserved_minutes: game.reserved_minutes,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Game {
    pub id: i64,
    pub location_id: i64,
    pub name: String,
    pub abbreviation: String,
    pub disabled_at: Option<NaiveDateTime>,
    pub reserved_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    pub reserved_minutes: i32,
}

impl Game {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<GameWithNotes> {
        let game = sqlx::query_as!(
            Game,
            r#"SELECT *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                WHERE deleted_at IS NULL
                  AND id = $1"#,
            id
        )
        .fetch_one(pool)
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
        .fetch_all(pool)
        .await?;

        Ok(GameWithNotes::build(game, notes))
    }

    pub async fn find_by_location_id(pool: &PgPool, id: i64) -> Result<Vec<GameWithNotes>> {
        let games = sqlx::query_as!(
            Game,
            r#"SELECT game.*, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                 JOIN location
                   ON location.id = game.location_id
                WHERE game.deleted_at IS NULL
                  AND location.deleted_at IS NULL
                  AND location_id = $1
             ORDER BY abbreviation, id ASC"#,
            id
        )
        .fetch_all(pool)
        .await?;

        if games.is_empty() {
            sqlx::query!(
                r#"SELECT id FROM location WHERE id = $1 AND deleted_at IS NULL"#,
                id
            )
            .fetch_one(pool)
            .await?;
            return Ok(Vec::new());
        }

        let game_ids: Vec<i64> = games.iter().map(|g| g.id).collect();
        let notes = sqlx::query_as!(
            Note,
            r#"SELECT *
                 FROM note
                WHERE deleted_at IS NULL
                  AND game_id = ANY($1)
             ORDER BY created_at ASC"#,
            &game_ids
        )
        .fetch_all(pool)
        .await?;

        let mut notes_map: HashMap<i64, Vec<Note>> = HashMap::new();
        for note in notes {
            notes_map.entry(note.game_id).or_default().push(note);
        }

        let games_with_notes = games
            .into_iter()
            .map(|g| {
                let notes = notes_map.remove(&g.id).unwrap_or_default();
                GameWithNotes::build(g, notes)
            })
            .collect();

        Ok(games_with_notes)
    }

    pub async fn add(
        pool: &PgPool,
        location_id: i64,
        name: String,
        abbreviation: String,
    ) -> Result<Game> {
        let game = sqlx::query_as!(
                Game,
                r#"INSERT INTO game (location_id, name, abbreviation)
                        SELECT id, $2, $3 FROM location
                         WHERE id = $1
                           AND deleted_at IS NULL
                RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
                location_id,
                name,
                abbreviation
            )
            .fetch_one(pool)
            .await?;

        Ok(game)
    }

    pub async fn update_by_id(
        pool: &PgPool,
        id: i64,
        name: String,
        abbreviation: String,
    ) -> Result<Game> {
        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET name = $1,
                      abbreviation = $2
                WHERE id = $3
                  AND deleted_at IS NULL
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            name,
            abbreviation,
            id,
        )
        .fetch_one(pool)
        .await?;

        Ok(game)
    }

    pub async fn disable_by_id(pool: &PgPool, id: i64) -> std::result::Result<Game, DbError> {
        let mut tx = pool.begin().await?;

        let game = sqlx::query_as!(
            Game,
            r#"SELECT *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                WHERE id = $1
                  AND deleted_at IS NULL
             FOR UPDATE"#,
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        if game.disabled_at.is_some() {
            tx.commit().await?;
            return Ok(game);
        }

        if let Some(reserved_at) = game.reserved_at {
            sqlx::query!(
                r#"INSERT INTO reservation (game_id, reserved_at, released_at)
                        VALUES ($1, $2, now())"#,
                id,
                reserved_at,
            )
            .execute(&mut *tx)
            .await?;
        }

        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET disabled_at = now(),
                      reserved_at = NULL
                WHERE id = $1
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn enable_by_id(pool: &PgPool, id: i64) -> std::result::Result<Game, DbError> {
        let mut tx = pool.begin().await?;

        let game = sqlx::query_as!(
            Game,
            r#"SELECT *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                WHERE id = $1
                  AND deleted_at IS NULL
             FOR UPDATE"#,
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        if game.disabled_at.is_none() {
            tx.commit().await?;
            return Ok(game);
        }

        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET disabled_at = NULL
                WHERE id = $1
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn delete_by_id(pool: &PgPool, id: i64) -> Result<Game> {
        let mut tx = pool.begin().await?;

        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET deleted_at = now()
                WHERE id = $1
                  AND deleted_at IS NULL
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        let _result = sqlx::query!(
            r#"UPDATE note
                  SET deleted_at = now()
                WHERE game_id = $1
                  AND deleted_at IS NULL"#,
            id,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn reserve_by_id(pool: &PgPool, id: i64) -> std::result::Result<Game, DbError> {
        let mut tx = pool.begin().await?;

        let game = sqlx::query_as!(
            Game,
            r#"SELECT *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                WHERE id = $1
                  AND deleted_at IS NULL
             FOR UPDATE"#,
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        if game.disabled_at.is_some() {
            return Err(DbError::Disabled);
        }
        if game.reserved_at.is_some() {
            tx.commit().await?;
            return Ok(game);
        }

        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET reserved_at = now()
                WHERE id = $1
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn reserve_random_by_location_id(pool: &PgPool, location_id: i64) -> Result<Game> {
        let mut tx = pool.begin().await?;

        let game = sqlx::query_as!(
            Game,
            r#"SELECT *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                WHERE deleted_at IS NULL
                  AND disabled_at IS NULL
                  AND reserved_at IS NULL
                  AND location_id = $1
                ORDER BY random() FOR UPDATE
                LIMIT 1"#,
                location_id
        )
        .fetch_one(&mut *tx)
        .await?;

        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET reserved_at = now()
                WHERE id = $1
                  AND location_id = $2
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            game.id,
            location_id,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn release_reservation_by_id(
        pool: &PgPool,
        id: i64,
    ) -> std::result::Result<Game, DbError> {
        let mut tx = pool.begin().await?;

        let game = sqlx::query_as!(
            Game,
            r#"SELECT *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                WHERE id = $1
                  AND deleted_at IS NULL
             FOR UPDATE"#,
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        let Some(reserved_at) = game.reserved_at else {
            tx.commit().await?;
            return Ok(game);
        };

        sqlx::query!(
            r#"INSERT INTO reservation (game_id, reserved_at, released_at)
                    VALUES ($1, $2, now())"#,
            id,
            reserved_at,
        )
        .execute(&mut *tx)
        .await?;

        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET reserved_at = NULL
                WHERE id = $1
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(game)
    }

    pub async fn find_reserved_longer_than(pool: &PgPool, minutes: i32) -> Result<Vec<Game>> {
        let games = sqlx::query_as!(
            Game,
            r#"SELECT *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                WHERE deleted_at IS NULL
                  AND reserved_at IS NOT NULL
                  AND EXTRACT(EPOCH FROM (now() - reserved_at)) / 60 > $1::int
                ORDER BY reserved_at ASC"#,
            minutes
        )
        .fetch_all(pool)
        .await?;

        Ok(games)
    }
}

#[derive(Debug, Serialize)]
pub struct ReservationStats {
    pub game_id: i64,
    pub reservation_count: i64,
    pub reserved_minutes: i64,
    pub average_reserved_minutes: f64,
    pub median_reserved_minutes: f64,
}

impl ReservationStats {
    pub async fn get_reservations_stats_by_game_id(
        pool: &PgPool,
        game_id: i64,
    ) -> Result<ReservationStats> {
        sqlx::query!(
            r#"SELECT id FROM game WHERE id = $1 AND deleted_at IS NULL"#,
            game_id
        )
        .fetch_one(pool)
        .await?;

        let stats = sqlx::query_as!(
            ReservationStats,
            r#"
            WITH reservation_durations AS (
                SELECT
                    game_id,
                    EXTRACT(EPOCH FROM (released_at - reserved_at)) / 60 as duration_minutes
                 FROM reservation
                WHERE game_id = $1
                  AND released_at IS NOT NULL
            )
            SELECT
                $1 as "game_id!",
                COUNT(*) as "reservation_count!",
                COALESCE(SUM(duration_minutes)::bigint, 0) as "reserved_minutes!",
                CAST(COALESCE(AVG(duration_minutes), 0) as float8) as "average_reserved_minutes!",
                CAST(COALESCE(PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY duration_minutes), 0) as float8) as "median_reserved_minutes!"
            FROM reservation_durations"#,
            game_id
        )
        .fetch_one(pool)
        .await?;

        Ok(stats)
    }
}

#[derive(Debug, Serialize)]
pub struct Note {
    pub id: i64,
    game_id: i64,
    // Nullable for now, fix once we add players to the mix.
    player_id: Option<i64>,
    pub note: String,
    deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl Note {
    pub async fn add_by_game_id(pool: &PgPool, note: String, game_id: i64) -> Result<Note> {
        let note = sqlx::query_as!(
            Note,
            r#"INSERT INTO note (note, game_id)
                    SELECT $1, id FROM game
                     WHERE id = $2
                       AND deleted_at IS NULL
                 RETURNING *"#,
            note,
            game_id,
        )
        .fetch_one(pool)
        .await?;

        Ok(note)
    }

    pub async fn delete_by_id(pool: &PgPool, game_id: i64, id: i64) -> Result<Note> {
        let note = sqlx::query_as!(
            Note,
            r#"UPDATE note
                  SET deleted_at = now()
                WHERE id = $2
                  AND game_id = $1
                  AND deleted_at IS NULL
                  AND EXISTS (SELECT 1 FROM game WHERE game.id = $1 AND game.deleted_at IS NULL)
            RETURNING *"#,
            game_id,
            id,
        )
        .fetch_one(pool)
        .await?;

        Ok(note)
    }
}
