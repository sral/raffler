Backend:
- We err when for example a game is reserved twice (IS NULL WHERE-clauses prevent mutation), maybe just return game state and 200?
- Refactor db to re-use common queries etc... remove duplication... maybe... :reverse_shaking_fist:
    - For a blatant example see disable/enable
