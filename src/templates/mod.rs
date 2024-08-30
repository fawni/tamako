use actix_web::{
    dev::ServiceResponse, get, middleware::ErrorHandlerResponse, web, HttpRequest, HttpResponse,
    Responder,
};
use rinja::Template;

use crate::{
    api::{Private, Whisper},
    db::Database,
};

#[get("/robots.txt")]
pub async fn robots() -> impl Responder {
    "User-agent: *\nDisallow: /"
}

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
#[get("/")]
pub async fn home(
    req: HttpRequest,
    database: web::Data<Database>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
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

    Ok(WhispersTemplate::new(whispers, authenticated))
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
#[get("/auth")]
#[allow(clippy::unused_async)]
pub async fn auth() -> impl Responder {
    AuthTemplate::new()
}

/// The template that renders the not found page
#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate {
    /// The user of the instance
    pub user: User,
}

impl NotFoundTemplate {
    /// Returns a new not found template
    pub fn new() -> Self {
        Self {
            user: User::default(),
        }
    }
}

/// Renders the not found page
#[allow(clippy::unnecessary_wraps)]
pub fn not_found<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        HttpResponse::NotFound()
            .body(NotFoundTemplate::new().render().unwrap())
            .map_into_right_body(),
    )))
}
