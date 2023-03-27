use miette::IntoDiagnostic;
use nanorand::Rng;
use serde::{Deserialize, Serialize};
use tide::{prelude::json, Request, Response};

#[derive(Deserialize, Serialize, Debug)]
struct Whisper {
    name: Option<String>,
    message: String,
    private: bool,
    #[serde(skip_deserializing)]
    #[serde(default = "Whisper::generate_snowflake")]
    snowflake: i64,
    #[serde(skip_deserializing)]
    #[serde(default = "Whisper::generate_timestamp")]
    timestamp: String,
}

impl Whisper {
    fn generate_snowflake() -> i64 {
        let mut rng = nanorand::WyRand::new();
        snowflake::SnowflakeIdGenerator::new(
            rng.generate_range(1..=1024),
            rng.generate_range(1..=1024),
        )
        .real_time_generate()
    }

    fn generate_timestamp() -> String {
        chrono::Utc::now()
            .with_timezone(&chrono_tz::Tz::Africa__Cairo)
            .format("%d %b %Y, %I:%M:%S %p")
            .to_string()
    }
}

impl Default for Whisper {
    fn default() -> Self {
        Self {
            name: Some("Anonymous".to_owned()),
            message: String::new(),
            private: false,
            snowflake: Self::generate_snowflake(),
            timestamp: Self::generate_timestamp(),
        }
    }
}

async fn add(mut req: Request<()>) -> tide::Result<Response> {
    let whisper: Whisper = req.body_json().await?;
    let mut res = Response::new(tide::StatusCode::Created);
    res.set_body(json!(&whisper));
    Ok(res)
}

async fn list(_req: Request<()>) -> tide::Result<tide::Body> {
    let whispers = vec![
        Whisper {
            name: Some("guy".to_owned()),
            message: "yep".to_owned(),
            ..Whisper::default()
        },
        Whisper {
            name: None,
            message: "nop".to_owned(),
            ..Whisper::default()
        },
        Whisper {
            message: "secret love letter".to_owned(),
            private: true,
            ..Whisper::default()
        },
    ];
    tide::Body::from_json(&whispers)
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    femme::start();

    let mut app = tide::new();

    app.at("/").get(|_| async { Ok("🐞") });

    app.at("/api/whisper").nest({
        let mut api = tide::new();

        api.at("/").get(list);
        api.at("/").post(add);

        api
    });

    app.listen("127.0.0.1:8714").await.into_diagnostic()?;

    Ok(())
}
