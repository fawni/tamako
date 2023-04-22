use std::sync::Arc;

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
            pool: SqlitePool::connect(dotenvy_macro::dotenv!("DATABASE_URL")).await?,
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
}

/// Opens a connection to the database
pub async fn open() -> tide::Result<Database> {
    Ok(Arc::new(DatabaseState::new().await?))
}
