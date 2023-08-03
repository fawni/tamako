mod api;
mod auth;
mod db;
mod templates;

#[api::main]
async fn main() -> tide::Result<()> {
    femme::start();
    dotenvy::dotenv().ok();

    let database = db::open().await?;
    let mut tamako = tide::with_state(database);
    tamako.with(tide_compress::CompressMiddleware::new());

    tamako.at("/").get(templates::home);
    tamako.at("/auth").get(templates::auth);
    tamako.at("/assets").serve_dir("assets")?;

    tamako.at("/api/health").get(|_| async move { Ok("💚") });

    tamako.at("/api/whisper").get(api::list);
    tamako.at("/api/whisper/:snowflake").get(api::get);
    tamako
        .at("/api/whisper")
        .with(tide_governor::GovernorMiddleware::per_minute(2)?)
        .post(api::add);
    tamako.at("/api/whisper/:snowflake").delete(api::delete);

    tamako.at("/api/auth").post(api::auth);

    tamako.at("*").get(templates::not_found);
    let addr = (api::HOST.as_str(), *api::PORT);
    tamako.listen(addr).await?;

    Ok(())
}
