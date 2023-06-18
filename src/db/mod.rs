use std::sync::Arc;

use async_std::process::Command;
use sqlx::postgres::PgPool;

use crate::api::Whisper;

/// Convenience type for the database state
pub type Database = Arc<DatabaseState>;

/// The database state that holds the connection pool
pub struct DatabaseState {
    /// The connection pool
    pool: PgPool
}

impl DatabaseState {
    /// Creates a new database state
    pub async fn new() -> tide::Result<Self> {
        Ok(Self {
            pool: PgPool::connect(&std::env::var("DATABASE_URL")?).await?,
        })
    }

    /// Adds a whisper to the database
    pub async fn add(&self, whisper: &Whisper) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO whispers (name, message, private, snowflake, timestamp) VALUES ($1, $2, $3, $4, $5)",
            whisper.name,
            whisper.message,
            whisper.private,
            whisper.snowflake,
            whisper.timestamp
        ).execute(&self.pool).await?;

        Ok(())
    }

    /// Lists all whispers from the database
    pub async fn list(&self) -> sqlx::Result<Vec<Whisper>> {
        sqlx::query_as!(Whisper, "SELECT * FROM whispers")
            .fetch_all(&self.pool)
            .await
    }

    /// Gets a whisper from the database
    pub async fn get(&self, snowflake: i64) -> sqlx::Result<Whisper> {
        sqlx::query_as!(
            Whisper,
            "SELECT * FROM whispers WHERE snowflake = $1",
            snowflake
        )
        .fetch_one(&self.pool)
        .await
    }

    /// Deletes a whisper from the database
    pub async fn delete(&self, snowflake: i64) -> sqlx::Result<()> {
        sqlx::query!("SELECT * FROM whispers WHERE snowflake = $1", snowflake)
            .fetch_one(&self.pool)
            .await?;

        sqlx::query!("DELETE FROM whispers WHERE snowflake = $1", snowflake)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

/// Opens a connection to the database
pub async fn open() -> tide::Result<Database> {
    Command::new("sqlx").args(["db", "create"]).output().await?;
    let database = Arc::new(DatabaseState::new().await?);
    sqlx::migrate!().run(&database.pool).await?;

    Ok(database)
}
