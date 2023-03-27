use std::sync::Arc;

use sqlx::{Pool, Sqlite, SqlitePool};

use crate::api::Whisper;

pub type Database = Arc<DatabaseState>;

pub struct DatabaseState {
    pool: Pool<Sqlite>,
}

impl DatabaseState {
    pub async fn new() -> tide::Result<Self> {
        let pool = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;
        Ok(Self { pool })
    }

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

    pub async fn list(&self) -> tide::Result<Vec<Whisper>> {
        let whispers = sqlx::query_as!(Whisper, "SELECT * FROM whispers")
            .fetch_all(&self.pool)
            .await?;

        Ok(whispers)
    }
}

pub async fn open() -> tide::Result<Database> {
    let result = Arc::new(DatabaseState::new().await?);

    Ok(result)
}
