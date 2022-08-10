use crate::{handlers, Whisper};
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
        .and_then(move || handlers::list_whispers(collection.clone()))
}

pub fn whispers_add(
    collection: Collection<Whisper>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path("whisper")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 4))
        .and(warp::body::json())
        .and_then(move |whisper: Whisper| {
            handlers::add_whisper(collection.clone(), whisper.text, whisper.private)
        })
}
