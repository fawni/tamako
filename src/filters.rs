use crate::{handlers, Whisper};
// use argon2::{
//     password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
//     Argon2, PasswordVerifier,
// };
use crate::jwt;
use mongodb::Collection;
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
            handlers::list_whispers(collection.clone(), jwt::verify_token(token).unwrap())
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

// currently we assume that the header is set to the unhashed password.
// todo: set the header to the hashed password and then verify the hash.
// todo: clean up
// pub fn verify_token(token: Option<String>) -> Result<bool, argon2::password_hash::Error> {
//     let input = match token {
//         Some(token) => token,
//         None => return Ok(false),
//     };

//     let password = env::var("PASSWORD").unwrap();
//     // println!("Password: {:?}", password);
//     let salt = SaltString::generate(&mut OsRng);

//     let input_hash = Argon2::default().hash_password(input.as_bytes(), &salt)?;
//     println!("Password hash: {}", input_hash);

//     let matches = Argon2::default()
//         .verify_password(password.as_bytes(), &input_hash)
//         .is_ok();

//     Ok(matches)
// }

pub fn set_cookie(
    name: String,
    value: String,
) -> impl Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    warp::any().map(warp::reply).map(move |reply| {
        warp::reply::with_header(reply, "set-cookie", &format!("{}={}", name, value))
    })
}
