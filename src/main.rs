use actix_files::Files;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    http::StatusCode,
    middleware::{Compress, ErrorHandlers, NormalizePath, TrailingSlash},
    web, App, HttpServer,
};

mod api;
mod auth;
mod db;
mod templates;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    twink::log::setup();
    dotenvy::dotenv().ok();

    let database = db::open().await?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .wrap(actix_logger::Logger::new(twink::fmt!(
                "<green>%s <purple>%r</> took <cyan>%Dms</> | %{X-Forwarded-For}i <i>%{User-Agent}i</>"
            )))
            .wrap(Compress::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, templates::not_found))
            .service(templates::robots)
            .service(templates::home)
            .service(templates::auth)
            .service(Files::new("/assets", "assets"))
            .service(web::resource("/api/health").route(web::get().to(|| async { "💚" })))
            .service(api::list)
            .service(api::get)
            .service(
                web::resource("/api/whisper")
                    .route(web::post().to(api::add))
                    .wrap(Governor::new(
                        &GovernorConfigBuilder::default()
                            .requests_per_second(360)
                            .burst_size(2)
                            .finish()
                            .unwrap_or_default(),
                    )),
            )
            .service(api::delete)
            .service(api::authentication)
    })
    .bind((api::HOST.as_str(), *api::PORT))?
    .run()
    .await?;

    Ok(())
}
