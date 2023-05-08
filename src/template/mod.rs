use askama::Template;
use tide::{Request, Response};

use crate::{
    api::{Private, Whisper},
    db::Database,
};

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
pub async fn tamako(req: Request<Database>) -> tide::Result<Response> {
    let database = req.state();
    let mut whispers = if let Some(cookie) = req.cookie("token") {
        if crate::auth::validate(cookie.value()) {
            database.list().await?
        } else {
            database.list().await?.filter()
        }
    } else {
        database.list().await?.filter()
    };
    whispers.reverse();

    Ok(WhispersTemplate::new(whispers).into())
}

/// The template that renders the auth page
#[derive(Template)]
#[template(path = "auth.html")]
pub struct AuthTemplate;

/// Renders the auth page
pub async fn auth(_req: Request<Database>) -> tide::Result<Response> {
    Ok(AuthTemplate.into())
}
