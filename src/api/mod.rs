pub use async_std::main;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tide::{prelude::json, Body, Request, Response, StatusCode};

use crate::db::Database;

mod snowflake;
mod webhook;

static SNOWFLAKE: OnceCell<snowflake::Snowflake> = OnceCell::new();

pub fn host() -> &'static str {
    static HOST: OnceCell<String> = OnceCell::new();
    HOST.get_or_init(|| std::env::var("HOST").unwrap())
}

pub fn port() -> &'static u16 {
    static PORT: OnceCell<u16> = OnceCell::new();
    PORT.get_or_init(|| std::env::var("PORT").unwrap().parse::<u16>().unwrap())
}

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
        if let Some(name) = &self.name {
            if name.len() > 32 {
                return Err(tide::Error::from_str(
                    StatusCode::BadRequest,
                    "name cannot be longer than 32 characters",
                ));
            }
        }
        if self.message.len() > 1024 {
            return Err(tide::Error::from_str(
                StatusCode::BadRequest,
                "whispers cannot be longer than 1024 characters",
            ));
        }

        Ok(())
    }

    /// Checks if the whisper is public
    const fn is_public(&self) -> bool {
        !self.private
    }

    /// Generates a unique snowflake for the whisper
    fn generate_snowflake() -> i64 {
        SNOWFLAKE
            .get_or_init(snowflake::Snowflake::new)
            .clone()
            .generate()
    }

    /// Generates an RFC3339 timestamp
    fn generate_timestamp() -> String {
        Utc::now().with_timezone(&Tz::Africa__Cairo).to_rfc3339()
    }

    /// Returns a pretty timestamp in the format: `dd MMM yyyy, hh:mm:ss a`
    fn pretty_timestamp(&self) -> String {
        DateTime::parse_from_rfc3339(&self.timestamp)
            .unwrap()
            .with_timezone(&Tz::Africa__Cairo)
            .format("%d %b %Y, %I:%M:%S %p")
            .to_string()
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

/// A trait for filtering out private whispers
pub trait Private {
    /// Filters out private whispers from a vector of whispers
    fn filter(self) -> Self;
}

impl Private for Vec<Whisper> {
    fn filter(self) -> Self {
        self.into_iter()
            .filter(Whisper::is_public)
            .collect::<Self>()
    }
}

/// Adds a new whisper
pub async fn add(mut req: Request<Database>) -> tide::Result<Response> {
    let mut whisper = req.body_json::<Whisper>().await?;
    whisper.validate()?;

    let database = req.state();
    database.add(&whisper).await?;
    match webhook::send(&whisper).await {
        Ok(_) => tide::log::info!("--> Webhook sent"),
        Err(e) => tide::log::error!("Webhook error --> {e}"),
    };

    let mut res = Response::new(StatusCode::Created);
    res.set_body(json!(&whisper));

    Ok(res)
}

#[derive(Deserialize, Default)]
#[serde(default)]
/// Query params for the list endpoint
struct ListParams {
    /// The amount of whispers to take
    take: Option<usize>,

    /// Whether to return pretty timestamps or not
    pretty: Option<bool>,
}

/// Lists all whispers
pub async fn list(req: Request<Database>) -> tide::Result<Body> {
    let database = req.state();
    let mut whispers = database.list().await?.filter();

    let params = req.query::<ListParams>()?;
    whispers.reverse();
    if let Some(n) = params.take {
        whispers.truncate(n);
    }
    if let Some(true) = params.pretty {
        whispers
            .iter_mut()
            .for_each(|w| w.timestamp = w.pretty_timestamp());
    }

    Body::from_json(&whispers)
}
