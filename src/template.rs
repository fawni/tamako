use askama::Template;
use tide::Request;

use crate::api::Whisper;

#[derive(Template)]
#[template(path = "tamako.html")]
pub struct WhispersTemplate {
    pub whispers: Vec<Whisper>,
}

pub async fn render(_req: Request<()>) -> tide::Result<tide::Response> {
    let host = dotenvy_macro::dotenv!("HOST");
    let port = dotenvy_macro::dotenv!("PORT").parse::<u16>()?;
    let whispers = reqwest::get(format!("http://{}:{}/api/whisper", host, port))
        .await?
        .json::<Vec<Whisper>>()
        .await?;
    let res = WhispersTemplate { whispers };

    Ok(res.into())
}
