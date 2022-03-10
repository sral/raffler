CREATE TABLE IF NOT EXISTS location (
  id BIGSERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT TIMEZONE('UTC', NOW()),
  deleted_at TIMESTAMP
);


CREATE TABLE IF NOT EXISTS game (
  id BIGSERIAL PRIMARY KEY,
  location_id BIGINT NOT NULL REFERENCES location(id),
  name TEXT NOT NULL,
  abbreviation TEXT NOT NULL,
  deleted_at TIMESTAMP,
  disabled_at TIMESTAMP,
  reserved_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT TIMEZONE('UTC', NOW())
);

CREATE TABLE IF NOT EXISTS player (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    initials TEXT NOT NULL CHECK (length(name) <= 3),
    name TEXT NOT NULL,
    surname TEXT NOT NULL,
    disabled_at TIMESTAMP,
    deleted_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT TIMEZONE('UTC', NOW())
);

CREATE TABLE IF NOT EXISTS note (
  id BIGSERIAL PRIMARY KEY,
  game_id BIGINT NOT NULL REFERENCES game(id),
  -- Allow NULL for now, add constraint once players are implemented.
  player_id BIGINT REFERENCES player(id),
  note TEXT NOT NULL,
  deleted_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT TIMEZONE('UTC', NOW())
);

CREATE TABLE IF NOT EXISTS reservation (
  id BIGSERIAL PRIMARY KEY,
  game_id BIGINT REFERENCES game(id),
  -- tsrange
  reserved_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  released_at TIMESTAMP WITHOUT TIME ZONE
);
