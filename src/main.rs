mod api;
mod db;

use axum::{routing::delete, routing::get, routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::time::Duration;

const RESERVATION_TIMEOUT_MINUTES: i32 = 90;
const CLEANUP_INTERVAL_SECONDS: u64 = 15 * 60;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "raffler=debug,tower_http=debug,axum=debug".into()),
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

    // Spawn background task for game reservation cleanup
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(CLEANUP_INTERVAL_SECONDS)).await;

            // Find and release games that have been reserved for more than 90 minutes
            match db::Game::find_reserved_longer_than(&pool_clone, RESERVATION_TIMEOUT_MINUTES)
                .await
            {
                Ok(games) => {
                    for game in games {
                        match db::Game::release_reservation_by_id(
                            &pool_clone,
                            game.location_id,
                            game.id,
                        )
                        .await
                        {
                            Ok(_) => tracing::info!(
                                "Released game {} at location {}",
                                game.id,
                                game.location_id
                            ),
                            Err(e) => tracing::error!(
                                "Failed to release game {} at location {}: {}",
                                game.id,
                                game.location_id,
                                e
                            ),
                        }
                    }
                }
                Err(e) => tracing::error!("Failed to find games to release: {}", e),
            }
        }
    });

    // Run migrations.
    match sqlx::migrate!().run(&pool).await {
        Ok(_) => tracing::info!("Migrations applied"),
        Err(e) => tracing::debug!("Error {e}"),
    }

    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("static/index.html"));

    let app = Router::new()
        .route(
            "/v1/locations",
            get(api::get_all_locations).post(api::post_add_location),
        )
        .route(
            "/v1/locations/{id}",
            get(api::get_location_by_id).delete(api::delete_location_by_id),
        )
        .route(
            "/v1/locations/{location_id}/games",
            get(api::get_games_by_location_id).post(api::post_add_game_at_location),
        )
        .route(
            "/v1/locations/{location_id}/games/{game_id}",
            get(api::get_game_at_location_by_id)
                .put(api::put_update_game_at_location)
                .delete(api::delete_game_at_location_by_id),
        )
        .route(
            "/v1/locations/{location_id}/games/{game_id}/disable",
            post(api::post_disable_game_at_location_by_id),
        )
        .route(
            "/v1/locations/{location_id}/games/reservations",
            post(api::post_reserve_random_game_at_location),
        )
        .route(
            "/v1/locations/{location_id}/games/{game_id}/reservations",
            get(api::get_game_reservation_stats)
                .post(api::post_reserve_game_at_location_by_id)
                .delete(api::delete_game_reservation_at_location_by_id),
        )
        .route(
            "/v1/locations/{location_id}/games/{game_id}/enable",
            post(api::post_enable_game_at_location_by_id),
        )
        .route(
            "/v1/locations/{location_id}/games/{game_id}/notes",
            post(api::post_add_note_for_game_at_location),
        )
        .route(
            "/v1/locations/{location_id}/games/{game_id}/notes/{note_id}",
            delete(api::delete_note_for_game_by_id),
        )
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
