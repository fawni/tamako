use actix_web::{middleware, web, App, HttpServer};

mod api;
mod auth;
mod db;
mod templates;

#[tokio::main]
async fn main() -> actix_web::Result<()> {
    femme::start();
    dotenvy::dotenv().ok();

    let database = db::open().await?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(templates::home)
            .service(templates::auth)
            .service(actix_files::Files::new("/assets", "assets"))
            .service(web::resource("/api/health").route(web::get().to(|| async { "💚" })))
            .service(api::list)
            .service(api::get)
            .service(
                web::resource("/api/whisper")
                    .route(web::post().to(api::add))
                    .wrap(actix_governor::Governor::new(
                        &actix_governor::GovernorConfigBuilder::default()
                            .per_second(360)
                            .burst_size(1)
                            .finish()
                            .unwrap_or_default(),
                    )),
            )
            .service(api::delete)
            .service(api::authentication)
            .default_service(web::to(templates::not_found))
    })
    .bind((api::HOST.as_str(), *api::PORT))?
    .run()
    .await?;

    Ok(())
}
