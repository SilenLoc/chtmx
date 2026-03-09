use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    log::info!("Starting server on http://0.0.0.0:{}", port);

    HttpServer::new(|| App::new().service(health))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
