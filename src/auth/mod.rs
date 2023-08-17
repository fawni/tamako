use once_cell::sync::Lazy;

/// The secret token used for authentication
pub static TOKEN: Lazy<String> = Lazy::new(|| std::env::var("TAMAKO_SECRET").unwrap());

/// Validates the given secret
pub fn validate(secret: &str) -> bool {
    TOKEN.eq(secret)
}

/// Validates the cookie of the given request
pub fn validate_cookie(req: &actix_web::HttpRequest) -> bool {
    req.cookie("token")
        .map_or(false, |cookie| validate(cookie.value()))
}

/// Validates the header of the given request
pub fn validate_header(req: &actix_web::HttpRequest) -> bool {
    req.headers()
        .get("token")
        .map_or(false, |header| validate(header.to_str().unwrap()))
}
