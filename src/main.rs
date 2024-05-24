mod api;
mod db;

use axum::{routing::delete, routing::get, routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::time::Duration;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "raffler=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/raffler".to_string());

    // Set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Can't connect to database");

    // Run migrations.
    match sqlx::migrate!().run(&pool).await {
        Ok(_) => tracing::debug!("Migrations applied"),
        Err(e) => tracing::debug!("Error {e}"),
    }

    let app = Router::new()
        .route(
            "/v1/locations",
            get(api::get_all_locations).post(api::post_add_location),
        )
        .route(
            "/v1/locations/:id",
            get(api::get_location_by_id).delete(api::delete_location_by_id),
        )
        .route(
            "/v1/locations/:location_id/games",
            get(api::get_games_by_location_id).post(api::post_add_game_at_location),
        )
        .route(
            "/v1/locations/:location_id/games/:game_id",
            get(api::get_game_at_location_by_id)
                .put(api::put_update_game_at_location)
                .delete(api::delete_game_at_location_by_id),
        )
        .route(
            "/v1/locations/:location_id/games/:game_id/disable",
            post(api::post_disable_game_at_location_by_id),
        )
        .route(
            "/v1/locations/:location_id/games/reservations",
            post(api::post_reserve_random_game_at_location),
        )
        .route(
            "/v1/locations/:location_id/games/:game_id/reservations",
            post(api::post_reserve_game_at_location_by_id)
                .delete(api::delete_game_reservation_at_location_by_id),
        )
        .route(
            "/v1/locations/:location_id/games/:game_id/enable",
            post(api::post_enable_game_at_location_by_id),
        )
        .route(
            "/v1/locations/:location_id/games/:game_id/notes",
            post(api::post_add_note_for_game_at_location),
        )
        .route(
            "/v1/locations/:location_id/games/:game_id/notes/:note_id",
            delete(api::delete_note_for_game_by_id),
        )
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
