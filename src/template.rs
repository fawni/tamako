use askama::Template;
use tide::Request;

use crate::api::{self, Whisper};

/// The template that renders the whispers page
#[derive(Template)]
#[template(path = "tamako.html")]
pub struct WhispersTemplate {
    /// The whispers to be rendered
    pub whispers: Vec<Whisper>,
}

impl WhispersTemplate {
    /// Returns a new template with the given whispers
    pub fn new(whispers: Vec<Whisper>) -> Self {
        Self { whispers }
    }
}

/// Renders the whispers page
pub async fn render(_req: Request<()>) -> tide::Result<tide::Response> {
    let (host, port) = (api::host(), api::port());
    let whispers = reqwest::get(format!("http://{host}:{port}/api/whisper"))
        .await?
        .json::<Vec<Whisper>>()
        .await?;

    Ok(WhispersTemplate::new(whispers).into())
}
