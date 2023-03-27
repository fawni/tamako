mod api;
mod db;

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();
    dotenvy::dotenv().ok();

    let mut tamako = tide::new();

    tamako.at("/").get(|_| async { Ok("🐞") });
    tamako.at("/api/whisper").nest({
        let database = db::open().await?;
        let mut api = tide::with_state(database);

        api.at("/").get(api::list);
        api.at("/").post(api::add);

        api
    });

    let host = String::from("127.0.0.1");
    let port = std::env::var("PORT")?.parse::<u16>()?;
    tamako.listen((host, port)).await?;

    Ok(())
}
