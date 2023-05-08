use once_cell::sync::Lazy;

/// The secret token used for authentication
pub static TOKEN: Lazy<String> = Lazy::new(|| std::env::var("TAMAKO_SECRET").unwrap());

/// Validates the given secret
pub fn validate(secret: &str) -> bool {
    TOKEN.to_owned() == secret
}
