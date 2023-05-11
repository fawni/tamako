use once_cell::sync::Lazy;

/// The secret token used for authentication
pub static TOKEN: Lazy<String> = Lazy::new(|| std::env::var("TAMAKO_SECRET").unwrap());

/// Validates the given secret
pub fn validate(secret: &str) -> bool {
    TOKEN.eq(secret)
}

/// Validates the cookie of the given request
pub fn validate_cookie<T>(req: &tide::Request<T>) -> bool {
    let Some(cookie) = req.cookie("token") else {
        return false;
    };

    validate(cookie.value())
}

/// Validates the header of the given request
pub fn validate_header<T>(req: &tide::Request<T>) -> bool {
    let Some(header) = req.header("token") else {
        return false;
    };

    validate(&header[0].to_string())
}
