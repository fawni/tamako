set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Setup the database
setup:
    sqlx db setup

# Exessive clippy lints
lint:
    cargo clippy --locked -- -W clippy::pedantic -W clippy::nursery

push:
    git push
    git push gh