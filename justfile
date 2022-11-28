set dotenv-load
database_file := env_var_or_default("DATABASE_FILE", "demo.db")
database_url := "sqlite://" + database_file

# Run tests using nextest
test:
	cargo nextest run

alias m := migrate
# Run SQLx migrations on {{database_url}}
migrate:
    sqlx migrate run -D {{database_url}}

alias am := add-mig
# Add reversible SQLx migration with the given NAME
add-mig NAME:
    sqlx migrate add -r {{NAME}}

alias cdb := create-db
# Bootstrap the demo database file by creating the file and running migrations
create-db: rmdb
    touch {{database_file}}
    sqlx migrate run -D {{database_url}} --sqlite-create-db-wal true

# Remove the demo database file
rmdb:
    -rm {{database_file}}*

alias ae := add-entities
# Generate sea-orm entities from list of comma-separated TABLES to the given LOCATION
add-entities TABLES LOCATION:
    #sea-orm-cli generate entity -u {{database_url}} --tables {{TABLES}} -o {{LOCATION}} --ignore-tables events,snapshots
    # Temporary until https://github.com/SeaQL/sea-orm/pull/1245 is released
    ~/dev/thirdparty/rust/sea-orm/sea-orm-cli/target/debug/sea-orm-cli generate entity -u {{database_url}} --tables {{TABLES}} -o {{LOCATION}} --ignore-tables events,snapshots --with-serde both

