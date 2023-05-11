Frontend (crude beginning still):
- Vendor dependencis (?) and figure out how to build production app
- Add error handling (handle 404s etc) or fix API responses to not err as described below
- Etc etc

Backend:
- Persist reservation history (INSERT INTO reservation ...)
- Notes returned by API should include timestamps
- Improve API error handling, implement and document HTTP statuses.
    - Do we need an API error object to provide context around failures? Not yet maybe but in the future?
- Spawn and pass transaction to db functions to allow composing/multiple queries in same transaction?
- Can we compose and move GameWithNotes up to API layer only?
- Currently we err when for example a game is reserved twice (IS NULL WHERE-clauses prevent mutation), maybe just return game state and 200?
- API needs to be revisted once we build frontend
    - Inconsistent responses (always echo state?)
    - Game related requests don't really need location id (i.e. we could drop the location part from /locations/<id>/games/<id>)
- Refactor db to re-use common queries etc... remove duplication... maybe... :reverse_shaking_fist:
    - For a blatant example see disable/enable
- Learn how to deal with sessions/auth in Rocket
- Ditch Rocket and go for Actix?!
- Don't forget to add locks (SELECT ... FOR UPDATE)?
- There's currently no validation of actions on deleted entities. Ex you can add games at deleted locations
- UNIQUE constraint on location name makes no sense if you are re-adding (delete and recreate) the same location
