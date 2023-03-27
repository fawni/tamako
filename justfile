set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Runs exessive clippy lints (possible false positives so just warn)
lint:
    cargo clippy --locked -- -W clippy::pedantic -W clippy::nursery