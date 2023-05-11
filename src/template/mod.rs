use askama::Template;
use tide::{Request, Response};

use crate::{
    api::{Private, Whisper},
    auth::validate_cookie,
    db::Database,
};

/// The template that renders the whispers page
#[derive(Template)]
#[template(path = "tamako.html")]
pub struct WhispersTemplate {
    /// The whispers to be rendered
    pub whispers: Vec<Whisper>,

    /// Whether the user is authenticated or not
    pub authenticated: bool,
}

impl WhispersTemplate {
    /// Returns a new template with the given whispers
    pub fn new(whispers: Vec<Whisper>, authenticated: bool) -> Self {
        Self {
            whispers,
            authenticated,
        }
    }
}

/// Renders the whispers page
pub async fn tamako(req: Request<Database>) -> tide::Result<Response> {
    let database = req.state();
    let authenticated = validate_cookie(&req);
    // If the user is authenticated, show all whispers, otherwise only show public whispers.
    let mut whispers = if authenticated {
        database.list().await?
    } else {
        database.list().await?.filter()
    };
    whispers.reverse();

    Ok(WhispersTemplate::new(whispers, authenticated).into())
}

/// The template that renders the auth page
#[derive(Template)]
#[template(path = "auth.html")]
pub struct AuthTemplate;

/// Renders the auth page
pub async fn auth(_req: Request<Database>) -> tide::Result<Response> {
    Ok(AuthTemplate.into())
}
