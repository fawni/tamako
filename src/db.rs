use std::sync::Arc;

use sqlx::{Pool, Sqlite, SqlitePool};

use crate::Whisper;

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

// pub async fn set(&self, id: &String, status: u8) -> miette::Result<()> {
//     let status = status.to_string();
//     sqlx::query!("INSERT INTO favorites (id, status) VALUES (?, ?) ON CONFLICT(id) DO UPDATE SET status = excluded.status", id, status)
//         .execute(&self.pool)
//         .await.into_diagnostic()?;

//     Ok(())
// }

// pub async fn get_status(&self, id: &String) -> miette::Result<i64> {
//     let rec = sqlx::query!("SELECT status FROM favorites WHERE id = ?", id)
//         .fetch_one(&self.pool)
//         .await
//         .into_diagnostic()?;

//     Ok(rec.status)
// }

// pub async fn get_new_favorites(&self) -> miette::Result<Vec<String>> {
//     let favorites = sqlx::query!("SELECT id FROM favorites WHERE status = 0")
//         .fetch_all(&self.pool)
//         .await
//         .into_diagnostic()?
//         .into_iter()
//         .map(|r| r.id)
//         .collect::<Vec<String>>();

//     Ok(favorites)
// }
// }

pub async fn open() -> tide::Result<Database> {
    let result = Arc::new(DatabaseState::new().await?);

    Ok(result)
}
