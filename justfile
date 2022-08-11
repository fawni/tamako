set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Build
build: backend frontend

# Runs clippy
check:
    cargo clippy --locked -- -D warnings

# Build backend
backend:
    cargo build --locked --release

# Build frontend
frontend:
    pnpm -C frontend build
