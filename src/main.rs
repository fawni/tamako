use std::env;
use tamako::{db, filters, Result};
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env::set_var("RUST_LOG", env::var("RUST_LOG")?);
    pretty_env_logger::init();

    let port: u16 = env::var("PORT")?.parse::<u16>()?;
    let collection = db::get_collection(db::connect_to_db().await?);

    // let home = warp::path::end().map(|| "🍙");
    let home = warp::path::end().and(warp::fs::dir("frontend/dist"));
    let assets = warp::path::path("assets").and(warp::fs::dir("frontend/dist/assets"));
    let api = filters::whispers(collection);

    let routes = home.or(assets).or(api).with(warp::log("tamako"));
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}
