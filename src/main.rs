use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use env_logger::Env;
use log::info;

mod assets;
mod config;
mod db;
mod upload;
mod view;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/health/db")]
async fn health_db(ch: web::Data<db::Ch>) -> impl Responder {
    match ch.query("SELECT 1").fetch_one::<u8>().await {
        Ok(_) => HttpResponse::Ok().body("Database connection OK"),
        Err(e) => {
            log::error!("Database health check failed: {}", e);
            HttpResponse::ServiceUnavailable().body(format!("Database connection failed: {}", e))
        }
    }
}

#[get("/health/db/status")]
async fn health_db_status(ch: web::Data<db::Ch>) -> impl Responder {
    use maud::html;

    let (status_class, status_text, icon) = match ch.query("SELECT 1").fetch_one::<u8>().await {
        Ok(_) => ("bg-dark-green white", "Healthy", "●"),
        Err(e) => {
            log::error!("Database health check failed: {}", e);
            ("bg-dark-red white", "Unhealthy", "●")
        }
    };

    let markup = html! {
        div id="db-status"
            class=(format!("flex items-center pa2 br2 dib {}", status_class))
            hx-get="/health/db/status"
            hx-trigger="every 10s"
            hx-swap="outerHTML" {
            span class="f6 mr2" { (icon) }
            span class="f6 b" { "Database: " (status_text) }
        }
    };

    maud::PreEscaped(markup.into_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::from_env();

    env_logger::Builder::from_env(Env::default().default_filter_or(config.log_level())).init();

    info!("{config}");

    let clickhouse = db::connect(&config);
    let clickhouse_data = web::Data::new(clickhouse);

    let bind_address = config.address();

    HttpServer::new(move || {
        App::new()
            .app_data(clickhouse_data.clone())
            .service(health)
            .service(health_db)
            .service(health_db_status)
            .service(assets::assets)
            .service(view::index)
            .service(view::home::home_page)
            .service(view::about::about_page)
            .service(view::how_it_works::how_it_works_page)
            .service(view::upload::upload_page)
            .service(view::databases::databases_page)
            .service(view::databases::get_tables)
            .service(upload::upload_csv)
    })
    .bind(bind_address)?
    .run()
    .await
}
