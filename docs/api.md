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
        "name": "The Cow Pasture"
    },
    {
        "id": 2,
        "name": "Choclate Nudge Brownie"
    }
]
```

#### List specific location

Example request:
```GET v1/locations/{id} HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name": "The Cow Pasture"
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

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 3,
    "name": "The Church of Slap Saves"
}
```

#### Remove location

Example request:
```DELETE v1/locations/{id} HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name": "The Cow Pasture"
}
```

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
        "name": "Attack From Mars",
        "abbreviation": "AFM",
        "disabled_at": "2023-05-11T12:22:16.522042",
        "reserved_at": null,
        "reserved_minutes": 0,
        "notes": [
            {
                "id": 1,
                "note": "Autoplunger is infested with cows",
                "created_at": "2023-05-04T07:42:51.260146"
            }
        ]
    }
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
    "name": "Attack From Mars",
    "abbreviation": "AFM",
    "disabled_at": "2023-05-11T12:22:16.522042",
    "reserved_at": null,
    "reserved_minutes": 0,
    "notes": [
        {
            "id": 1,
            "note": "Autoplunger is infested with cows",
            "created_at": "2023-05-04T07:42:51.260146"
        }
    ]
}
```

#### Add game at location

Example request:
```
POST v1/locations/{location_id}/games HTTP/1.1
Content-Type: application/json

{
    "name": "Scared Stiff",
    "abbreviation": "SS"
}
```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 3,
    "name": "Scared Stiff",
    "abbreviation": "SS"
}
```

#### Update game information

Example request:
```
PUT v1/locations/{location_id}/games/{game_id} HTTP/1.1
Content-Type: application/json

{
    "name": "Attack From Lars",
    "abbreviation": "AFL"
}
```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name": "Attack From Lars",
    "abbreviation": "AFL"
}
```

#### Remove game from location

Example request:
```DELETE v1/locations/{location_id}/games/{game_id} HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name": "Attack From Mars",
    "abbreviation": "AFM"
}
```

#### Disable game at location

Example request:
```POST v1/locations/{location_id}/games/{game_id}/disable HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name": "Attack From Mars",
    "abbreviation": "AFM"
}
```

#### Enable game at location

Example request:
```POST v1/locations/{location_id}/games/{game_id}/enable HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name": "Attack From Mars",
    "abbreviation": "AFM"
}
```

#### Get game reservation stats

Example request:
```GET v1/locations/{location_id}/games/{game_id}/reservations HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "game_id": 1,
    "reservation_count": 0,
    "reserved_minutes": 0,
    "average_reserved_minutes": 0.0,
    "median_reserved_minutes": 0.0
}
```

#### Reserve random game at location

Example request:
```POST v1/locations/{location_id}/games/reservations HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name": "Attack From Mars",
    "abbreviation": "AFM"
}
```

#### Reserve game at location

Example request:
```POST v1/locations/{location_id}/games/{game_id}/reservations HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name": "Attack From Mars",
    "abbreviation": "AFM"
}
```

#### Release current game reservation at location

Example request:
```DELETE v1/locations/{location_id}/games/{game_id}/reservations HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name": "Attack From Mars",
    "abbreviation": "AFM"
}
```

### Notes

Actions related to notes. Notes can be used as for example a service log.

#### Add note for game at location

Example request:
```
POST v1/locations/{location_id}/games/{game_id}/notes HTTP/1.1
Content-Type: application/json

{
    "note": "Auto plunger infested with cows"
}
```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 3,
    "note": "Auto plunger infested with cows",
    "created_at": "2022-03-11T01:01:00"
}
```

#### Delete note for game at location

Example request:
```DELETE v1/locations/{location_id}/games/{game_id}/notes/{note_id} HTTP/1.1```

Example response:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "note": "Auto plunger infested with cows",
    "created_at": "2022-03-11T01:01:00"
}
```

### Error handling

API errors will be communicated using HTTP status codes:
- 400 Bad Request: Invalid request parameters or body
- 404 Not Found: Resource not found
