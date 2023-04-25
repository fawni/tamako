mod api;
mod db;
mod template;

#[api::main]
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

    let addr = (api::host().to_owned(), api::port().to_owned());
    tamako.listen(addr).await?;

    Ok(())
}
