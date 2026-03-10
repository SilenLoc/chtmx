use actix_web::{HttpRequest, HttpResponse, Responder, get};

const TCSS: &str = include_str!("../assets/t.css");
const HTMX: &str = include_str!("../assets/h.js");
const BACKGROUND: &[u8] = include_bytes!("../assets/background.jpg");

#[get("/assets/{filename:.*}")]
pub async fn assets(req: HttpRequest) -> impl Responder {
    let path = req.match_info().query("filename");

    match path {
        "t.css" => HttpResponse::Ok()
            .content_type("text/css; charset=utf-8")
            .body(TCSS),
        "h.js" => HttpResponse::Ok()
            .content_type("application/javascript; charset=utf-8")
            .body(HTMX),
        "background.jpg" => HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(BACKGROUND),
        _ => HttpResponse::NotFound().body("Not found"),
    }
}
