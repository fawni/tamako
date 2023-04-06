mod api;
mod db;
mod snowflake;

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut tamako = tide::new();

    tamako.at("/").get(|_| async { Ok("🐞") });
    tamako.at("/api/whisper").nest({
        let database = db::open().await?;
        let mut api = tide::with_state(database);

        api.at("/").get(api::list);
        api.at("/").post(api::add);

        api
    });

    let port = dotenvy_macro::dotenv!("PORT").parse::<u16>()?;
    tamako.listen(("127.0.0.1".to_owned(), port)).await?;

    Ok(())
}
