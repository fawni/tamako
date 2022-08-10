use crate::{WarpResult, Whisper};
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection};
use snowflake::SnowflakeIdGenerator;
use warp::Reply;

pub async fn add_whisper(
    collection: Collection<Whisper>,
    text: String,
    private: bool,
) -> WarpResult<impl Reply> {
    let whisper = Whisper {
        timestamp: Some(
            chrono::Utc::now()
                .with_timezone(&chrono_tz::Tz::Africa__Cairo)
                .format("%d %b %Y, %I:%M:%S %p")
                .to_string(),
        ),
        snowflake: Some(SnowflakeIdGenerator::new(1, 1).real_time_generate()),
        text,
        private,
    };

    collection.insert_one(&whisper, None).await.unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&whisper),
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn list_whispers(collection: Collection<Whisper>) -> WarpResult<impl Reply> {
    let whispers: Vec<Whisper> = collection
        .find(doc! { "private": false }, None)
        .await
        .unwrap()
        .try_collect()
        .await
        .expect("failed to collect whispers");

    Ok(warp::reply::json(&whispers))
}
