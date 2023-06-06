use askama::Template;
use tide::{Request, Response};

use crate::{
    api::{Private, Whisper},
    db::Database,
};

/// User information for the instance
pub struct User {
    /// The name of the user
    pub name: String,

    /// The description of the user
    pub description: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: std::env::var("TAMAKO_USER_NAME").unwrap_or_else(|_| "tamako".to_owned()),
            description: std::env::var("TAMAKO_USER_DESCRIPTION")
                .unwrap_or_else(|_| "Cozy anonymous whispers 🐞".to_owned()),
        }
    }
}

/// The template that renders the whispers page
#[derive(Template)]
#[template(path = "home.html")]
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
    pub fn new(whispers: Vec<Whisper>, authenticated: bool) -> Self {
        Self {
            whispers,
            authenticated,
            user: User::default(),
        }
    }
}

/// Renders the whispers page
pub async fn home(req: Request<Database>) -> tide::Result<Response> {
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

    Ok(WhispersTemplate::new(whispers, authenticated).into())
}

/// The template that renders the auth page
#[derive(Template)]
#[template(path = "auth.html")]
pub struct AuthTemplate {
    /// The user of the instance
    pub user: User,
}

impl AuthTemplate {
    /// Returns a new auth template
    pub fn new() -> Self {
        Self {
            user: User::default(),
        }
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
