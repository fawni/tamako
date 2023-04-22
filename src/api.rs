use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tide::{prelude::json, Body, Request, Response, StatusCode};

use crate::{db::Database, snowflake::Snowflake};

static SNOWFLAKE: OnceCell<Snowflake> = OnceCell::new();

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct Whisper {
    /// The name of the whisperer
    pub name: Option<String>,

    /// The message of the whisper
    pub message: String,

    /// Whether the whisper is private or not
    pub private: bool,

    /// The unique snowflake of the whisper
    #[serde(skip_deserializing)]
    pub snowflake: i64,

    /// The timestamp of the whisper
    #[serde(skip_deserializing)]
    pub timestamp: String,
}

impl Whisper {
    /// Validates the accuracy of the whisper's data
    fn validate(&mut self) -> tide::Result<()> {
        self.name = self.name.take().filter(|name| !name.is_empty());
        if self.message.is_empty() {
            return Err(tide::Error::from_str(
                StatusCode::BadRequest,
                "whispers cannot be empty",
            ));
        }
        Ok(())
    }

    /// Checks if the whisper is public
    fn is_public(&self) -> bool {
        !self.private
    }

    /// Generates a unique snowflake for the whisper
    fn generate_snowflake() -> i64 {
        SNOWFLAKE.get_or_init(Snowflake::new).clone().generate()
    }

    /// Generates a timestamp in the format: `dd MMM yyyy, hh:mm:ss a`
    fn generate_timestamp() -> String {
        chrono::Utc::now()
            .with_timezone(&chrono_tz::Tz::Africa__Cairo)
            .format("%d %b %Y, %I:%M:%S %p")
            .to_string()
    }

    /// Filters out private whispers from a vector of whispers
    fn filter(v: Vec<Self>) -> Vec<Self> {
        v.into_iter()
            .filter(|whisper| whisper.is_public())
            .collect::<Vec<Whisper>>()
    }
}

impl Default for Whisper {
    /// Creates a new whisper with default values
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
    whisper.validate()?;

    let database = req.state();
    database.add(&whisper).await?;

    let mut res = Response::new(StatusCode::Created);
    res.set_body(json!(&whisper));

    Ok(res)
}

pub async fn list(req: Request<Database>) -> tide::Result<Body> {
    let database = req.state();
    let whispers = Whisper::filter(database.list().await?);

    Body::from_json(&whispers)
}
