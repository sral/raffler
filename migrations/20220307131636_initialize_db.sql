CREATE TABLE IF NOT EXISTS location (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  -- This really should be timestamp(s) but sqlite+sqlx currently says NO! :reverse_shaking_fist:
  deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS game (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  location_id INTEGER REFERENCES location(id) NOT NULL,
  name TEXT NOT NULL,
  abbreviation TEXT NOT NULL,
  -- This really should be timestamp(s) but sqlite+sqlx currently says NO! :reverse_shaking_fist:
  disabled BOOLEAN NOT NULL DEFAULT FALSE,
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS note (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  game_id INTEGER REFERENCES game(id),
  player_id INTEGER REFERENCES player(id),
  note TEXT NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS reservation (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  game_id INTEGER REFERENCES game(id),
  reserved_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  released_at TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE IF NOT EXISTS player (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    initials TEXT NOT NULL CHECK (length(name) <= 3),
    name TEXT NOT NULL,
    surname TEXT NOT NULL,
    -- This really should be timestamp(s) but sqlite+sqlx currently says NO! :reverse_shaking_fist:
    disabled BOOLEAN NOT NULL DEFAULT FALSE,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);
