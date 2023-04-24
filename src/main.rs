mod api;
mod db;
mod snowflake;
mod template;

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();
    dotenvy::dotenv().ok();

    let mut tamako = tide::new();

    tamako.at("/").get(template::render);
    tamako.at("/api").nest({
        let database = db::open().await?;
        let mut api = tide::with_state(database);

        api.at("/whisper").get(api::list);
        api.at("/whisper")
            .with(tide_governor::GovernorMiddleware::per_minute(2)?)
            .post(api::add);

        api
    });

    let host = api::host();
    let port = api::port();
    tamako.listen((host.to_owned(), port.to_owned())).await?;

    Ok(())
}
