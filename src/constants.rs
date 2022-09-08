use std::env;
use std::str::FromStr;

use hmac::digest::KeyInit;
use hmac::Hmac;
use lazy_static::lazy_static;
use sha2::Sha384;

lazy_static! {
    pub static ref PORT: u16 = match env::var("PORT") {
        Ok(var) => match u16::from_str(&var) {
            Ok(val) => val,
            Err(_) => panic!("invalid PORT env var"),
        },
        Err(_) => 3030,
    };
    pub static ref JWT_KEY: Hmac<Sha384> = {
        let secret = match env::var("SECRET") {
            Ok(var) => var,
            Err(_) => panic!("missing SECRET env var"),
        };
        type HmacSha384 = Hmac<Sha384>;
        HmacSha384::new_from_slice(secret.as_bytes()).unwrap()
    };
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("missing DATABASE_URL env var");
    pub static ref RUST_LOG: String = match env::var("RUST_LOG") {
        Ok(var) => var,
        Err(_) => "tamako=info".to_string(),
    };
}
