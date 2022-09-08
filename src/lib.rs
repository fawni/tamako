use serde::{Deserialize, Serialize};
use warp::Rejection;

pub mod constants;
pub mod db;
pub mod filters;
pub mod handlers;
pub mod jwt;

pub type WarpResult<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Whisper {
    pub text: String,
    pub private: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake: Option<i64>,
}
