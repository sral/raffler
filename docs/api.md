### Locations

Actions related to locations.

#### List all locations

Example request:
```GET v1/locations HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

[
    {
        "id": 1,
        "name: "The Cow Pasture",
    },
    {
        "id": 2,
        "name: "Choclate Nudge Brownie",
    },
]
```

#### List specific location

Example request:
```GET v1/locations/{location_id} HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name: "The Cow Pasture",
}
```

#### Add new location

Example request:
```
POST v1/locations HTTP/1.1
Content-Type: application/json

{
    "name": "The Church of Slap Saves"
}
```
Example response
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 3,
    "name: "The Church of Slap Saves",
}
```
#### Remove location
Example request:
```DELETE v1/locations/{location_id} HTTP/1.1```

Example response:
```HTTP/1.1 200 OK```

### Games

Actions related to games.
#### List all games at location

Example request:
```GET v1/locations/{location_id}/games HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

[
    {
        "id": 1,
        "name: "Attack From Mars",
        "abbreviation": "AFM",
        "reserved": false,
        "reserved_for_minutes": 0,
        "disabled_at": "2022-03-11T01:01:00.00000+00:00",
        "notes": [
            {
                "id": 1,
                "note": "",
            },
            {
                "id": 2,
                "note": "",
            },
        ]
    },
    {
        "id": 2,
        "name: "Iron Maiden (Pro)",
        "abbreviation": "IRMA",
        "reserved": true,
        "reserved_for_minutes": 5,
        "disabled_at": null,
        "notes": [
            {
                "id": 3,
                "note": "",
            },
        ]
    },
]
```

#### List game at location

Example request:
```GET v1/locations/{location_id}/games/{game_id} HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name: "Attack From Mars",
    "abbreviation": "AFM",
    "reserved": false,
    "reserved_for_minutes": 0,
    "disabled_at": null,
    "notes": [
        {
            "id": 1,
            "note": "",
        },
        {
            "id": 2,
            "note": "",
        },
    ]
}
```

#### Add game at location
```
POST v1/locations/{location_id}/games HTTP/1.1
Content-Type: application/json

{
    "name": "Scared Stiff",
    "abbreviation": "SS",
}
```
Example response
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 3,
}
```

#### Update game information

Example request:
```
PUT v1/locations/{location_id}/games/{game_id} HTTP/1.1

{
    "name": "Attack From Lars",
    "abbreviation": "AFL",
}
```

Example response:
```
HTTP/1.1 200 OK
```
#### Remove game from location
Example request:
```
DELETE v1/locations/{location_id}/games/{game_id} HTTP/1.1
```

Example response:
```
HTTP/1.1 200 OK
```

#### Disable game at location
This marks the game as not being included for random selection, in tournaments, etc.

Example request:
```
GET v1/locations/{location_id}/games/{game_id}/disable HTTP/1.1
```

Example response:
```
HTTP/1.1 200 OK
```

#### Enable game at location
This marks the game as no longer being disabled (see disabled docs for what that means).

Example request:
```
GET v1/locations/{location_id}/games/{game_id}/enable HTTP/1.1
```

Example response:
```
HTTP/1.1 200 OK
```


#### List reservations for game at location
If we ever want to construct reservation history in some sort of game overview.
Might not be implemented/required. Could be baked into game object when fetching specific
game as well.

Example request:
```
POST v1/locations/{location_id}/games/{game_id}/reservations HTTP/1.1
```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "game_id": 1,
    "reservations" [
        {
            "reserved_at": "2022-03-11T01:01:00.00000+00:00",
            "released_at": "2022-03-11T01:05:00.00000+00:00",
        },
        {
            "reserved_at": "2022-03-12T01:07:00.00000+00:00",
            "released_at": "2022-03-12T01:15:00.00000+00:00",
        },
    ]
}
```
#### Reserve random game at location

Example request:
```
POST v1/locations/{location_id}/games/reserve HTTP/1.1
```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name: "Attack From Mars",
    "abbreviation": "AFM",
    "reserved": true,
    "reserved_for_minutes": 0,
    "disabled_at": null,
    "notes": [
        {
            "id": 1,
            "note": "",
        },
    ]
}
```
#### Reserve game at location

Example request:
```
POSTT v1/locations/{location_id}/games/{game_id}/reserve HTTP/1.1
```

Example response:
```
HTTP/1.1 200 OK
```
#### Release current game reservation at location

Example request:
```
DELETE v1/locations/{location_id}/games/{game_id}/reservation HTTP/1.1
```

Example response:
```
HTTP/1.1 200 OK
```

### Notes

Actions related to notes. Notes can be used as for example a service log.

#### Create note

#### List notes for game at locations
TODO: Maybe? Nested in game response. Depends on how we want things to work.

#### Add note for game at location
```
POST v1/locations/{location_id}/games/<game_id>/notes HTTP/1.1
Content-Type: application/json

{
    "note": "Auto plunger infested with cows"
}
```
Example response
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 3,
}
```

#### Delete note for game at location
Example request:
```
DELETE v1/locations/{location_id}/games/{game_id}/note/<note_id> HTTP/1.1
```

Example response:
```
HTTP/1.1 200 OK
```

### Player managment & authentication

Authentication details TBD. The intent is to only allowed logged in players (or locations) to
make changes to games, locations, tournaments, etc. Login needs to be optional as in the lottery needs to
be functional without signing in.

Logging in will setup a session which will be used for authorization. Details TBD.

#### Create player
Example request:
```
POST v1/players HTTP/1.1
Content-Type: application/json

{
    "username": "",
    "password": "",
    "initials": "",
    "name": "",
    "surname": "",
}
```
Example response
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "username": "",
    "initials": "",
    "name": "",
    "surname": "",
}
```

#### Delete player

Example request:
```DELETE v1/players/{player_id} HTTP/1.1```

Example response:
```HTTP/1.1 200 OK```

#### Login player
Example request:
```
POST v1/players/login HTTP/1.1
Content-Type: application/json

{
    "username": "",
    "password": "",
}
```
Example response
```
HTTP/1.1 200 OK
Content-Type: application/json
```

### Logout player
Example request:
```
GET v1/players/{player_id}/logout HTTP/1.1
Content-Type: application/json

```
Example response
```
HTTP/1.1 200 OK
Content-Type: application/json
```

### Tournaments
TBD! The intent is to allow creating and running tournaments with automated matchmaking, scorekeeping,
standings etc. Initially naive match play will be implemented.

### Error handling
API errors will be communicated using HTTP status codes.
