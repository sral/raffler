Backend:
- We err when for example a game is reserved twice (IS NULL WHERE-clauses prevent mutation), maybe just return game state and 200?
- API needs to be revisited
    - Use HTTP status codes to signal outcomes (ex: Should we return things like 201 Created)
- Refactor db to re-use common queries etc... remove duplication... maybe... :reverse_shaking_fist:
    - For a blatant example see disable/enable
- There's currently no validation of actions on deleted entities. Ex you can add games at deleted locations
