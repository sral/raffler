Frontend (crude beginning still):
- Vendor dependencis (?) and figure out how to build production app
- Add error handling (handle 404s etc) or fix API responses to not err as described below
- Visual: Fix the note/comment display to include date/time and add controls for deleting them (if not there)

Backend:
- Persist reservation history (INSERT INTO reservation ...)
- Improve API error handling, implement and document HTTP statuses.
    - Do we need an API error object to provide context around failures? Not yet maybe but in the future?
- Currently we err when for example a game is reserved twice (IS NULL WHERE-clauses prevent mutation), maybe just return game state and 200?
- API needs to be revisted once we build frontend
    - Inconsistent responses (always echo state?)
    - Game related requests don't really need location id (i.e. we could drop the location part from /locations/<id>/games/<id>)
- Refactor db to re-use common queries etc... remove duplication... maybe... :reverse_shaking_fist:
    - For a blatant example see disable/enable
- There's currently no validation of actions on deleted entities. Ex you can add games at deleted locations
- We probably want to spawn some thread that does clean-up as in remove reservations older than N
