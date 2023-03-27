use nanorand::Rng;
use serde::{Deserialize, Serialize};
use tide::{prelude::json, Request, Response};

use crate::db::Database;

#[derive(Deserialize, Serialize, Debug)]
pub struct Whisper {
    pub name: Option<String>,

    pub message: String,

    #[serde(default)]
    pub private: bool,

    #[serde(skip_deserializing)]
    #[serde(default = "Whisper::generate_snowflake")]
    pub snowflake: i64,

    #[serde(skip_deserializing)]
    #[serde(default = "Whisper::generate_timestamp")]
    pub timestamp: String,
}

impl Whisper {
    fn generate_snowflake() -> i64 {
        let mut rng = nanorand::WyRand::new();
        snowflake::SnowflakeIdGenerator::new(
            rng.generate_range(1..=1024),
            rng.generate_range(1..=1024),
        )
        .real_time_generate()
    }

    fn generate_timestamp() -> String {
        chrono::Utc::now()
            .with_timezone(&chrono_tz::Tz::Africa__Cairo)
            .format("%d %b %Y, %I:%M:%S %p")
            .to_string()
    }
}

impl Default for Whisper {
    fn default() -> Self {
        Self {
            name: None,
            message: String::new(),
            private: false,
            snowflake: Self::generate_snowflake(),
            timestamp: Self::generate_timestamp(),
        }
    }
}

pub async fn add(mut req: Request<Database>) -> tide::Result<Response> {
    let whisper: Whisper = req.body_json().await?;
    let database = req.state();
    database.add(&whisper).await?;
    let mut res = Response::new(tide::StatusCode::Created);
    res.set_body(json!(&whisper));
    Ok(res)
}

pub async fn list(req: Request<Database>) -> tide::Result<tide::Body> {
    let database = req.state();
    let whispers = database.list().await?;
    tide::Body::from_json(&whispers)
}
