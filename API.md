# Raffler API

All endpoints are rooted at `/v1`. Requests and responses are JSON (`Content-Type: application/json`).

## Conventions

**Errors** — any non-2xx response has the shape `{"error": "message"}`.

| Status | Meaning |
|---|---|
| `200 OK` | Success (including idempotent no-op mutations that return current state) |
| `201 Created` | Resource created (`POST` on a collection) |
| `404 Not Found` | Resource does not exist or has been soft-deleted |
| `409 Conflict` | Request conflicts with current state (e.g. reserving a disabled game, reserving random when none available) |
| `500 Internal Server Error` | Unexpected server error |

**Idempotency** — `reserve`, `release`, `enable`, and `disable` return `200` with the current game state when the game is already in the target state. Repeating one of these calls is a safe no-op. Disabling a reserved game auto-releases the reservation (and writes the `reservation` row used for stats).

**Soft deletes** — `DELETE` on locations and games sets a `deleted_at` timestamp; the row becomes invisible to subsequent reads and mutations (`404`). Deleting a location cascades soft-deletes to its games and their notes in a single transaction.

---

## Locations

### `GET /v1/locations`

List all locations.

**Response** `200` — array of `{id, name}`:
```json
[
  {"id": 1, "name": "Spola Tilten"},
  {"id": 2, "name": "Special When Shit"}
]
```

### `POST /v1/locations`

Create a location.

**Request** — `{"name": string}`

**Response** `201` — `{id, name}`

### `GET /v1/locations/{location_id}`

Fetch a single location.

**Response** `200` — `{id, name}`
**Errors** — `404 Location not found`

### `DELETE /v1/locations/{location_id}`

Soft-delete the location. Cascade soft-deletes its games and their notes.

**Response** `200` — the deleted location `{id, name}`
**Errors** — `404 Location not found`

---

## Games

### `GET /v1/locations/{location_id}/games`

List the games at a location (with their notes).

**Response** `200` — array of:
```json
{
  "id": 1,
  "name": "Attack From Mars",
  "abbreviation": "AFM",
  "disabled_at": null,
  "reserved_at": "2026-04-21T07:12:06.851264",
  "reserved_minutes": 3,
  "notes": [
    {"id": 12, "note": "Autoplunger is infested with cows", "created_at": "2026-04-20T14:00:00"}
  ]
}
```
`reserved_minutes` is computed from `reserved_at`; `0` when not reserved.
**Errors** — `404 Location not found`

### `POST /v1/locations/{location_id}/games`

Create a game at a location.

**Request** — `{"name": string, "abbreviation": string}`
**Response** `201` — `{id, name, abbreviation}`
**Errors** — `404 Location not found` (location does not exist or is soft-deleted)

### `GET /v1/games/{game_id}`

Fetch a single game with its notes.

**Response** `200` — same shape as the list entry above.
**Errors** — `404 Game not found`

### `PUT /v1/games/{game_id}`

Update a game's name and abbreviation.

**Request** — `{"name": string, "abbreviation": string}`
**Response** `200` — `{id, name, abbreviation}`
**Errors** — `404 Game not found`

### `DELETE /v1/games/{game_id}`

Soft-delete a game. Cascade soft-deletes its notes.

**Response** `200` — `{id, name, abbreviation}`
**Errors** — `404 Game not found`

### `POST /v1/games/{game_id}/disable`

Disable a game. Idempotent. If the game is currently reserved, the reservation is released in the same transaction (a `reservation` row is written for stats).

**Response** `200` — `{id, name, abbreviation}`
**Errors** — `404 Game not found`

### `POST /v1/games/{game_id}/enable`

Enable a previously disabled game. Idempotent.

**Response** `200` — `{id, name, abbreviation}`
**Errors** — `404 Game not found`

---

## Reservations

A game has at most one active reservation, tracked by `reserved_at` on the game row. When a reservation is released (or auto-released on disable), a row is written to the `reservation` table for stats.

Reservations older than 90 minutes are automatically released by a background task that runs every 15 minutes.

### `POST /v1/games/{game_id}/reservations`

Reserve a specific game. Idempotent — if the game is already reserved, returns the current game state.

**Response** `200` — `{id, name, abbreviation}`
**Errors**
- `404 Game not found`
- `409 Game is disabled`

### `DELETE /v1/games/{game_id}/reservations`

Release a reservation. Idempotent — if the game is not reserved, returns the current game state without writing a duplicate `reservation` row.

**Response** `200` — `{id, name, abbreviation}`
**Errors** — `404 Game not found`

### `POST /v1/locations/{location_id}/games/reservations`

Reserve a random unreserved, enabled game at the location.

**Response** `200` — `{id, name, abbreviation}` of the chosen game
**Errors** — `409 No available games at this location`

### `GET /v1/games/{game_id}/reservations`

Return aggregate stats over completed reservations (released only).

**Response** `200`:
```json
{
  "game_id": 1,
  "reservation_count": 42,
  "reserved_minutes": 1337,
  "average_reserved_minutes": 31.83,
  "median_reserved_minutes": 25.5
}
```
**Errors** — `404 Game not found`

---

## Notes

### `POST /v1/games/{game_id}/notes`

Add a note to a game.

**Request** — `{"note": string}`
**Response** `201` — `{id, note, created_at}`
**Errors** — `404 Game not found`

### `DELETE /v1/games/{game_id}/notes/{note_id}`

Soft-delete a note.

**Response** `200` — `{id, note, created_at}`
**Errors** — `404 Note not found`
