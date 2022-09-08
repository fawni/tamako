use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};

use crate::constants;

pub fn make_token() -> String {
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    Token::new(header, "tamako")
        .sign_with_key(&*constants::JWT_KEY)
        .unwrap()
        .as_str()
        .to_string()
}

pub fn verify_token(token: Option<String>) -> Result<bool, jwt::error::Error> {
    let token_str = match token {
        Some(token) => token,
        None => return Ok(false),
    };

    let token: Token<Header, String, _> =
        Token::parse_unverified(&token_str)?.verify_with_key(&*constants::JWT_KEY)?;
    Ok(token.claims().eq("tamako"))
}
