cargo install sqlx-cli
cargo sqlx prepare


sqlx db create
sqlx database create
sqlx migrate add <NAME>
sqlx migrate run


$ export DATABASE_URL="sqlite:raffler.sqlite"
$ sqlx database create
$ sqlx migrate run