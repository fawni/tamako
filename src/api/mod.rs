pub use async_std::main;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tide::{prelude::json, Body, Request, Response, StatusCode};

use crate::{auth, db::Database};

mod snowflake;
mod webhook;

pub static HOST: Lazy<String> = Lazy::new(|| std::env::var("HOST").unwrap());
pub static PORT: Lazy<u16> = Lazy::new(|| std::env::var("PORT").unwrap().parse::<u16>().unwrap());

static SNOWFLAKE: Lazy<snowflake::Snowflake> = Lazy::new(snowflake::Snowflake::new);

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
    /// Preforms basic validation checks for the whisper
    fn validate(&mut self) -> Result<(), Response> {
        self.name = self.name.take().filter(|name| !name.is_empty());
        if self.message.is_empty() {
            return Err(Response::builder(StatusCode::BadRequest)
                .body("whispers cannot be empty")
                .build());
        }
        if let Some(name) = &self.name {
            if name.len() > 32 {
                return Err(Response::builder(StatusCode::BadRequest)
                    .body("name cannot be longer than 32 characters")
                    .build());
            }
        }
        if self.message.len() > 1024 {
            return Err(Response::builder(StatusCode::BadRequest)
                .body("whispers cannot be longer than 1024 characters")
                .build());
        }

        Ok(())
    }

    /// Checks if the whisper is public
    const fn is_public(&self) -> bool {
        !self.private
    }

    /// Generates a unique snowflake for the whisper
    fn generate_snowflake() -> i64 {
        SNOWFLAKE.clone().generate()
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

/// Authenticates the secret key
pub async fn auth(req: Request<Database>) -> tide::Result<Response> {
    let token = if let Some(value) = req.header("token") {
        &value[0]
    } else {
        return Ok(Response::builder(StatusCode::BadRequest)
            .body("Missing token header")
            .build());
    };

    if !auth::validate(&token.to_string()) {
        return Ok(Response::builder(StatusCode::Forbidden)
            .body("Invalid token")
            .build());
    }

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from("Authenticated"));

    Ok(res)
}

/// Adds a new whisper
pub async fn add(mut req: Request<Database>) -> tide::Result<Response> {
    let mut whisper = req.body_json::<Whisper>().await?;
    if let Err(res) = whisper.validate() {
        return Ok(res);
    }

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
    /// The number of whispers to return
    limit: Option<usize>,

    /// Whether to return pretty timestamps or not
    pretty: Option<bool>,
}

/// Lists all whispers
pub async fn list(req: Request<Database>) -> tide::Result<Body> {
    let database = req.state();
    let mut whispers = database.list().await?;

    // Filter out private whispers if the token is invalid or not provided
    match req.header("token") {
        Some(header_values) if auth::validate(&header_values[0].to_string()) => (),
        _ => whispers = whispers.filter(),
    }

    // Reverse the order of the whispers so that the latest whispers are at the top
    whispers.reverse();

    let params = req.query::<ListParams>()?;
    // Truncate the whispers if the `limit` param is provided
    if let Some(n) = params.limit {
        whispers.truncate(n);
    }

    // Convert the whispers' timestamps to pretty timestamps if the `pretty` param is provided
    if let Some(true) = params.pretty {
        whispers
            .iter_mut()
            .for_each(|w| w.timestamp = w.pretty_timestamp());
    }

    Body::from_json(&whispers)
}

/// Deletes a whisper
pub async fn delete(req: Request<Database>) -> tide::Result<Response> {
    match req.header("token") {
        Some(header_values) if auth::validate(&header_values[0].to_string()) => (),
        _ => {
            return Err(tide::Error::from_str(
                StatusCode::Forbidden,
                "Invalid token",
            ))
        }
    }

    let snowflake = req.param("snowflake")?;
    let database = req.state();
    database.delete(snowflake).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(format!("Deleted {snowflake}"));

    Ok(res)
}
