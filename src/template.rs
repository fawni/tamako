use askama::Template;
use tide::Request;

use crate::api::Whisper;

/// The template that renders the whispers page
#[derive(Template)]
#[template(path = "tamako.html")]
pub struct WhispersTemplate {
    /// The whispers to be rendered
    pub whispers: Vec<Whisper>,
}

/// Renders the whispers page
pub async fn render(_req: Request<()>) -> tide::Result<tide::Response> {
    let host = dotenvy_macro::dotenv!("HOST");
    let port = dotenvy_macro::dotenv!("PORT").parse::<u16>()?;
    let whispers = reqwest::get(format!("http://{host}:{port}/api/whisper"))
        .await?
        .json::<Vec<Whisper>>()
        .await?;
    let res = WhispersTemplate { whispers };

    Ok(res.into())
}
