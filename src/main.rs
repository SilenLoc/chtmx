use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use env_logger::Env;
use log::info;

mod assets;
mod config;
mod index;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::from_env();

    env_logger::Builder::from_env(Env::default().default_filter_or(config.log_level())).init();

    info!("{config}");

    let bind_address = config.address();

    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(assets::assets)
            .route("/", web::get().to(index::index))
    })
    .bind(bind_address)?
    .run()
    .await
}
