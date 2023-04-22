use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tide::{prelude::json, Request, Response};

use crate::{db::Database, snowflake::Snowflake};

static SNOWFLAKE: OnceCell<Snowflake> = OnceCell::new();

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct Whisper {
    pub name: Option<String>,

    pub message: String,

    pub private: bool,

    #[serde(skip_deserializing)]
    pub snowflake: i64,

    #[serde(skip_deserializing)]
    pub timestamp: String,
}

impl Whisper {
    fn generate_snowflake() -> i64 {
        SNOWFLAKE.get_or_init(Snowflake::new).clone().generate()
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
    let mut whisper = req.body_json::<Whisper>().await?;
    whisper.name = whisper.name.filter(|name| !name.is_empty());
    if whisper.message.is_empty() {
        return Ok(Response::new(tide::StatusCode::BadRequest));
    }
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
