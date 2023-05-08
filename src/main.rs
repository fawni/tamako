mod api;
mod auth;
mod db;
mod template;

#[api::main]
async fn main() -> tide::Result<()> {
    femme::start();
    dotenvy::dotenv().ok();

    let database = db::open().await?;
    let mut tamako = tide::with_state(database.clone());

    tamako.at("/").get(template::tamako);
    tamako.at("/auth").get(template::auth);

    tamako.at("/api").nest({
        let mut api = tide::with_state(database);

        api.at("/whisper").get(api::list);
        api.at("/whisper")
            .with(tide_governor::GovernorMiddleware::per_minute(2)?)
            .post(api::add);
        api.at("/whisper/:snowflake").delete(api::delete);

        api.at("/auth").post(api::auth);

        api
    });

    tamako.at("/assets").serve_dir("assets")?;

    let addr = (api::HOST.to_owned(), api::PORT.to_owned());
    tamako.listen(addr).await?;

    Ok(())
}
