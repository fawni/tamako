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

    /// Whether the user is authenticated or not
    pub authenticated: bool,

    /// The user of the instance
    pub user: User,
}

impl WhispersTemplate {
    /// Returns a new template with the given whispers
    pub fn new(whispers: Vec<Whisper>, authenticated: bool, user: User) -> Self {
        Self {
            whispers,
            authenticated,
            user,
        }
    }
}

pub struct User {
    pub name: String,
    pub description: String,
}

impl User {
    pub fn new() -> Self {
        Self {
            name: std::env::var("TAMAKO_USER_NAME").unwrap_or_else(|_| Self::default_name()),
            description: std::env::var("TAMAKO_USER_DESCRIPTION")
                .unwrap_or_else(|_| Self::default_description()),
        }
    }

    fn default_name() -> String {
        "tamako".to_owned()
    }

    fn default_description() -> String {
        "Cozy anonymous whispers 🐞".to_owned()
    }
}

/// Renders the whispers page
pub async fn tamako(req: Request<Database>) -> tide::Result<Response> {
    let database = req.state();
    let authenticated = crate::auth::validate_cookie(&req);
    // If the user is authenticated, show all whispers, otherwise only show public whispers.
    let whispers = if authenticated {
        database.list().await?
    } else {
        database.list().await?.filter()
    }
    .into_iter()
    .rev()
    .collect::<Vec<Whisper>>();

    Ok(WhispersTemplate::new(whispers, authenticated, User::new()).into())
}

/// The template that renders the auth page
#[derive(Template)]
#[template(path = "auth.html")]
pub struct AuthTemplate {
    pub user: User,
}

impl AuthTemplate {
    pub fn new() -> Self {
        Self { user: User::new() }
    }
}

/// Renders the auth page
#[allow(clippy::unused_async)]
pub async fn auth<T>(_: Request<T>) -> tide::Result<Response>
where
    T: Send,
{
    Ok(AuthTemplate::new().into())
}
