Frontend (crude beginning still):
- Vendor dependencies (?) and figure out how to build production app
- Add error handling (handle 404s etc) or fix API responses to not err as described below
- Visual: Fix the note/comment display to include date/time and add controls for deleting them (if not there)
- Make it possible to refresh the page and only reload game state?

Backend:
- We err when for example a game is reserved twice (IS NULL WHERE-clauses prevent mutation), maybe just return game state and 200?
- API needs to be revisited once we build frontend
    - Use HTTP status codes to signal outcomes (ex: Should we return things like 201 Created)
    - Game related requests don't really need location id (i.e. we could drop the location part from /locations/<id>/games/<id>)
    - Do we want to provide context around failures? As in some form of error object in responses?
- Refactor db to re-use common queries etc... remove duplication... maybe... :reverse_shaking_fist:
    - For a blatant example see disable/enable
- There's currently no validation of actions on deleted entities. Ex you can add games at deleted locations
- We probably want to spawn some thread that does clean-up as in remove reservations older than N
