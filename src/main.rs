use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::FmtSubscriber;

mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::init_db().await;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Fail to config logger");

    info!("Starting API...");

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route(
                "/auction/properties",
                web::get().to(routes::property::get_properties),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
