use std::sync::Arc;

use async_std::process::Command;
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::api::Whisper;

/// Convenience type for the database state
pub type Database = Arc<DatabaseState>;

/// The database state that holds the connection pool
pub struct DatabaseState {
    /// The connection pool
    pool: Pool<Sqlite>,
}

impl DatabaseState {
    /// Creates a new database state
    pub async fn new() -> tide::Result<Self> {
        Ok(Self {
            pool: SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?,
        })
    }

    /// Adds a whisper to the database
    pub async fn add(&self, whisper: &Whisper) -> tide::Result<()> {
        sqlx::query!(
            "INSERT INTO whispers (name, message, private, snowflake, timestamp) VALUES (?, ?, ?, ?, ?)",
            whisper.name,
            whisper.message,
            whisper.private,
            whisper.snowflake,
            whisper.timestamp
        ).execute(&self.pool).await?;

        Ok(())
    }

    /// Lists all whispers from the database
    pub async fn list(&self) -> tide::Result<Vec<Whisper>> {
        let whispers = sqlx::query_as!(Whisper, "SELECT * FROM whispers")
            .fetch_all(&self.pool)
            .await?;

        Ok(whispers)
    }

    /// Gets a whisper from the database
    pub async fn get(&self, snowflake: &str) -> tide::Result<Whisper> {
        let whisper = sqlx::query_as!(
            Whisper,
            "SELECT * FROM whispers WHERE snowflake = ?",
            snowflake
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(whisper)
    }

    /// Deletes a whisper from the database
    pub async fn delete(&self, snowflake: &str) -> tide::Result<()> {
        sqlx::query!("SELECT * FROM whispers WHERE snowflake = ?", snowflake)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| tide::Error::from_str(tide::StatusCode::NotFound, "Whisper not found"))?;
        sqlx::query!("DELETE FROM whispers WHERE snowflake = ?", snowflake)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

/// Opens a connection to the database
pub async fn open() -> tide::Result<Database> {
    Command::new("sqlx").args(["db", "create"]).output().await?;
    let database = Arc::new(DatabaseState::new().await?);
    sqlx::migrate!("./migrations").run(&database.pool).await?;

    Ok(database)
}
