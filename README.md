### Raffler

Nothing to see here yet. Move along.

#### Building/running
Might require:
`$ export DATABASE_URL="sqlite:raffler.sqlite"`

To recreate db with sqlx-cli installed:
```
$ sqlx database drop
$ sqlx database create
$ sqlx migrate run
```

To build:

`$ cargo build`

To build and run:

`$ cargo run`

#### Testing
Useful commands for testing service/API locally.

##### Add location
`$ curl -X POST http://localhost:8000/v1/locations -H 'Content-Type: application/json' -d '{"name":"Spola Tilted"}'`

##### List locations
`$ curl http://localhost:8000/v1/locations`

##### Delete location
`$ curl -X DELETE http://localhost:8000/v1/locations/1`

##### List games at location
`$ curl http://localhost:8000/v1/locations/1/games`

##### Add game at location
`$ curl -X POST http://localhost:8000/v1/locations/1/games -H 'Content-Type: application/json' -d '{"name":"Attack From Mars", "abbreviation": "AFM"}'`

##### List information for game at location
`$ curl http://localhost:8000/v1/locations/1/games/1`

##### Update information for game at location
`$ curl -X PUT http://localhost:8000/v1/locations/1/games/1 -H 'Content-Type: application/json' -d '{"name":"Attack From Lars", "abbreviation": "AFL"}'`

##### Delete game at location
`$ curl -X DELETE http://localhost:8000/v1/locations/1/games/1`

#### Disabled game at location
`curl http://localhost:8000/v1/locations/1/games/1/disable`

#### Disabled game at location
`curl http://localhost:8000/v1/locations/1/games/1/enable`

##### Add note to game at location
`curl -X POST http://localhost:8000/v1/locations/1/games/1/notes -H 'Content-Type: application/json' -d '{"note":"Autoplunger is infested with cows"}'`

##### Delete note for game at location
`curl -X DELETE http://localhost:8000/v1/locations/1/games/1/notes/1`
