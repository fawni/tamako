use crate::{handlers, Whisper};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use mongodb::Collection;
use std::env;
use warp::{Filter, Rejection};

pub fn whispers(
    collection: Collection<Whisper>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path("api").and(whispers_list(collection.clone()).or(whispers_add(collection)))
}

pub fn whispers_list(
    collection: Collection<Whisper>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path("whispers")
        .and(warp::get())
        .and(warp::header::optional::<String>("auth"))
        .and_then(move |token| {
            handlers::list_whispers(collection.clone(), verify_token(token).unwrap())
        })
}

pub fn whispers_add(
    collection: Collection<Whisper>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path("whisper")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and_then(move |whisper: Whisper| {
            handlers::add_whisper(
                collection.clone(),
                whisper.text,
                whisper.private,
                whisper.name,
            )
        })
}

pub fn verify_token(token: Option<String>) -> Result<bool, argon2::password_hash::Error> {
    // todo!()
    let input = match token {
        Some(token) => token,
        None => return Ok(false),
    };

    let password = env::var("PASSWORD").unwrap();
    // println!("Password: {:?}", password);
    let salt = SaltString::generate(&mut OsRng);

    let input_hash = Argon2::default().hash_password(input.as_bytes(), &salt)?;
    println!("Password hash: {}", input_hash);

    let matches = Argon2::default()
        .verify_password(password.as_bytes(), &input_hash)
        .is_ok();

    Ok(matches)
}
