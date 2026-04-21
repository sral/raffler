### Raffler

Web application for managing pinball games at locations. Users can track games, reserve games, add notes, and temporarily disable games. Stale reservations are automatically cleaned up after 90 minutes.

#### Prerequisites

- Rust toolchain
- PostgreSQL
- Node.js 20+ and npm

#### Building

Build the frontend (outputs to `dist/`):
```
$ npm install
$ npm run build
```

Build the backend:
```
$ cargo build
```

#### Database setup

Requires a running PostgreSQL instance:
```
$ export DATABASE_URL="postgres://localhost/raffler"
$ sqlx database create
$ sqlx migrate run
```

Migrations also run automatically on application startup.

To build without a database (CI):
```
$ SQLX_OFFLINE=true cargo build
```

#### Running

Build the frontend first, then start the server:
```
$ npm run build
$ cargo run
```

Open http://localhost:8000. The backend serves both the API and the frontend from `dist/`.

#### Development

For development with hot reload, use two terminals:

1. `cargo run` (backend on http://localhost:8000)
2. `npm run dev` (Vite dev server on http://localhost:5173, proxies `/v1` to backend)

Access http://localhost:5173 during development.

#### Testing

Backend tests:
```
$ SQLX_OFFLINE=true cargo test
```

Linting:
```
$ cargo clippy --all-targets --all-features -- -D warnings
$ cargo fmt -- --check
```

#### API

See [API.md](API.md) for the full endpoint reference.

Quick examples for local testing:
```
$ curl http://localhost:8000/v1/locations
$ curl -X POST http://localhost:8000/v1/locations -H 'Content-Type: application/json' -d '{"name":"Spola Tilten"}'
$ curl -X POST http://localhost:8000/v1/locations/1/games -H 'Content-Type: application/json' -d '{"name":"Attack From Mars", "abbreviation": "AFM"}'
$ curl -X POST http://localhost:8000/v1/games/1/reservations
$ curl -X DELETE http://localhost:8000/v1/games/1/reservations
```
