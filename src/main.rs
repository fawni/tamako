use color_eyre::Result;
use std::env;
use tamako::{constants, db, filters};
use warp::Filter;

fn init() -> Result<()> {
    dotenvy::dotenv().ok();
    // kankyo::init();
    color_eyre::install()?;
    env::set_var("RUST_LOG", &*constants::RUST_LOG);
    pretty_env_logger::init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init()?;

    let collection = db::get_collection(db::connect_to_db().await?);

    // let home = warp::path::end().map(|| "🍙");
    let home = warp::path::end().and(warp::fs::dir("frontend/dist"));
    let assets = warp::path::path("assets").and(warp::fs::dir("frontend/dist/assets"));
    let api = filters::whispers(collection);

    let routes = home.or(assets).or(api).with(warp::log("tamako"));
    warp::serve(routes)
        .run(([0, 0, 0, 0], *constants::PORT))
        .await;

    Ok(())
}
