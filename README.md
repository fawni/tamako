# tamako🐞

[![status-badge](https://ci.codeberg.org/api/badges/12523/status.svg)](https://ci.codeberg.org/repos/12523)
[![crates.io](https://img.shields.io/crates/v/tamako.svg)](https://crates.io/crates/tamako)
[![dependency status](https://deps.rs/repo/codeberg/fawn/tamako/status.svg)](https://deps.rs/repo/codeberg/fawn/tamako)

tamako is a cozy, minimalistic, single-user, _anonymous_ whispers service

![scrot](meta/scrot.png)

## Prerequisites

- [sqlx-cli](https://crates.io/crates/sqlx-cli)
- a postgresql database

## Environment variables

| Name                      | Type   | Default                   | Notes                                                                                                              |
| ------------------------- | ------ | ------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `TAMAKO_HOST`             | String | 127.0.0.1                 | the host to run the server on                                                                                      |
| `TAMAKO_PORT`             | u16    | 8715                      | the port to serve the server on                                                                                    |
| `DATABASE_URL`            | String | _none_                    | the database url. this should be a valid postgresql connection url                                                 |
| `WEBHOOK_URL`             | String | _none_                    | _(optional)_ the discord webhook url. this will be used to send _private and public_ whispers to a discord channel |
| `TAMAKO_SECRET`           | String | _none_                    | the secret key used for authentication. think of it like a master password                                         |
| `TAMAKO_USER_NAME`        | String | tamako                    | _(optional)_ used in the fronted header                                                                            |
| `TAMAKO_USER_DESCRIPTION` | String | Cozy anonymous whispers 🐞 | _(optional)_ used in the fronted header                                                                            |

## Installation

### Docker Compose (Recommended)

This will also create a postgres service container.

1. Clone the repo
2. Configure `docker-compose.yml` to match your setup, most importantly change the environment variables
3. `docker compose up -d`

### Locally

1. Clone the repo
2. Rename `.env.example` to `.env` and change env variables inside it
3. Build tamako: `cargo build --release`
4. Run tamako: `./target/release/tamako`

### Docker

Using docker directly is possible, but you have to configure the container yourself.

```sh
docker run -d --name tamako --restart unless-stopped -p 8715:8715 fawni/tamako:latest
```

## TUI

tamako comes with a pretty little tui frontend for it called mochi

![mochi](meta/mochi.png)

### Installation

```sh
go install codeberg.org/fawn/tamako/cmd/mochi@latest
```
### Usage

```sh
mochi --url https://tamako.fawn.moe
```

`mochi -h` for more info.

## License

[OSL-3.0](LICENSE)