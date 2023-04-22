set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Setup the database
setup:
    sqlx db create
    sqlx migrate run

# Exessive clippy lints
lint:
    cargo clippy --locked -- -W clippy::pedantic -W clippy::nursery