use actix_web::{
    delete,
    error::{ErrorBadRequest, ErrorForbidden, ErrorInternalServerError, ErrorNotFound},
    get, post, web, HttpRequest, HttpResponse,
};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{auth, db::Database};

mod snowflake;
mod webhook;

use self::snowflake::Snowflake;

/// The host of the server
pub static HOST: Lazy<String> =
    Lazy::new(|| std::env::var("TAMAKO_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned()));
/// The port of the server
pub static PORT: Lazy<u16> = Lazy::new(|| {
    std::env::var("TAMAKO_PORT")
        .unwrap_or_else(|_| "8715".to_owned())
        .parse::<u16>()
        .unwrap_or(8715)
});

/// The snowflake generator
static SNOWFLAKE: Lazy<Snowflake> = Lazy::new(Snowflake::new);

macro_rules! bail {
    ($e:expr) => {
        return Err($e);
    };
}

/// The representation of a whisper
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
    fn validate(&mut self) -> actix_web::Result<()> {
        self.name = self.name.take().filter(|name| !name.is_empty());
        if self.message.is_empty() {
            bail!(ErrorBadRequest("whispers cannot be empty"));
        }
        if let Some(name) = &self.name {
            if name.len() > 32 {
                bail!(ErrorBadRequest("name cannot be longer than 32 characters"));
            }
        }
        if self.message.len() > 1024 {
            bail!(ErrorBadRequest(
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
        SNOWFLAKE.clone().generate()
    }

    /// Generates an RFC3339 timestamp
    fn generate_timestamp() -> String {
        Utc::now().with_timezone(&Tz::Africa__Cairo).to_rfc3339()
    }

    /// Returns a pretty timestamp in the format: `dd MMM yyyy, hh:mm:ss a`
    pub fn pretty_timestamp(&self) -> String {
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

/// Validates the secret key
#[allow(clippy::unused_async)]
#[post("/api/auth")]
pub async fn authentication(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if !auth::validate_header(&req) {
        bail!(ErrorForbidden("Invalid token"));
    }

    Ok(HttpResponse::Ok().body("Validated"))
}

/// Adds a new whisper
pub async fn add(
    database: web::Data<Database>,
    mut whisper: web::Json<Whisper>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    whisper.validate()?;

    database.add(&whisper).await?;
    match webhook::send(&whisper).await {
        Ok(_) => log::info!("Webhook sent successfully"),
        Err(e) => log::error!("Webhook error: {e}"),
    };

    Ok(HttpResponse::Created().json(whisper))
}

#[derive(Deserialize, Default)]
#[serde(default)]
/// Query params for the list method
pub struct ListParams {
    /// The number of whispers to return
    limit: Option<usize>,

    /// The number of whispers to skip
    offset: Option<usize>,

    /// Whether to return pretty timestamps or not
    pretty: Option<bool>,
}

/// Lists all whispers
#[get("/api/whisper")]
pub async fn list(
    req: HttpRequest,
    params: web::Query<ListParams>,
    database: web::Data<Database>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let mut whispers = database.list().await?;

    // Filter out private whispers if the token is invalid or not provided
    if !auth::validate_header(&req) {
        whispers = whispers.filter();
    }

    // Reverse the order of the whispers so that the latest whispers are at the top
    whispers.reverse();

    // Skip `n` whispers if the `offset` param is provided
    if let Some(n) = params.offset {
        whispers = whispers.into_iter().skip(n).collect();
    }

    // Truncate whispers if the `limit` param is provided
    if let Some(n) = params.limit {
        whispers.truncate(n);
    }

    // Convert the whispers' timestamps to pretty timestamps if the `pretty` param is provided
    if params.pretty == Some(true) {
        whispers
            .iter_mut()
            .for_each(|w| w.timestamp = w.pretty_timestamp());
    }

    Ok(HttpResponse::Ok().json(whispers))
}

#[derive(Deserialize, Default)]
#[serde(default)]
/// Query params for the get method
pub struct GetParams {
    /// Whether to return pretty timestamps or not
    pretty: Option<bool>,
}

/// Gets a whisper by its snowflake
#[get("/api/whisper/{snowflake}")]
pub async fn get(
    path: web::Path<i64>,
    params: web::Query<GetParams>,
    database: web::Data<Database>,
) -> actix_web::Result<HttpResponse> {
    let snowflake = path.into_inner();
    let mut whisper = database.get(snowflake).await.map_err(|e| {
        log::warn!("Database Error: {} [{0:?}]", e);
        if let sqlx::Error::RowNotFound = e {
            ErrorNotFound("Invalid Whisper ID")
        } else {
            ErrorInternalServerError(e)
        }
    })?;

    // Convert the whispers' timestamps to pretty timestamps if the `pretty` param is provided
    if params.pretty == Some(true) {
        whisper.timestamp = whisper.pretty_timestamp();
    }

    Ok(HttpResponse::Ok().json(whisper))
}

/// Deletes a whisper
#[delete("/api/whisper/{snowflake}")]
pub async fn delete(
    req: HttpRequest,
    path: web::Path<i64>,
    database: web::Data<Database>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    if !auth::validate_header(&req) {
        bail!(Box::new(ErrorForbidden("Invalid token")));
    }

    let snowflake = path.into_inner();
    database
        .delete(snowflake)
        .await
        .map_err(|_| ErrorNotFound(format!("Whisper {snowflake} not found")))?;

    Ok(HttpResponse::Ok().body(format!("Deleted {snowflake}")))
}
