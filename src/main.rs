use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::init_db().await;

    HttpServer::new(move || {
        App::new()
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
