# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Raffler is a web application for managing pinball games at locations. Users can track games at different locations, reserve games, add notes, and temporarily disable games. Stale reservations are automatically cleaned up after 90 minutes.

## Tech Stack

- **Backend**: Rust with Axum web framework, Tokio async runtime
- **Database**: PostgreSQL with SQLx (compile-time verified queries)
- **Frontend**: React 19 + React Bootstrap, built with Vite
- **CI**: GitHub Actions (`.github/workflows/rust.yml`)

## Common Commands

### Backend
```bash
cargo build                  # Build
cargo run                    # Build and run (serves on http://localhost:8000)
cargo test                   # Run tests
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo fmt -- --check         # Check formatting
cargo fmt                    # Fix formatting
```

### Frontend
```bash
npm install                  # Install dependencies (required first time)
npm run dev                  # Start Vite dev server with HMR (http://localhost:5173)
npm run build                # Production build to dist/
```

### Database
```bash
export DATABASE_URL="postgres://localhost/raffler"
sqlx database create         # Create database
sqlx migrate run             # Run migrations (also runs automatically on app startup)
```

### CI builds (no database required)
```bash
SQLX_OFFLINE=true cargo build    # Uses cached queries from .sqlx/ directory
SQLX_OFFLINE=true cargo test
```

## Development Workflow

Two-terminal setup for development with hot reload:
1. **Terminal 1**: `cargo run` (backend on :8000)
2. **Terminal 2**: `npm run dev` (Vite dev server on :5173, proxies `/v1` to backend)

Access http://localhost:5173 during development. The Vite proxy config is in `vite.config.js`.

For production, the Rust server serves static files from `dist/` (or directory set by `STATIC_DIR` env var).

## Architecture

### Backend (3 files)

- **`src/main.rs`**: Axum router setup, database pool, background cleanup task (releases reservations >90min every 15min)
- **`src/api.rs`**: HTTP handlers — request/response types and endpoint implementations. Thin layer that delegates to `db.rs`
- **`src/db.rs`**: Database models and SQLx queries. All queries use `query_as!` for compile-time checking

### Frontend (`static/` directory)

- **`raffler.jsx`**: Main React app component
- **`api.js`**: API client for all backend calls
- **`components/`**: React components organized by feature (game/, location/, modals/, shared/)
- **`hooks/`**: Custom hooks — `useGames`, `useLocations`, `useGameOperations`, `useModal`
- **`utils/formatting.js`**: Display formatting helpers

### Data Model

Hierarchical: **Location** → **Game** → **Note** (+ **Reservation** history per game)

All entities use soft deletes (`deleted_at` timestamps). Deleting a location cascades soft-deletes to its games and notes within a transaction.

Games track current reservation state via `reserved_at` on the game table. When released, a record is written to the `reservation` table for statistics. The `reserved_minutes` field is computed dynamically in SQL as `EXTRACT(EPOCH FROM (now() - reserved_at)) / 60`.

The `player` table exists in the schema but is not used in the application.

### API Routes

Location-scoped collection routes under `/v1/locations/{id}`:
- `GET/POST /v1/locations/{id}/games` — list/create games at a location
- `POST /v1/locations/{id}/games/reservations` — reserve a random unreserved game

Per-game routes are flat under `/v1/games/{game_id}` (location is not part of the path):
- `GET/PUT/DELETE /v1/games/{game_id}` — fetch/update/delete
- `POST /v1/games/{game_id}/reservations` — reserve a specific game
- `DELETE /v1/games/{game_id}/reservations` — release a reservation
- `GET /v1/games/{game_id}/reservations` — reservation stats
- `POST /v1/games/{game_id}/disable` / `enable` — toggle availability
- `POST /v1/games/{game_id}/notes`, `DELETE /v1/games/{game_id}/notes/{note_id}` — notes

### SQLx Offline Mode

The `.sqlx/` directory contains cached query metadata enabling builds without a live database (`SQLX_OFFLINE=true`). When modifying SQL queries, you need a running database to regenerate these with `cargo sqlx prepare`.

## Known Issues

- No validation of actions on deleted entities (e.g. games can be added at soft-deleted locations)
- Successful `POST` creates return `200` rather than `201 Created`
- Significant duplication in `db.rs` (near-identical `disable_by_id`/`enable_by_id`, the `reserved_minutes` SQL expression copy-pasted across queries)
