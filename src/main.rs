mod api;
mod db;
mod snowflake;
mod template;

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut tamako = tide::new();

    tamako.at("/").get(template::render);
    tamako.at("/api").nest({
        let database = db::open().await?;
        let mut api = tide::with_state(database);

        api.at("/whisper").get(api::list);
        api.at("/whisper").post(api::add);

        api
    });

    let host = dotenvy_macro::dotenv!("HOST");
    let port = dotenvy_macro::dotenv!("PORT").parse::<u16>()?;
    tamako.listen((host, port)).await?;

    Ok(())
}
