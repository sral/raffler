use chrono::prelude::*;

use futures_util::TryStreamExt;
use serde::Serialize;
use sqlx::PgPool;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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
        .fetch(pool)
        .try_collect::<Vec<_>>()
        .await?;

        Ok(locations)
    }

    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Location> {
        let location = sqlx::query_as!(
            Location,
            r#"SELECT *
                 FROM location
                WHERE deleted_at IS NULL
                  AND id = $1"#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(location)
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
    location_id: i64,
    pub name: String,
    pub abbreviation: String,
    pub disabled_at: Option<NaiveDateTime>,
    pub reserved_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    pub reserved_minutes: i32,
}

impl Game {
    pub async fn find_by_id(pool: &PgPool, id: i64, location_id: i64) -> Result<GameWithNotes> {
        let mut tx = pool.begin().await?;
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

        // tx.commit().await?;
        Ok(GameWithNotes::build(game, notes))
    }

    pub async fn find_by_location_id(pool: &PgPool, id: i64) -> Result<Vec<GameWithNotes>> {
        let mut tx = pool.begin().await?;

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
        .fetch(&mut *tx)
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

        //tx.commit().await?;
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
                        VALUES ($1, $2, $3)
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
        location_id: i64,
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
                  AND location_id = $4
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            name,
            abbreviation,
            id,
            location_id,
        )
        .fetch_one(pool)
        .await?;

        Ok(game)
    }

    pub async fn disable_by_id(pool: &PgPool, location_id: i64, id: i64) -> Result<Game> {
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
        .fetch_one(pool)
        .await?;

        Ok(game)
    }

    pub async fn enable_by_id(pool: &PgPool, location_id: i64, id: i64) -> Result<Game> {
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
        .fetch_one(pool)
        .await?;

        Ok(game)
    }

    pub async fn delete_by_id(pool: &PgPool, location_id: i64, id: i64) -> Result<Game> {
        let mut tx = pool.begin().await?;

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

    pub async fn reserve_by_id(pool: &PgPool, location_id: i64, id: i64) -> Result<Game> {
        let game = sqlx::query_as!(
            Game,
            r#"UPDATE game
                  SET reserved_at = now()
                WHERE id = $1
                  AND location_id = $2
                  AND reserved_at IS NULL
            RETURNING *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!""#,
            id,
            location_id,
        )
        .fetch_one(pool)
        .await?;

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
        location_id: i64,
        id: i64,
    ) -> Result<Game> {
        let mut tx = pool.begin().await?;

        // First get the current reserved_at value
        let game = sqlx::query_as!(
            Game,
            r#"SELECT *, COALESCE((EXTRACT(EPOCH FROM (now() - reserved_at)) / 60)::int, 0) as "reserved_minutes!"
                 FROM game
                WHERE id = $1
                  AND location_id = $2
                  AND reserved_at IS NOT NULL"#,
            id,
            location_id,
        )
        .fetch_one(&mut *tx)
        .await?;

        // Insert the reservation record
        let _result = sqlx::query!(
            r#"INSERT INTO reservation (game_id, reserved_at, released_at)
                VALUES ($1, $2, now())"#,
            id,
            game.reserved_at,
        )
        .execute(&mut *tx)
        .await?;

        // Update the game's reserved_at to NULL
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
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(game)
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
                    VALUES ($1, $2)
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
            RETURNING *"#,
            game_id,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(note)
    }
}
